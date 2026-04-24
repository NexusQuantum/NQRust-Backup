async (page) => {
  await page.goto('http://127.0.0.1:9104/dashboard/', { waitUntil: 'networkidle', timeout: 30000 });
  return await page.evaluate(() => {
    const nav = document.querySelector('.navbar-collapse');
    const toggle = document.querySelector('.navbar-toggle');
    return {
      width: window.innerWidth,
      navClass: nav && nav.className,
      navDisplay: nav && getComputedStyle(nav).display,
      navHeight: nav && getComputedStyle(nav).height,
      toggleDisplay: toggle && getComputedStyle(toggle).display,
      navText: nav && nav.innerText
    };
  });
}