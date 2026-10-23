# scripts

Build and maintenance scripts for Clip5. Run them from the repository root.

| Script | Platform | What it does |
|---|---|---|
| [`build_windows.ps1`](build_windows.ps1) | Windows (PowerShell) | Builds `release/Clip5-<version>-windows.exe` |
| [`build_macos.sh`](build_macos.sh) | macOS | Builds a universal, ad-hoc-signed `Clip5.app` and zips it to `release/Clip5-<version>-macos.zip` |
| [`make_icons.py`](make_icons.py) | Any | Regenerates the icon PNGs and ICO in `assets/` from `assets/icon.svg` |

```powershell
# Windows (Bypass lets the script run under the default execution policy)
powershell -ExecutionPolicy Bypass -File scripts/build_windows.ps1
```

```sh
# macOS (one-time setup: cargo install cargo-bundle)
bash scripts/build_macos.sh
```

```sh
# Icons: needs rsvg-convert (librsvg) and Pillow (pip install pillow)
python scripts/make_icons.py
```

`release/` is git-ignored. Upload its files to the GitHub Release, then delete them. Step-by-step release instructions are in the [README](../README.md#building-the-release-files).
