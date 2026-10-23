# Release Notes

Everything needed to fill in the GitHub **Draft a new release** form for the next release. Update this file as part of each release (see [skills/release/SKILL.md](skills/release/SKILL.md)). The permanent history of changes lives in [CHANGELOG.md](CHANGELOG.md).

## Next release: v1.0.0

| Field | Value |
|---|---|
| **Tag** | `v1.0.0` (create new tag on publish) |
| **Target** | `main` |
| **Release title** | `Clip5 v1.0.0` |
| **Set as a pre-release** | No |
| **Set as the latest release** | Yes |
| **Release date** | 2026-10-23 (as in `CHANGELOG.md`; update both if you publish on a later day) |

### Files to attach

| File | Built by | Built on |
|---|---|---|
| `Clip5-1.0.0-windows.zip` | `powershell -ExecutionPolicy Bypass -File scripts/build_windows.ps1` | Windows PC |
| `Clip5-1.0.0-macos.zip` | `bash scripts/build_macos.sh` | Mac |

Both files land in `release/`.

### Before publishing

- [x] `version` in `Cargo.toml` is `1.0.0`
- [x] `CHANGELOG.md` has a dated `## [1.0.0]` section
- [x] Windows build smoke-tested: type, copy, clear, quit, reopen, text still there
- [ ] Mac build smoke-tested the same way (see the README, "Building the release files → macOS")
- [x] Website re-rendered (`quarto render website`)
- [ ] Everything committed and pushed, and the repository made public
- [ ] GitHub Pages turned on (**Settings → Pages**: branch `main`, folder `/docs`)

### Description

Copy everything between the lines into the release description box.

---

**Clip5 is a tiny always-on-top window with five clipboard slots for Windows and macOS.**

Type or paste text into any of the five slots. Click **Copy** beside a slot to put its text on the clipboard. Your text is saved automatically and is still there the next time you open Clip5.

#### What's in this first release

- Five text slots, each with a **Copy** button and a small **×** to clear it
- Text is saved automatically as you type
- Long text is shortened with "…" to fit its slot. Click a slot to see and edit all of it. **Copy** always copies the full text.
- Spaces and blank lines around pasted text are removed automatically
- **On top** option keeps Clip5 above your other windows (on by default)
- Resize the window to suit you; making it taller shows more of each slot
- Clip5 reopens with the same text, size, position, and **On top** setting

#### Download

- **Windows:** download `Clip5-1.0.0-windows.zip`, right-click it, and choose **Extract All**. `Clip5.exe` is a standalone program with nothing to install: move it wherever you like and double-click it.
  Because Clip5 isn't code-signed, Windows may show "Windows protected your PC". Click **More info**, then **Run anyway**.
- **macOS** (Apple Silicon and Intel): download `Clip5-1.0.0-macos.zip`, double-click it to unzip, and drag `Clip5.app` into your **Applications** folder.
  The first time you open it, macOS can't verify the developer. Click **Done**, then go to **System Settings → Privacy & Security** and click **Open Anyway**.

To update later, replace `Clip5.exe` or `Clip5.app` with the new version. Your saved text and settings are kept.

#### Where your text and settings are saved

- Windows: `%APPDATA%\Clip5\clip5.json`
- macOS: `~/Library/Application Support/Clip5/clip5.json`

#### Links

- Website: https://peter.beens.ca/Clip5/
- Report a bug or request a feature: https://github.com/pbeens/Clip5/issues
- Full changelog: https://github.com/pbeens/Clip5/blob/main/CHANGELOG.md
- If Clip5 saves you time, please consider [buying me a coffee](https://buymeacoffee.com/pbeens).

---
