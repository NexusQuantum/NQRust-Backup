//! End-to-end backup + restore round-trip test.
//!
//! Runs the bundled `backup-bareos-fd` job, restores it into a temp dir, and
//! SHA-256 spot-checks N restored files against their originals. Designed as
//! a smoke test that proves the full DIR + SD + FD + Catalog chain works.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};

use crate::app::{LogLevel, LogRing};

/// Outcome of a single round-trip run.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RoundtripReport {
    pub backup_jobid: u32,
    pub restore_jobid: u32,
    pub files_backed_up: u64,
    pub bytes_backed_up: u64,
    pub files_restored: u64,
    pub bytes_restored: u64,
    pub samples_checked: usize,
    pub samples_matched: usize,
    pub backup_secs: u64,
    pub restore_secs: u64,
    pub restore_dir: PathBuf,
}

impl RoundtripReport {
    pub fn passed(&self) -> bool {
        self.files_backed_up == self.files_restored
            && self.bytes_backed_up == self.bytes_restored
            && self.samples_matched == self.samples_checked
            && self.samples_checked > 0
    }
}

pub struct RoundtripOptions {
    pub job_name: String,
    pub volume_pool: String,
    pub volume_name: String,
    pub restore_dir: PathBuf,
    pub samples: usize,
    pub keep_restore_dir: bool,
}

impl Default for RoundtripOptions {
    fn default() -> Self {
        let ts = chrono::Local::now().format("%Y%m%d-%H%M%S");
        Self {
            job_name: "backup-bareos-fd".to_string(),
            volume_pool: "Full".to_string(),
            volume_name: format!("nqrb-roundtrip-{ts}"),
            restore_dir: PathBuf::from(format!("/tmp/nqrb-roundtrip-{ts}")),
            samples: 10,
            keep_restore_dir: false,
        }
    }
}

pub fn run(opts: &RoundtripOptions, log: &LogRing) -> Result<RoundtripReport> {
    log.push(LogLevel::Info, "==> NQRustBackup round-trip test");
    log.push(
        LogLevel::Info,
        format!(
            "    job={}  pool={}  volume={}",
            opts.job_name, opts.volume_pool, opts.volume_name
        ),
    );
    log.push(
        LogLevel::Info,
        format!("    restore -> {}", opts.restore_dir.display()),
    );

    // 1. Label a fresh volume (idempotent: if it already exists we just continue)
    log.push(LogLevel::Info, "labelling volume…");
    let _ = bconsole(&format!(
        "label storage=File volume={} pool={}\nquit\n",
        opts.volume_name, opts.volume_pool
    ));

    // 2. Run backup
    log.push(LogLevel::Info, "running backup job…");
    let backup_t0 = std::time::Instant::now();
    let backup_out = bconsole(&format!(
        "run job={} yes\nwait\nlist jobs\nquit\n",
        opts.job_name
    ))?;
    let backup_secs = backup_t0.elapsed().as_secs();

    let backup_jobid = parse_jobid(&backup_out)
        .ok_or_else(|| anyhow!("could not find JobId in bconsole output"))?;
    log.push(LogLevel::Info, format!("backup jobid={backup_jobid}"));

    let (b_files, b_bytes, b_status) = job_stats(backup_jobid)?;
    if b_status != "T" {
        bail!(
            "backup job {} terminated with status '{}' (expected 'T'). See `journalctl -u bareos-director`.",
            backup_jobid, b_status
        );
    }
    log.push(
        LogLevel::Ok,
        format!("backup OK: {b_files} files / {b_bytes} bytes / {backup_secs}s"),
    );

    // 3. Prepare restore dir
    std::fs::create_dir_all(&opts.restore_dir)
        .with_context(|| format!("creating {}", opts.restore_dir.display()))?;

    // 4. Run restore (non-interactive form)
    log.push(LogLevel::Info, "running restore…");
    let restore_t0 = std::time::Instant::now();
    let restore_out = bconsole(&format!(
        "restore client=bareos-fd jobid={} where={} all done\nyes\nwait\nlist jobs\nquit\n",
        backup_jobid,
        opts.restore_dir.display()
    ))?;
    let restore_secs = restore_t0.elapsed().as_secs();
    let restore_jobid = parse_jobid(&restore_out)
        .ok_or_else(|| anyhow!("could not find restore JobId in bconsole output"))?;
    log.push(LogLevel::Info, format!("restore jobid={restore_jobid}"));

    let (r_files, r_bytes, r_status) = job_stats(restore_jobid)?;
    if r_status != "T" {
        bail!(
            "restore job {} terminated with status '{}' (expected 'T').",
            restore_jobid,
            r_status
        );
    }
    log.push(
        LogLevel::Ok,
        format!("restore OK: {r_files} files / {r_bytes} bytes / {restore_secs}s"),
    );

    // 5. SHA-256 spot check
    let (samples_checked, samples_matched) = sha_sample(&opts.restore_dir, opts.samples, log)?;

    let report = RoundtripReport {
        backup_jobid,
        restore_jobid,
        files_backed_up: b_files,
        bytes_backed_up: b_bytes,
        files_restored: r_files,
        bytes_restored: r_bytes,
        samples_checked,
        samples_matched,
        backup_secs,
        restore_secs,
        restore_dir: opts.restore_dir.clone(),
    };

    if !opts.keep_restore_dir {
        if let Err(e) = std::fs::remove_dir_all(&opts.restore_dir) {
            log.push(
                LogLevel::Warn,
                format!(
                    "could not remove {}: {e} (left in place)",
                    opts.restore_dir.display()
                ),
            );
        }
    }

    Ok(report)
}

