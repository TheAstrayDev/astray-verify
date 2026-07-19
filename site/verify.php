<?php
declare(strict_types=1);

require __DIR__ . '/includes/bootstrap.php';
require __DIR__ . '/includes/data.php';

$pageTitle = 'Astray Verify — MCP regression tests';
$lockSplash = false;
$isHome = false;
$extraStyles = ['/assets/css/verify.css?v=1'];
$seo = [
    'title' => $pageTitle,
    'description' => 'Record an MCP server contract once. Astray Verify catches breaking tool and schema changes before AI clients do.',
    'keywords' => 'MCP testing, Model Context Protocol, MCP regression tests, MCP CI, AI tools, Astray Verify',
    'path' => '/verify',
    'type' => 'website',
    'jsonld' => [
        seo_jsonld_website(),
        seo_jsonld_person(),
        seo_jsonld_breadcrumb([
            ['name' => 'The Astray', 'url' => abs_url('/')],
            ['name' => 'Astray Verify', 'url' => abs_url('/verify')],
        ]),
    ],
];

require __DIR__ . '/includes/header.php';

$unixInstall = 'curl -fsSL https://theastraydev.online/verify-install.sh | sh';
$windowsInstall = 'curl.exe -fsSL https://theastraydev.online/verify-install.ps1 | powershell -NoProfile -ExecutionPolicy Bypass -';
?>

