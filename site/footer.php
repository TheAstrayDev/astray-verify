  <section class="cli-strip" aria-label="Astray Verify">
    <div class="cli-strip__inner">
      <div class="cli-strip__copy">
        <span class="cli-strip__prompt" aria-hidden="true">✓</span>
        <div class="cli-strip__text">
          <span class="cli-strip__name">Astray Verify</span>
          <span class="cli-strip__tag">MCP contract tests · record once, test every release</span>
        </div>
      </div>
      <a class="cli-strip__cta" href="/verify">Open Astray Verify →</a>
    </div>
  </section>

  <footer class="footer" role="contentinfo">
    <div>
      © <?= (int) ($site['year'] ?? date('Y')) ?>
      <span itemprop="name"><?= e($site['name'] ?? 'The Astray') ?></span>
      · <?= e(__('footer.rights')) ?>
    </div>
    <div class="footer__links">
      <a href="mailto:<?= e($site['email'] ?? 'hello@theastraydev.online') ?>"><?= e($site['email'] ?? 'hello@theastraydev.online') ?></a>
      <span aria-hidden="true"> · </span>
      <a href="https://t.me/Jkkaall" target="_blank" rel="noopener noreferrer me">Telegram</a>
      <span aria-hidden="true"> · </span>
      <a href="/sitemap.xml">Sitemap</a>
    </div>
  </footer>
  <script src="/assets/js/tracker.js?v=1" defer></script>
  <script src="/assets/js/main.js?v=seo3" defer></script>
</body>
</html>
