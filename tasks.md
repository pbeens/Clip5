# Tasks

## Current Focus

- Ship Clip5 v1.0.0.

## Next Tasks

- [x] Write the app: five slots, Copy buttons, optional always-on-top, no maximize button, auto-save to `clip5.json`
- [x] Add a footer with the Buy Me a Coffee and Report an issue links
- [x] Run on Windows and check that the slots and "On top" setting are restored after a restart
- [x] Add a Clip5 app icon to `assets/` (window, Windows `.exe`, macOS `.app`)
- [x] Produce the Windows release zip with `scripts/build_windows.ps1` and test it
- [x] Pre-release review: docs, changelog (dated 2026-10-23), release notes, `.gitignore`
- [ ] Commit and push to GitHub (repository private until the Mac build is done)
- [ ] Build and test on the Mac (README: "Building the release files → macOS"); first real run of the universal build script
- [x] Add a Windows screenshot, open-source summary, and support section to the website
- [ ] Add a macOS screenshot to the website (take it on the Mac)
- [ ] Make the repository public, then turn on GitHub Pages (`main` branch, `/docs` folder)
- [ ] First release: v1.0.0 (follow [skills/release/SKILL.md](skills/release/SKILL.md))

## Open Questions

- Should slots have labels? (Deferred to after v1.0.0.)

## Recently Completed

- Project structure initialized (Rust + egui chosen over Python + Tkinter and Electron).
