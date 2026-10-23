# Clip5

A tiny always-on-top window with five clipboard slots for Windows and macOS.

Type or paste text into any of the five fields. Click the button beside a field to copy its text to the clipboard, or the small **×** in a field to clear it. Your text is saved automatically and is still there the next time you open Clip5. Long text is shortened with "…" until you click into the slot to edit it, and spaces and blank lines around pasted text are removed automatically. Untick **On top** if you don't want Clip5 to stay above your other windows.

Current version: **1.0.0**. See [CHANGELOG.md](CHANGELOG.md) for what's new.

## Download

Get the latest version from the [Releases page](https://github.com/pbeens/Clip5/releases).

### Windows

1. Download the Windows `.zip` file.
2. Right-click it and choose **Extract All**. Don't run Clip5 from inside the zip, because Windows would run it from a temporary folder.
3. Clip5 is a standalone program, so there's nothing to install. Move `Clip5.exe` wherever you like, such as your Desktop or a `C:\Tools` folder, and double-click it to run.
4. The first time, Windows may show "Windows protected your PC" because Clip5 isn't code-signed. Click **More info**, then **Run anyway**.

Tip: to start Clip5 more easily, right-click `Clip5.exe` and choose **Pin to Start** or **Pin to taskbar**.

**To update**, replace your `Clip5.exe` with the new one. Your pins, shortcuts, and saved text keep working.

### macOS

1. Download the `.zip` file and double-click it to unzip it. You'll get `Clip5.app`.
2. Drag `Clip5.app` into your **Applications** folder.
3. The first time you open it, macOS warns that it can't verify the developer, because Clip5 isn't signed by Apple. Click **Done**, then open **System Settings → Privacy & Security**, scroll down, and click **Open Anyway** next to the message about Clip5. You only need to do this once.

**To update**, drag the new `Clip5.app` into **Applications** and choose **Replace**.

Moving the program doesn't affect your saved text. That's stored separately, as described below.

## Where your text and settings are saved

Clip5 keeps your slots, the **On top** setting, and the window size and position in one file:

- Windows: `%APPDATA%\Clip5\clip5.json`
- macOS: `~/Library/Application Support/Clip5/clip5.json`

## Links

- Website: <https://peter.beens.ca/Clip5/>
- Report a bug or request a feature: [GitHub Issues](https://github.com/pbeens/Clip5/issues)
- Changes: [CHANGELOG.md](CHANGELOG.md)
- Support the project: [Buy Me a Coffee](https://buymeacoffee.com/pbeens)

## Building from source

You can download the source code, change it, and build your own copy of Clip5. You don't need any Rust experience to follow these steps.

### 1. Install Rust

Rust is the only tool you need to build Clip5.

**Windows**

1. Download and run `rustup-init.exe` from <https://rustup.rs/>.
   - If it says the **Visual Studio C++ Build Tools** are missing, accept its offer to install them. Rust needs them to build Windows programs.
   - When asked, choose **1) Proceed with standard installation**.
2. Close and reopen your terminal so the new commands are found.

**macOS**

1. Install Apple's command-line tools, which include the C compiler and linker Rust needs (and Git):

   ```sh
   xcode-select --install
   ```

2. Install Rust:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Choose the default installation, then close and reopen Terminal.

**Check that it worked** (on either system):

```sh
cargo --version
```

It should print a version number. To update Rust later, run `rustup update`.

### 2. Get the source code

Choose one of these options:

- **Download ZIP (simplest):** on the [repository page](https://github.com/pbeens/Clip5), click **Code → Download ZIP**, then unzip it. You won't get future updates automatically. You'd download the ZIP again.
- **GitHub Desktop (recommended if you'll keep working on it):** install [GitHub Desktop](https://desktop.github.com/), then click **Code → Open with GitHub Desktop** on the repository page. GitHub Desktop downloads the code, lets you pull updates with one click, and tracks your changes. You don't need to use any Git commands.
- **Git command line:** if you already use Git:

  ```sh
  git clone https://github.com/pbeens/Clip5.git
  ```

Then open a terminal in the `Clip5` folder. In GitHub Desktop, use **Repository → Open in Command Prompt** (Windows) or **Open in Terminal** (macOS).

If you plan to share your changes, first **fork** the repository on GitHub (GitHub Desktop will offer to do this for you), then download your fork instead.

### 3. Run it

```sh
cargo run
```

The first build downloads the libraries Clip5 uses and takes a few minutes. Later builds are much faster. The app window opens when the build finishes.

### 4. Make changes

- The app's code is in [`src/main.rs`](src/main.rs). The GUI uses [egui](https://github.com/emilk/egui). Its [online demo](https://www.egui.rs/) shows the available widgets.
- Images and icons are in [`assets/`](assets/README.md).
- After editing, run `cargo run` again to see your change.
- Optional tidy-up: `cargo fmt` formats the code, and `cargo clippy` suggests improvements.

**Recommended editor:** [Visual Studio Code](https://code.visualstudio.com/) with the **rust-analyzer** extension. It gives you error highlighting and auto-complete.

### 5. Build a standalone app

- **Windows:** `cargo build --release` produces `target/release/clip5.exe`. This single file can be copied anywhere and run.
- **macOS:** run `cargo install cargo-bundle` once. Then `cargo bundle --release` produces `target/release/bundle/osx/Clip5.app`, which runs on the kind of Mac you built it on. The release script below makes a universal app instead.

For release builds with version numbers in the file names, see [Building the release files](#building-the-release-files) below.

### 6. Editing the website (optional)

The website source is in [`website/`](website/README.md) and is built with [Quarto](https://quarto.org/docs/get-started/). After installing Quarto, run `quarto preview website` to see your changes live.

### 7. Changing the icon (optional)

The icon is drawn in [`assets/icon.svg`](assets/icon.svg). After editing it, run `python scripts/make_icons.py` to regenerate the PNG and ICO files. This needs `rsvg-convert` and Pillow; see [scripts/README.md](scripts/README.md).

## Building the release files

These are the steps the maintainer follows for each release. The full checklist is in [skills/release/SKILL.md](skills/release/SKILL.md), and the text for the GitHub Release is in [RELEASE_NOTES.md](RELEASE_NOTES.md).

### Windows

On the Windows PC, in the Clip5 folder:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/build_windows.ps1
```

(`-ExecutionPolicy Bypass` lets this one script run even though Windows blocks PowerShell scripts by default.)

This creates `release/Clip5-<version>-windows.zip`, containing `Clip5.exe` (with the Clip5 icon and version details built in) and `LICENSE.txt`.

### macOS

The Mac version has to be built on a Mac.

1. **Install Rust** on the Mac (see [step 1](#1-install-rust) above), if you haven't already.
2. **Get the latest code.** In GitHub Desktop, click **Fetch origin**, then **Pull origin**, or run `git pull` in Terminal. If the Mac doesn't have the repository yet, clone it first (see [step 2](#2-get-the-source-code)).
3. **One-time setup:** install the tool that packages Rust programs as Mac apps:

   ```sh
   cargo install cargo-bundle
   ```

4. **Build.** Open Terminal in the `Clip5` folder and run:

   ```sh
   bash scripts/build_macos.sh
   ```

   This creates `release/Clip5-<version>-macos.zip`, which contains `Clip5.app`. The app is **universal**: it runs on both Apple Silicon and Intel Macs. The first run downloads the Intel toolchain and takes a few minutes.

5. **Test it.** In Finder, double-click the `.zip` in the `release` folder, drag `Clip5.app` into **Applications**, and open it. Check that:
   - the Clip5 icon shows in the Dock, and the title bar reads **Clip5 v\<version\>**
   - **Copy**, **×** and **On top** work
   - after you quit and reopen it, your text, the window size and position, and the **On top** setting come back

6. **After publishing the release**, download the `.zip` from the Releases page and open it once. That's the only way to see the first-launch security warning a new user will see. Check that **System Settings → Privacy & Security → Open Anyway** works as described in [Download](#macos) above.

### Contributing

Pull requests are welcome. To send one, you need a fork downloaded with GitHub Desktop or Git; a ZIP download can't be submitted back. For anything bigger than a small fix, please [open an issue](https://github.com/pbeens/Clip5/issues) first so we can discuss it. Add a line describing your change under an `## [Unreleased]` heading at the top of [CHANGELOG.md](CHANGELOG.md), creating the heading if it isn't there yet.

## For contributors and agents

- [AGENTS.md](AGENTS.md): project conventions and rules
- [tasks.md](tasks.md): current work
- [RELEASE_NOTES.md](RELEASE_NOTES.md): details for the next GitHub Release

## Licence

[MIT](LICENSE) © 2026 Peter Beens
