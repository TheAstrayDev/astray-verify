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

try {
  Write-Host "Downloading $asset…"
  Invoke-WebRequest -Uri $url -OutFile $destination
  Write-Host "Installed prebuilt Astray Verify to $destination"
  Write-Host "Add $installDir to your User PATH, then open a new terminal."
} catch {
  if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "No prebuilt Windows binary is available yet and Rust is not installed. Install Rust from https://rustup.rs and run this command again."
  }
  Write-Host "No prebuilt binary yet; building the latest release from source…"
  cargo install --git "https://github.com/$repo.git" --locked --force astray-verify
}
