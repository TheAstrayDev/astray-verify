<?php
declare(strict_types=1);

require __DIR__ . '/includes/bootstrap.php';
require __DIR__ . '/includes/data.php';
require_once __DIR__ . '/includes/seo.php';

header('Content-Type: application/xml; charset=utf-8');
header('X-Robots-Tag: noindex');
header('Cache-Control: public, max-age=3600');

$base = site_base_url();
$today = gmdate('Y-m-d');
$urls = [
    ['loc' => $base . '/', 'lastmod' => $today, 'changefreq' => 'weekly', 'priority' => '1.0', 'alts' => true],
    ['loc' => $base . '/verify', 'lastmod' => $today, 'changefreq' => 'weekly', 'priority' => '0.9', 'alts' => false],
];

foreach ($projects as $p) {
    $urls[] = ['loc' => $base . '/project/' . rawurlencode($p['slug']), 'lastmod' => $today, 'changefreq' => 'monthly', 'priority' => '0.8', 'alts' => false];
}

echo '<?xml version="1.0" encoding="UTF-8"?>' . "\n";
?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
<?php foreach ($urls as $u): ?>
  <url>
    <loc><?= htmlspecialchars($u['loc'], ENT_XML1 | ENT_QUOTES, 'UTF-8') ?></loc>
    <lastmod><?= htmlspecialchars($u['lastmod'], ENT_XML1, 'UTF-8') ?></lastmod>
    <changefreq><?= htmlspecialchars($u['changefreq'], ENT_XML1, 'UTF-8') ?></changefreq>
    <priority><?= htmlspecialchars($u['priority'], ENT_XML1, 'UTF-8') ?></priority>
  </url>
<?php endforeach; ?>
</urlset>
