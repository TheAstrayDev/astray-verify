$ErrorActionPreference = "Stop"

$repo = "TheAstrayDev/astray-verify"
$installDir = if ($env:ASTRAY_VERIFY_INSTALL_DIR) {
  $env:ASTRAY_VERIFY_INSTALL_DIR
} else {
  Join-Path $env:LOCALAPPDATA "AstrayVerify\bin"
}
$asset = "astray-verify-windows-x86_64.exe"
$url = "https://github.com/$repo/releases/latest/download/$asset"

New-Item -ItemType Directory -Force -Path $installDir | Out-Null
$destination = Join-Path $installDir "astray-verify.exe"

Write-Host "Downloading $asset…"
Invoke-WebRequest -Uri $url -OutFile $destination
Write-Host "Installed Astray Verify to $destination"
Write-Host "Add $installDir to your User PATH, then open a new terminal."