fn bconsole(input: &str) -> Result<String> {
    let mut child = Command::new("bconsole")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("spawning bconsole — is it installed and on PATH?")?;
    {
        use std::io::Write;
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input.as_bytes())?;
    }
    let out = child.wait_with_output()?;
    let s = String::from_utf8_lossy(&out.stdout).to_string();
    Ok(s)
}

fn parse_jobid(out: &str) -> Option<u32> {
    // Look for "Job queued. JobId=N"
    for line in out.lines() {
        if let Some(rest) = line.split_once("JobId=") {
            let num: String = rest.1.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = num.parse() {
                return Some(n);
            }
        }
    }
    None
}

fn job_stats(jobid: u32) -> Result<(u64, u64, String)> {
    // Query catalog directly via bconsole's list jobs jobid=N
    let out = bconsole(&format!("list jobs jobid={jobid}\nquit\n"))?;
    // Parse the table row
    for line in out.lines() {
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        // Header row contains "jobid"; skip
        if cells.len() < 11 || cells[1] == "jobid" || cells[1].is_empty() {
            continue;
        }
        if let Ok(j) = cells[1].parse::<u32>() {
            if j == jobid {
                let files = cells[8].replace(',', "").parse::<u64>().unwrap_or(0);
                let bytes = cells[9].replace(',', "").parse::<u64>().unwrap_or(0);
                let status = cells[10].to_string();
                return Ok((files, bytes, status));
            }
        }
    }
    bail!("job {jobid} not found in catalog (yet?)");
}

/// Pick up to `n` files from the restored tree and SHA-256-compare them
/// against the originals (the path the restore tree mirrors).
///
/// Returns (checked, matched).
fn sha_sample(restore_dir: &Path, n: usize, log: &LogRing) -> Result<(usize, usize)> {
    let mut sampled: Vec<PathBuf> = Vec::new();
    walk_files(restore_dir, &mut sampled, n.saturating_mul(4))?;
    sampled.truncate(n);

    let mut checked = 0usize;
    let mut matched = 0usize;
    for restored in &sampled {
        let rel = restored
            .strip_prefix(restore_dir)
            .unwrap_or(restored)
            .to_path_buf();
        let original = PathBuf::from("/").join(&rel);
        if !original.is_file() {
            continue; // file was removed/renamed since backup — skip
        }
        let a = sha256(restored)?;
        let b = sha256(&original)?;
        checked += 1;
        if a == b {
            matched += 1;
        } else {
            log.push(
                LogLevel::Warn,
                format!(
                    "sha mismatch: restored={} original={}",
                    restored.display(),
                    original.display()
                ),
            );
        }
    }
    log.push(
        LogLevel::Ok,
        format!("sha-256 spot check: {matched}/{checked} matched"),
    );
    Ok((checked, matched))
}

fn walk_files(dir: &Path, out: &mut Vec<PathBuf>, max: usize) -> Result<()> {
    if out.len() >= max {
        return Ok(());
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if out.len() >= max {
                return Ok(());
            }
            let p = entry.path();
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() && meta.len() > 0 && meta.len() < 50_000_000 {
                    out.push(p);
                } else if meta.is_dir() {
                    let _ = walk_files(&p, out, max);
                }
            }
        }
    }
    Ok(())
}

fn sha256(p: &Path) -> Result<String> {
    let out = Command::new("sha256sum")
        .arg(p)
        .output()
        .with_context(|| format!("sha256sum {}", p.display()))?;
    if !out.status.success() {
        bail!(
            "sha256sum {} failed: {}",
            p.display(),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let s = String::from_utf8_lossy(&out.stdout);
    Ok(s.split_whitespace().next().unwrap_or("").to_string())
}
