<?php
declare(strict_types=1);

require __DIR__ . '/includes/bootstrap.php';
require __DIR__ . '/includes/data.php';

$pageTitle = 'Astray Verify — MCP contract testing for CI';
$seo = [
    'title' => $pageTitle,
    'description' => 'Open-source MCP contract testing: record tools and JSON schemas once, then catch breaking changes in CI before AI clients fail.',
];

$unixInstall = 'curl -fsSL https://raw.githubusercontent.com/TheAstrayDev/astray-verify/main/install.sh | sh';
$windowsInstall = 'curl.exe -fsSL https://raw.githubusercontent.com/TheAstrayDev/astray-verify/main/install.ps1 | powershell -NoProfile -ExecutionPolicy Bypass -';
$verifySchema = [
    '@context' => 'https://schema.org',
    '@type' => 'SoftwareApplication',
    'name' => 'Astray Verify',
    'description' => $seo['description'],
    'applicationCategory' => 'DeveloperApplication',
    'operatingSystem' => 'Linux, macOS, Windows',
    'softwareVersion' => '0.1.0',
    'codeRepository' => 'https://github.com/TheAstrayDev/astray-verify',
    'url' => 'https://theastraydev.online/verify',
    'downloadUrl' => 'https://github.com/TheAstrayDev/astray-verify/releases/latest',
    'offers' => [
        '@type' => 'Offer',
        'price' => '0',
        'priceCurrency' => 'USD',
        'availability' => 'https://schema.org/InStock',
    ],
];
$verifyFaqSchema = [
    '@context' => 'https://schema.org',
    '@type' => 'FAQPage',
    'mainEntity' => [
        [
            '@type' => 'Question',
            'name' => 'Who is Astray Verify for?',
            'acceptedAnswer' => ['@type' => 'Answer', 'text' => 'Astray Verify is for people who build or maintain MCP servers and want to test the interface their AI clients call.'],
        ],
        [
            '@type' => 'Question',
            'name' => 'Does Astray Verify call an AI model?',
            'acceptedAnswer' => ['@type' => 'Answer', 'text' => 'No. It talks directly to an MCP server, so it does not need a model account or API key.'],
        ],
        [
            '@type' => 'Question',
            'name' => 'Does Astray Verify replace MCP Inspector?',
            'acceptedAnswer' => ['@type' => 'Answer', 'text' => 'No. MCP Inspector helps explore and debug a server. Astray Verify records a known-good interface and replays it after changes.'],
        ],
    ],
];
?>
<!doctype html>
<html lang="en" prefix="og: https://ogp.me/ns#">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title><?= e($seo['title']) ?></title>
  <meta name="description" content="<?= e($seo['description']) ?>">
  <meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1">
  <meta name="theme-color" content="#102121">
  <link rel="canonical" href="https://theastraydev.online/verify">
  <link rel="icon" href="/assets/img/logo.jpg" type="image/jpeg">
  <link rel="apple-touch-icon" href="/assets/img/logo.jpg">
  <link rel="manifest" href="/site.webmanifest">
  <meta property="og:type" content="website">
  <meta property="og:site_name" content="The Astray">
  <meta property="og:title" content="<?= e($seo['title']) ?>">
  <meta property="og:description" content="<?= e($seo['description']) ?>">
  <meta property="og:url" content="https://theastraydev.online/verify">
  <meta property="og:image" content="https://theastraydev.online/assets/img/logo.jpg">
  <meta property="og:image:alt" content="Astray Verify — MCP contract testing">
  <meta property="og:locale" content="en_US">
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:title" content="<?= e($seo['title']) ?>">
  <meta name="twitter:description" content="<?= e($seo['description']) ?>">
  <meta name="twitter:image" content="https://theastraydev.online/assets/img/logo.jpg">
  <script type="application/ld+json"><?= json_encode($verifySchema, JSON_UNESCAPED_SLASHES | JSON_HEX_TAG | JSON_HEX_AMP) ?></script>
  <script type="application/ld+json"><?= json_encode($verifyFaqSchema, JSON_UNESCAPED_SLASHES | JSON_HEX_TAG | JSON_HEX_AMP) ?></script>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=Manrope:wght@400;500;600&family=Newsreader:ital,opsz,wght@0,6..72,400;0,6..72,700;1,6..72,400&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/assets/css/main.css?v=cli6">
  <link rel="stylesheet" href="/assets/css/verify.css?v=6">
