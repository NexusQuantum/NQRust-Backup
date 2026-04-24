async (page) => {
  await page.goto('http://127.0.0.1:9104/dashboard/', { waitUntil: 'networkidle', timeout: 30000 });
  await page.screenshot({ path: 'output/playwright/nqrust-dashboard.png', fullPage: true, scale: 'css' });
  return { title: await page.title(), screenshot: 'output/playwright/nqrust-dashboard.png' };
}