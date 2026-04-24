async (page) => {
  const base = 'http://127.0.0.1:9104';
  const paths = [
    '/dashboard/',
    '/job/',
    '/client/',
    '/storage/',
    '/pool/',
    '/media/',
    '/fileset/',
    '/restore/',
    '/schedule/',
    '/console/',
    '/analytics/',
    '/director/',
    '/director/messages',
    '/director/subscription'
  ];
  const results = [];
  for (const path of paths) {
    const response = await page.goto(base + path, { waitUntil: 'networkidle', timeout: 30000 });
    const text = await page.locator('body').innerText({ timeout: 10000 }).catch(() => '');
    const matches = text.match(/NQRustBackup|nqrustbackup|NQRUSTBACKUP|nqrustbackup\.org|nqrustbackup\.com|download\.nqrustbackup\.com/g) || [];
    results.push({ path, status: response ? response.status() : null, title: await page.title(), nqrustbackupMatches: Array.from(new Set(matches)).slice(0, 5) });
  }
  return results;
}