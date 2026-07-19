<?php
declare(strict_types=1);

require __DIR__ . '/includes/bootstrap.php';
require __DIR__ . '/includes/data.php';

$slug = isset($_GET['slug']) ? (string) $_GET['slug'] : '';
$slug = preg_replace('/[^a-z0-9\-]/', '', strtolower($slug)) ?? '';
$project = $slug !== '' ? project_by_slug($slug) : null;

if (!$project) {
    http_response_code(404);
    $pageTitle = __('project.not_found') . ' — The Astray';
    $lockSplash = false;
    $isHome = false;
    $seo = [
        'title' => $pageTitle,
        'description' => __('project.not_found_text'),
        'noindex' => true,
        'path' => '/project/' . $slug,
    ];
    require __DIR__ . '/includes/header.php';
    ?>
    <main class="project-page" id="main">
      <a class="project-page__back" href="/#projects"><?= e(__('project.home')) ?></a>
      <div class="not-found">
        <div>
          <h1>404 — <?= e(__('project.not_found')) ?></h1>
          <p style="color: var(--mute); font-family: var(--font-body)"><?= e(__('project.not_found_text')) ?></p>
          <p><a href="/" style="text-decoration: underline"><?= e(__('project.home')) ?></a></p>
        </div>
      </div>
    </main>
    <?php
    require __DIR__ . '/includes/footer.php';
    exit;
}

$path = '/project/' . $project['slug'];
$pageTitle = $project['title'] . ' — ' . $project['category'] . ' | The Astray';
$desc = $project['summary'];
if (mb_strlen($desc) < 80 && !empty($project['body'][0])) {
    $desc = $project['summary'] . ' ' . $project['body'][0];
}
$desc = mb_substr($desc, 0, 160);
$keywords = implode(', ', array_merge(
    [$project['title'], $project['category'], 'The Astray'],
    $project['stack'] ?? []
));

$lockSplash = false;
$isHome = false;
$extraStyles = ['/assets/css/project-pages.css?v=1'];
$seo = [
    'title' => $pageTitle,
    'description' => $desc,
    'keywords' => $keywords,
    'path' => $path,
    'type' => 'article',
    'image' => $project['cover'],
    'jsonld' => [
        seo_jsonld_website(),
        seo_jsonld_person(),
        seo_jsonld_project($project),
        seo_jsonld_breadcrumb([
            ['name' => 'The Astray', 'url' => abs_url('/')],
            ['name' => __('projects.title'), 'url' => abs_url('/#projects')],
            ['name' => $project['title'], 'url' => abs_url($path)],
        ]),
    ],
];

require __DIR__ . '/includes/header.php';
?>

  <main class="project-page" id="main" itemscope itemtype="https://schema.org/SoftwareApplication">
    <nav aria-label="Breadcrumb" style="max-width:720px;margin:0 auto 1rem;font-size:0.85rem;color:var(--mute)">
      <ol style="list-style:none;padding:0;margin:0;display:flex;flex-wrap:wrap;gap:0.35rem">
        <li><a href="/">The Astray</a> <span aria-hidden="true">/</span></li>
        <li><a href="/#projects"><?= e(__('projects.title')) ?></a> <span aria-hidden="true">/</span></li>
        <li aria-current="page"><?= e($project['title']) ?></li>
      </ol>
    </nav>

    <a class="project-page__back" href="/#projects"><?= e(__('project.back')) ?></a>

    <header class="project-page__hero">
      <div class="project-page__meta">
        <span itemprop="applicationCategory"><?= e($project['category']) ?></span>
        ·
        <time datetime="<?= e($project['year']) ?>"><?= e($project['year']) ?></time>
      </div>
      <h1 itemprop="name"><?= e($project['title']) ?></h1>
      <p style="max-width: 48ch; color: #c9c3b6; font-size: 1.15rem" itemprop="description">
        <?= e($project['summary']) ?>
      </p>
    </header>

    <div class="project-page__cover">
      <img
        src="<?= e($project['cover']) ?>"
        alt="<?= e($project['title'] . ' — ' . $project['category'] . ' by The Astray') ?>"
        itemprop="image"
        width="1200"
        height="675"
        loading="eager"
        decoding="async"
      >
    </div>

    <section class="project-video" aria-labelledby="project-video-title">
      <div class="project-video__screen" role="img" aria-label="<?= e(($project['video_label'] ?? 'Видео проекта') . ': скоро') ?>">
        <span class="project-video__scan" aria-hidden="true"></span>
        <span class="project-video__eyebrow">PROJECT REEL / 00:00</span>
        <div>
          <span class="project-video__play" aria-hidden="true">▶</span>
          <h2 id="project-video-title"><?= e($project['video_label'] ?? 'Видео проекта в подготовке') ?></h2>
          <p><?= e($project['video_note'] ?? 'Здесь появится короткий ролик о задаче, процессе и результате проекта.') ?></p>
        </div>
        <span class="project-video__corner">THE ASTRAY</span>
      </div>
    </section>

    <div class="project-page__content" itemprop="about">
      <?php foreach ($project['body'] as $para): ?>
        <p><?= e($para) ?></p>
      <?php endforeach; ?>

      <h2 style="font-size:1rem;margin-top:2rem;letter-spacing:0.06em;text-transform:uppercase;color:var(--mute)">
        <?= e(__('project.stack')) ?>
      </h2>
      <ul class="project-page__stack">
        <?php foreach ($project['stack'] as $tag): ?>
          <li itemprop="keywords"><?= e($tag) ?></li>
        <?php endforeach; ?>
      </ul>

      <?php if (!empty($project['link'])): ?>
        <p style="margin-top: 2rem">
          <a class="offer__cta" style="border-color: var(--paper); color: var(--paper)" href="<?= e($project['link']) ?>" target="_blank" rel="noopener noreferrer">
            <?= e($project['link_label'] ?? 'Открыть проект') ?> →
          </a>
        </p>
      <?php endif; ?>

      <p style="margin-top: 2.5rem">
        <a class="offer__cta" style="border-color: var(--paper); color: var(--paper)" href="https://t.me/Jkkaall" target="_blank" rel="noopener noreferrer me">
          <?= e(__('project.cta')) ?>
        </a>
      </p>
    </div>
  </main>

<?php require __DIR__ . '/includes/footer.php'; ?>
