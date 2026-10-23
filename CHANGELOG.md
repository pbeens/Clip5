# Changelog

All notable changes to Clip5 are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-10-23

First public release.

### Added

- Five text slots, each with a Copy button; text is saved automatically to `clip5.json` in the user's app data folder.
- "On top" option to keep the window above other windows (on by default).
- Long text is shortened with "…" to fit its slot; click a slot to see and edit all of it. Switching to another app ends editing, so the "…" comes back.
- Spaces, tabs, and line breaks around pasted text are trimmed.
- The window can be resized; making it taller gives each slot more lines.
- The slots, the "On top" setting, and the window position and size are remembered between launches. If the saved position is off-screen (for example, a monitor was disconnected), the window opens centred on the main screen.
- A small × in each slot clears it.
- The window title shows the version number.
- Clip5 app icon for the window, the Windows `.exe`, and the macOS app.
- The macOS app is universal (Apple Silicon and Intel).
- Footer links to Buy Me a Coffee and to GitHub Issues.

[1.0.0]: https://github.com/pbeens/Clip5/releases/tag/v1.0.0
