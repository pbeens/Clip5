# assets

Files bundled into the Clip5 app at build time.

| File | Used for |
|---|---|
| `icon.svg` | The source drawing of the app icon. Edit this one. |
| `icon-256.png` | The window icon (embedded by `src/main.rs`). |
| `icon-32.png` … `icon-512.png`, `icon-512@2x.png` | The macOS app icon (listed in `Cargo.toml` for `cargo bundle`). The 1024-pixel file must be named `512@2x`: ICNS has no 1024 size at 1x. |
| `icon.ico` | The Windows `.exe` icon (embedded by `build.rs`). |

The PNG and ICO files are generated. After editing `icon.svg`, run `python scripts/make_icons.py` from the repository root (see [`../scripts/README.md`](../scripts/README.md)) and commit the results.

Rules:

- Use only original artwork or files whose licence allows redistribution in an MIT project. Note the source of any third-party file here. The Clip5 icon is original artwork.
- Use lowercase kebab-case file names.
- Website-only images belong in [`../website/`](../website/README.md), not here.
