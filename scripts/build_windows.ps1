# Builds Clip5.exe and zips it, with the licence, into release/Clip5-<version>-windows.zip.
# The exe keeps a plain name so users can update by replacing it without breaking pins.
$ErrorActionPreference = "Stop"

$version = (Select-String -Path Cargo.toml -Pattern '^version\s*=\s*"(.+)"').Matches[0].Groups[1].Value

cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$staging = "release/Clip5-$version-windows"
if (Test-Path $staging) { Remove-Item -Recurse -Force $staging }
New-Item -ItemType Directory -Force $staging | Out-Null
Copy-Item target/release/clip5.exe "$staging/Clip5.exe"
Copy-Item LICENSE "$staging/LICENSE.txt"

$out = "release/Clip5-$version-windows.zip"
if (Test-Path $out) { Remove-Item -Force $out }
Compress-Archive -Path "$staging/*" -DestinationPath $out
Remove-Item -Recurse -Force $staging
Write-Host "Built $out"