<main class="verify" id="main">
  <nav class="verify-nav" aria-label="Primary navigation">
    <a class="verify-nav__brand" href="/" aria-label="The Astray home">THE ASTRAY <span>/</span> VERIFY</a>
    <div class="verify-nav__links">
      <a href="#how">How it works</a>
      <a href="#install">Install</a>
      <a href="https://github.com/TheAstrayDev/astray-verify" target="_blank" rel="noopener noreferrer">GitHub ↗</a>
    </div>
  </nav>

  <section class="verify-hero" aria-labelledby="verify-title">
    <div class="verify-hero__copy">
      <p class="verify-kicker"><span class="verify-status" aria-hidden="true"></span> MCP CONTRACT TESTING · OPEN SOURCE</p>
      <h1 id="verify-title">Your MCP server still starts.<br><em>Did it still work?</em></h1>
      <p class="verify-hero__lead">Astray Verify records the tools your AI clients rely on, then catches broken names, schemas, and protocol output before you release.</p>
      <div class="verify-actions">
        <a class="verify-button verify-button--solid" href="#install">Install Astray Verify <span aria-hidden="true">↓</span></a>
        <a class="verify-button verify-button--line" href="https://github.com/TheAstrayDev/astray-verify" target="_blank" rel="noopener noreferrer">Read the source <span aria-hidden="true">↗</span></a>
      </div>
      <p class="verify-hero__note">A small CI check for authors of MCP servers. No model, account, or cloud required.</p>
    </div>

    <div class="verify-proof" aria-label="Example Astray Verify contract check">
      <div class="verify-proof__top"><span>ASTRAY VERIFY / FIXTURE</span><span>stdio</span></div>
      <div class="verify-proof__body">
        <p class="verify-proof__label">recorded interface</p>
        <div class="verify-tool is-good"><span class="verify-tool__mark">✓</span><span class="verify-tool__name">search_issues</span><span class="verify-tool__type">object</span></div>
        <div class="verify-tool is-good"><span class="verify-tool__mark">✓</span><span class="verify-tool__name">create_issue</span><span class="verify-tool__type">object</span></div>
        <div class="verify-tool is-change"><span class="verify-tool__mark">!</span><span class="verify-tool__name">list_repos</span><span class="verify-tool__type">changed</span></div>
        <div class="verify-proof__result"><span>CONTRACT</span><strong>FAIL</strong><span>1 unexpected change</span></div>
      </div>
      <div class="verify-proof__tape" aria-hidden="true">TOOLS/LIST · SNAPSHOT · REVIEW · REPLAY · TOOLS/LIST · SNAPSHOT · REVIEW · REPLAY ·</div>
    </div>
  </section>

  <section class="verify-problem" aria-labelledby="problem-title">
    <div class="verify-section-label">THE PROBLEM</div>
    <div>
      <h2 id="problem-title">A green process is not a stable integration.</h2>
      <p>Renaming one tool, changing one input field, or printing a debug line to stdout can break an AI client after deployment. The server may look healthy; the contract your client depends on is not.</p>
    </div>
    <div class="verify-problem__facts" aria-label="What Astray Verify catches">
      <p><b>01</b> Missing or renamed tools</p>
      <p><b>02</b> Changed JSON input schemas</p>
      <p><b>03</b> Invalid JSON-RPC output</p>
    </div>
  </section>

  <section class="verify-flow" id="how" aria-labelledby="flow-title">
    <div class="verify-flow__head">
      <div class="verify-section-label">THREE STEPS</div>
      <h2 id="flow-title">Record once.<br>Protect every release.</h2>
    </div>
    <ol class="verify-steps">
      <li>
        <span class="verify-step__number">01</span>
        <h3>Record</h3>
        <p>Start your MCP server once. Astray Verify performs the handshake and saves its tools and schemas as a small JSON fixture.</p>
        <code>astray-verify record --name github -- &lt;server&gt;</code>
      </li>
      <li>
        <span class="verify-step__number">02</span>
        <h3>Commit</h3>
        <p>Review the fixture with your code. It becomes the explicit promise your server makes to every AI client.</p>
        <code>fixtures/github.mcp.json</code>
      </li>
      <li>
        <span class="verify-step__number">03</span>
        <h3>Replay</h3>
        <p>Run one command locally or in CI. A change fails loudly before it becomes somebody else's broken agent workflow.</p>
        <code>astray-verify test</code>
      </li>
    </ol>
  </section>

  <section class="verify-now" aria-labelledby="now-title">
    <div>
      <div class="verify-section-label">FIRST RELEASE</div>
      <h2 id="now-title">Small on purpose.</h2>
      <p>Start with the part every MCP client sees: a clean stdio handshake and <code>tools/list</code>. No generated dashboards. No cloud account. Just a contract test you can trust.</p>
    </div>
    <ul class="verify-checklist">
      <li><span>✓</span> Stdio transport</li>
      <li><span>✓</span> Initialize handshake</li>
      <li><span>✓</span> Tool schema snapshot</li>
      <li><span>✓</span> Strict stdout validation</li>
    </ul>
  </section>

  <section class="verify-install" id="install" aria-labelledby="install-title">
    <div class="verify-install__head">
      <div class="verify-section-label">GET STARTED</div>
      <h2 id="install-title">One command. Then test the change you were about to ship.</h2>
    </div>
    <div class="verify-os" role="tablist" aria-label="Choose your operating system">
      <button class="is-active" role="tab" aria-selected="true" aria-controls="verify-linux" id="verify-linux-tab" data-os="linux">Linux</button>
      <button role="tab" aria-selected="false" aria-controls="verify-macos" id="verify-macos-tab" data-os="macos">macOS</button>
      <button role="tab" aria-selected="false" aria-controls="verify-windows" id="verify-windows-tab" data-os="windows">Windows</button>
    </div>
    <div class="verify-install__panel is-active" role="tabpanel" id="verify-linux" aria-labelledby="verify-linux-tab" data-panel="linux">
      <p>Downloads a release binary when available; otherwise builds the current release with Rust.</p>
      <div class="verify-command" data-copy>
        <code><?= e($unixInstall) ?></code>
        <button type="button" data-copy-button>Copy</button>
      </div>
    </div>
    <div class="verify-install__panel" role="tabpanel" id="verify-macos" aria-labelledby="verify-macos-tab" data-panel="macos" hidden>
      <p>Works on Apple Silicon and Intel Macs. A source build is used while a matching release binary is unavailable.</p>
      <div class="verify-command" data-copy>
        <code><?= e($unixInstall) ?></code>
        <button type="button" data-copy-button>Copy</button>
      </div>
    </div>
    <div class="verify-install__panel" role="tabpanel" id="verify-windows" aria-labelledby="verify-windows-tab" data-panel="windows" hidden>
      <p>Run from PowerShell. It downloads a Windows release binary or builds from source with Rust.</p>
      <div class="verify-command" data-copy>
        <code><?= e($windowsInstall) ?></code>
        <button type="button" data-copy-button>Copy</button>
      </div>
    </div>
    <p class="verify-install__foot">Prefer to read every line first? <a href="https://github.com/TheAstrayDev/astray-verify" target="_blank" rel="noopener noreferrer">Open the source on GitHub ↗</a></p>
  </section>

  <section class="verify-faq" aria-labelledby="faq-title">
    <div class="verify-section-label">FAQ</div>
    <div>
      <h2 id="faq-title">The short answers.</h2>
      <details open><summary>Who is this for?</summary><p>Anyone building or maintaining an MCP server. If an AI client calls your tools, their public interface is worth testing.</p></details>
      <details><summary>Does it call an AI model?</summary><p>No. Astray Verify talks to your MCP server directly. It checks the protocol contract, so it does not require an API key or model account.</p></details>
      <details><summary>Does it replace the MCP Inspector?</summary><p>No. Use Inspector to explore and debug. Use Astray Verify to commit a known-good interface and replay it after every change.</p></details>
    </div>
  </section>
</main>

<script src="/assets/js/verify.js?v=1" defer></script>
<?php require __DIR__ . '/includes/footer.php'; ?>
