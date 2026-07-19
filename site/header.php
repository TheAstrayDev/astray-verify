<?php
require_once __DIR__ . '/seo.php';

if (!isset($pageTitle)) {
    $pageTitle = __('seo.title');
}
if (!isset($seo) || !is_array($seo)) {
    $seo = [];
}
if (empty($seo['title'])) {
    $seo['title'] = $pageTitle;
}
$isHome = !isset($isHome) ? false : (bool) $isHome;
$lockSplash = !empty($lockSplash);
$extraStyles = isset($extraStyles) && is_array($extraStyles) ? $extraStyles : [];
?>
<!DOCTYPE html>
<html lang="<?= e(lang()) ?>" prefix="og: https://ogp.me/ns#">
<head>
  <?php seo_render_head($seo); ?>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=Manrope:wght@400;500;600&family=Newsreader:ital,opsz,wght@0,6..72,400;0,6..72,700;1,6..72,400&family=Special+Elite&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/assets/css/main.css?v=cli6">
<?php foreach ($extraStyles as $style): ?>
  <link rel="stylesheet" href="<?= e((string) $style) ?>">
<?php endforeach; ?>
  <link rel="author" href="/humans.txt">
</head>
<body<?= $lockSplash ? ' class="is-locked"' : '' ?><?= $isHome ? ' itemscope itemtype="https://schema.org/WebPage"' : '' ?>>
