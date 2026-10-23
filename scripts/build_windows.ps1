# Builds the release Clip5.exe and copies it to release/ with the version in the name.
$ErrorActionPreference = "Stop"

$version = (Select-String -Path Cargo.toml -Pattern '^version\s*=\s*"(.+)"').Matches[0].Groups[1].Value

cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

New-Item -ItemType Directory -Force release | Out-Null
$out = "release/Clip5-$version-windows.exe"
Copy-Item target/release/clip5.exe $out -Force
Write-Host "Built $out"