</head>
<body>

<main class="verify" id="main">
  <div class="verify-signal-field" aria-hidden="true"><span></span><span></span><span></span></div>
  <nav class="verify-nav" aria-label="Primary navigation">
    <a class="verify-nav__brand" href="/" aria-label="The Astray home">THE ASTRAY <span>/</span> VERIFY</a>
    <div class="verify-nav__links">
      <a href="#how">How it works</a>
      <a href="#install">Install</a>
      <a href="https://github.com/TheAstrayDev/astray-verify" target="_blank" rel="noopener noreferrer">GitHub ↗</a>
    </div>
  </nav>

  <section class="verify-hero" aria-labelledby="verify-title">
    <div class="verify-hero__copy verify-intro">
      <p class="verify-kicker"><span class="verify-status" aria-hidden="true"></span> MCP CONTRACT TESTING · OPEN SOURCE</p>
      <h1 id="verify-title">Your MCP server still starts.<br><em>Did it still work?</em></h1>
      <p class="verify-hero__lead">Astray Verify records the tools your AI clients rely on, then catches broken names, schemas, and protocol output before you release.</p>
      <div class="verify-actions">
        <a class="verify-button verify-button--solid" href="#install">Install Astray Verify <span aria-hidden="true">↓</span></a>
        <a class="verify-button verify-button--line" href="https://github.com/TheAstrayDev/astray-verify" target="_blank" rel="noopener noreferrer">Read the source <span aria-hidden="true">↗</span></a>
      </div>
      <p class="verify-hero__note">A small CI check for authors of MCP servers. No model, account, or cloud required.</p>
    </div>

    <div class="verify-proof verify-intro" aria-label="Example Astray Verify contract check">
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

  <section class="verify-problem verify-reveal" aria-labelledby="problem-title">
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

  <section class="verify-flow verify-reveal" id="how" aria-labelledby="flow-title">
    <div class="verify-flow__head">
      <div class="verify-section-label">THREE STEPS</div>
      <h2 id="flow-title">Record once.<br>Protect every release.</h2>
    </div>
    <ol class="verify-steps">
      <li class="verify-reveal__item">
        <span class="verify-step__number">01</span>
        <h3>Record</h3>
        <p>Start your MCP server once. Astray Verify performs the handshake and saves its tools and schemas as a small JSON fixture.</p>
        <code>astray-verify record --name github -- &lt;server&gt;</code>
      </li>
      <li class="verify-reveal__item">
        <span class="verify-step__number">02</span>
        <h3>Commit</h3>
        <p>Review the fixture with your code. It becomes the explicit promise your server makes to every AI client.</p>
        <code>fixtures/github.mcp.json</code>
      </li>
      <li class="verify-reveal__item">
        <span class="verify-step__number">03</span>
        <h3>Replay</h3>
        <p>Run one command locally or in CI. A change fails loudly before it becomes somebody else's broken agent workflow.</p>
        <code>astray-verify test</code>
      </li>
    </ol>
  </section>

  <section class="verify-now verify-reveal" aria-labelledby="now-title">
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

  <section class="verify-install verify-reveal" id="install" aria-labelledby="install-title">
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

  <section class="verify-faq verify-reveal" aria-labelledby="faq-title">
    <div class="verify-section-label">FAQ</div>
    <div>
      <h2 id="faq-title">The short answers.</h2>
      <details open><summary>Who is this for?</summary><p>Anyone building or maintaining an MCP server. If an AI client calls your tools, their public interface is worth testing.</p></details>
      <details><summary>Does it call an AI model?</summary><p>No. Astray Verify talks to your MCP server directly. It checks the protocol contract, so it does not require an API key or model account.</p></details>
      <details><summary>Does it replace the MCP Inspector?</summary><p>No. Use Inspector to explore and debug. Use Astray Verify to commit a known-good interface and replay it after every change.</p></details>
    </div>
  </section>
</main>

<script src="/assets/js/verify.js?v=3" defer></script>
<?php require __DIR__ . '/includes/footer.php'; ?>
