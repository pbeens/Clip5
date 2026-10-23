# Skill: Release a new Clip5 version

## Purpose

Publish a new version of Clip5: bump the version, finalize the changelog, build both platforms, update the website, and create the GitHub Release.

## When to use

- The maintainer asks to release, ship, or publish a version.

## When not to use

- Ordinary code changes. Those only add entries under `## [Unreleased]` at the top of `CHANGELOG.md` (creating it if needed).

## Inputs

- The new version number (Semantic Versioning: patch for fixes, minor for features).
- Access to the Windows PC and the Mac.

## Outputs and side effects

- Updated `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, `RELEASE_NOTES.md`, `docs/`
- `release/Clip5-<version>-windows.zip` and `release/Clip5-<version>-macos.zip`
- A git tag `v<version>` and a GitHub Release (externally visible)

## Steps

1. Confirm the version number with the maintainer.
2. Set `version` in `Cargo.toml`.
3. In `CHANGELOG.md`, rename `## [Unreleased]` to `## [<version>] - <YYYY-MM-DD>`, and add a `[<version>]: https://github.com/pbeens/Clip5/releases/tag/v<version>` link at the bottom. Don't leave an empty `[Unreleased]` section.
4. Update `RELEASE_NOTES.md` for this version: tag, title, file names, and a description built from the changelog section.
5. Update the version or download details on the website in `website/` if needed, then run `quarto render website`.
6. Run `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `powershell -ExecutionPolicy Bypass -File scripts/build_windows.ps1` on Windows. Launch the `.exe` and smoke-test it: type, copy, clear, quit, relaunch, check the text is still there.
7. **Ask for approval**, then commit: `Release v<version>`.
8. On the Mac: follow the README section "Building the release files → macOS" (pull, `bash scripts/build_macos.sh`, smoke-test).
9. **Ask for approval**, then tag `v<version>`, push, and create the GitHub Release using the values in `RELEASE_NOTES.md`, attaching both files from `release/`.
10. Update `tasks.md`.

## Verification

- The GitHub Release page shows both files, and each downloads and runs.
- The website shows the new version and its download links work.
- `CHANGELOG.md` has a dated section for the version, with a link at the bottom, and no `[Unreleased]` section.
