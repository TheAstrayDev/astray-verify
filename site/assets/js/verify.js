(() => {
  const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (!reducedMotion) {
    document.documentElement.classList.add('has-verify-motion');
  }

  const buttons = [...document.querySelectorAll('[data-os]')];
  const panels = [...document.querySelectorAll('[data-panel]')];

  function setOs(os) {
    buttons.forEach((button) => {
      const active = button.dataset.os === os;
      button.classList.toggle('is-active', active);
      button.setAttribute('aria-selected', String(active));
      button.tabIndex = active ? 0 : -1;
    });
    panels.forEach((panel) => {
      const active = panel.dataset.panel === os;
      panel.classList.toggle('is-active', active);
      panel.hidden = !active;
    });
  }

  function detectOs() {
    const userAgent = navigator.userAgent || '';
    if (/Windows/i.test(userAgent)) return 'windows';
    if (/Mac/i.test(userAgent)) return 'macos';
    return 'linux';
  }

  buttons.forEach((button, index) => {
    button.addEventListener('click', () => setOs(button.dataset.os));
    button.addEventListener('keydown', (event) => {
      if (!['ArrowLeft', 'ArrowRight'].includes(event.key)) return;
      event.preventDefault();
      const next = (index + (event.key === 'ArrowRight' ? 1 : buttons.length - 1)) % buttons.length;
      buttons[next].focus();
      setOs(buttons[next].dataset.os);
    });
  });
  setOs(detectOs());

  document.querySelectorAll('[data-copy-button]').forEach((button) => {
    button.addEventListener('click', async () => {
      const command = button.closest('[data-copy]')?.querySelector('code')?.textContent?.trim();
      if (!command) return;
      try {
        await navigator.clipboard.writeText(command);
      } catch {
        const field = document.createElement('textarea');
        field.value = command;
        field.style.position = 'fixed';
        field.style.opacity = '0';
        document.body.append(field);
        field.select();
        document.execCommand('copy');
        field.remove();
      }
      const original = button.textContent;
      button.textContent = 'Copied';
      button.setAttribute('aria-label', 'Command copied');
      window.setTimeout(() => { button.textContent = original; }, 1400);
      window.setTimeout(() => { button.removeAttribute('aria-label'); }, 1400);
    });
  });

  if (!reducedMotion && 'IntersectionObserver' in window) {
    const observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (!entry.isIntersecting) return;
        entry.target.classList.add('is-visible');
        observer.unobserve(entry.target);
      });
    }, { threshold: 0.16 });
    document.querySelectorAll('.verify-reveal').forEach((section) => observer.observe(section));
  } else {
    document.querySelectorAll('.verify-reveal').forEach((section) => section.classList.add('is-visible'));
  }
})();
