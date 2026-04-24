//! Generate the bareos / NQRustBackup config fragments.
//!
//! We write to `{config_dir}/bareos-dir.d/`, `.../bareos-sd.d/`, `.../bareos-fd.d/`.
//! Package install will have dropped default fragments too; we add a
//! "nqrb-installer/" subdir stash and a few override files.

use anyhow::Result;
use std::path::Path;

use crate::app::{InstallConfig, LogLevel, LogRing};
use crate::installer::executor::sudo_run_logged;

pub async fn render_and_install(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    log.push(LogLevel::Info, "writing director/SD/FD config fragments");

    let cd = cfg.config_dir.to_string_lossy().to_string();

    // FileSet — matches our proven P2V eval fileset.
    let fileset = r#"FileSet {
  Name = "FullRoot"
  Description = "Full root filesystem (exclude volatile/special paths)"
  Include {
    Options { signature = MD5; compression = LZ4; onefs = no; sparse = yes; noatime = yes; }
    File = /
  }
  Exclude {
    File = /proc
    File = /sys
    File = /dev
    File = /run
    File = /tmp
    File = /mnt
    File = /media
    File = /var/cache
    File = /var/tmp
    File = /swap.img
    File = /lost+found
    File = /var/lib/bareos/storage
    File = /var/spool/bareos
  }
}
"#;

    let pool = r#"Pool {
  Name = EvalPool
  Pool Type = Backup
  Recycle = yes
  AutoPrune = yes
  Volume Retention = 30 days
  Maximum Volume Bytes = 20G
  Maximum Volumes = 10
  Label Format = "eval-vol-"
}
"#;

    // Replace default "File" Storage's Address to avoid the `nqsrv:9103` DNS gotcha
    // hit in the P2V eval — for single-host installs localhost works fine.
    let storage_override = r#"Storage {
  Name = File
  Address = 127.0.0.1
  Password = ""   # leave blank; package-installed Storage/File.conf carries the real secret
  Device = FileStorage
  Media Type = File
  Maximum Concurrent Jobs = 10
}
"#;

    install_file(
        log,
        &format!("{cd}/bareos-dir.d/fileset/FullRoot.conf"),
        fileset,
        cfg.dry_run,
    )
    .await?;
    install_file(
        log,
        &format!("{cd}/bareos-dir.d/pool/EvalPool.conf"),
        pool,
        cfg.dry_run,
    )
    .await?;
    // NOTE: do not overwrite the package-provided Storage/File.conf (it has a
    // unique password). Instead leave an override recipe for the operator.
    install_file(
        log,
        &format!("{cd}/nqrb-installer/Storage-File.override.example"),
        storage_override,
        cfg.dry_run,
    )
    .await?;

    // Ensure storage dir exists with right ownership
    let mkdir = format!(
        "mkdir -p {} && chown bareos:bareos {} 2>/dev/null || true",
        cfg.storage_dir.display(),
        cfg.storage_dir.display()
    );
    sudo_run_logged(&["sh", "-c", &mkdir], log, cfg.dry_run).await?;

    Ok(())
}

async fn install_file(log: &LogRing, dest: &str, content: &str, dry_run: bool) -> Result<()> {
    let dir = Path::new(dest).parent().unwrap().to_string_lossy().to_string();
    let script = format!(
        r#"set -eu
mkdir -p '{dir}'
cat > '{dest}' <<'__EOF__'
{content}__EOF__
chown root:bareos '{dest}' 2>/dev/null || true
chmod 640 '{dest}'
"#
    );
    log.push(LogLevel::Info, format!("install {dest}"));
    sudo_run_logged(&["sh", "-c", &script], log, dry_run).await?;
    Ok(())
}
