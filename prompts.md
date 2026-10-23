# Prompt Log

Program-related prompts, newest first. Paraphrased; not a transcript.

## 2026-10-23T10:50

- **Goal:** Give the Windows download a stable program name.
- **Prompt:** After discussing versioned versus plain names for a portable exe, chose to zip the Windows build so it extracts as `Clip5.exe`.
- **Host:** XPS-8950
- **Context:** Build script now makes `Clip5-<version>-windows.zip` (Clip5.exe + LICENSE.txt); the Mac app also carries the licence; download and update steps revised.

## 2026-10-23T10:30

- **Goal:** Get the project ready to upload for v1.0.0.
- **Prompt:** Do a final check that everything is correct for release (assuming the Mac build gets done), date the changelog's 1.0.0 entry with today's date, and make sure `.gitignore` is right. The repository stays private until the Mac build is done.
- **Host:** XPS-8950

## 2026-10-23T10:10

- **Goal:** Tidy the website's screenshot.
- **Prompt:** The screenshot is centred but its caption is left-aligned; centre the caption to match.
- **Host:** XPS-8950

## 2026-10-23T10:05

- **Goal:** Clean up pasted text.
- **Prompt:** Trim spaces pasted at the beginning or end of a slot's text.
- **Host:** XPS-8950
- **Context:** Trims paste events (including tabs and line breaks) before the slot receives them; typed text is untouched.

## 2026-10-23T10:00

- **Goal:** Make the docs match the app's long-text behaviour.
- **Prompt:** Reported that long text is not shown with an ellipsis but simply hidden, and asked for the website and README to be corrected.
- **Host:** XPS-8950
- **Context:** Root cause: a slot stayed in edit mode (scrolling, no ellipsis) after switching apps. Fixed by ending editing when the window loses focus; docs clarified.

## 2026-10-23T09:45

- **Goal:** Give Clip5 its own icon and document the Mac build.
- **Prompt:** Design the proposed clipboard-with-a-5 icon, and make sure the Mac build and test instructions are in the README.
- **Host:** XPS-8950
- **Context:** SVG source rendered by `scripts/make_icons.py`; `.exe` icon embedded via `build.rs` (winresource); macOS build made universal and ad-hoc signed.

## 2026-10-23T09:30

- **Goal:** Prepare the 1.0.0 release.
- **Prompt:** Asked whether Clip5 is ready to ship as v1.0.0; if so, update the changelog and show "Clip5 v1.0.0" in the window title. Asked whether the app icon is a stock Rust icon or custom.
- **Host:** XPS-8950
- **Context:** Version bumped and title set from `CARGO_PKG_VERSION`; icon is egui's default, and the macOS build is still untested.

## 2026-10-23T09:20

- **Goal:** Make slots quick to empty.
- **Prompt:** Add a small × in the bottom-right corner of each text field that clears its contents.
- **Host:** XPS-8950

## 2026-10-23T09:10

- **Goal:** Clarify installation in the README.
- **Prompt:** Explain in the README's download section that the Windows `.exe` is standalone and can be moved anywhere, and that on macOS the app is dragged into the Applications folder.
- **Host:** XPS-8950
- **Context:** Also updated the macOS first-launch steps for current Gatekeeper behaviour (Privacy & Security → Open Anyway) in the README, website, and release notes.

## 2026-10-23T09:00

- **Goal:** Never open the window off-screen.
- **Prompt:** Agreed that if the saved window position is off-screen (such as after a monitor is disconnected), Clip5 should open on-screen instead.
- **Host:** XPS-8950
- **Context:** Checks the title bar against the connected monitors at startup; unit-tested.

## 2026-10-22T19:25

- **Goal:** Restore vertical resizing and remember the window.
- **Prompt:** Allow vertical resizing again, keep the current compact size as the first-launch default, and save the window size and position in Clip5's own config file so it reopens the same way.
- **Host:** XPS-8950
- **Context:** Window geometry and the "On top" setting moved from eframe's `app.ron` into `clip5.json`; slots share extra height.

## 2026-10-22T19:10

- **Goal:** Prepare for publishing a release.
- **Prompt:** Add a root-level release notes document with everything needed for the GitHub Release: tag, title, description, and other details.
- **Host:** XPS-8950

## 2026-10-22T19:00

- **Goal:** Keep the window compact.
- **Prompt:** Limit each slot to two visible lines with an ellipsis when there is more, instead of growing for multi-line text; allow the window to be resized horizontally but not vertically.
- **Host:** XPS-8950
- **Context:** Uses egui's text-wrap row limit for unfocused slots and locks the window height via viewport min/max size.

## 2026-10-22T18:50

- **Goal:** Show the app on the website.
- **Prompt:** Run the program with dummy data, take a screenshot, and add it to the website.
- **Host:** XPS-8950
- **Context:** Uncovered that a UTF-8 byte-order mark made the data file unreadable; fixed in storage.

## 2026-10-22T18:45

- **Goal:** Improve the website's support and open-source content.
- **Prompt:** Reword the support section to invite buying a coffee if Clip5 saves time; add an open-source section with a short summary of the README's build-from-source steps and a link to the README for details.
- **Host:** XPS-8950

## 2026-10-22T18:30

- **Goal:** Refine the window behaviour and data file naming.
- **Prompt:** Asked for no maximize button (keep minimize), for always-on-top to be an option rather than forced, and why the data file was named `clips.json` instead of after the app; renamed to `clip5.json`.
- **Host:** XPS-8950

## 2026-10-22T18:20

- **Goal:** Build the first version of the app.
- **Prompt:** Confirmed Rust is installed and asked to proceed with writing the app.
- **Host:** XPS-8950
- **Context:** eframe 0.37 with the glow backend; slots stored in the per-user config folder.

## 2026-10-22T18:15

- **Goal:** Lower the barrier to getting the source.
- **Prompt:** Questioned why Git was listed as required on Windows; agreed to make it optional (ZIP download as the simplest route) and asked to recommend GitHub Desktop for downloading the repository.
- **Host:** XPS-8950

## 2026-10-22T18:05

- **Goal:** Make the source code approachable for others.
- **Prompt:** Asked for the README to cover everything someone needs to download, modify, and build the source, including installing Rust.
- **Host:** XPS-8950

## 2026-10-22T18:00

- **Goal:** Settle the technology choice.
- **Prompt:** Asked whether Rust would be a better fit because it is fast and small; after a comparison, chose Rust + egui.
- **Host:** XPS-8950
- **Context:** Replaces the Python/Tkinter plan; build is `cargo build --release` on Windows and `cargo bundle` on macOS.

## 2026-10-22T17:58

- **Goal:** Decide on the build approach and confirm the toolkit.
- **Prompt:** Will build the macOS version manually on a Mac; asked for a changelog to be required; asked whether Electron would be overkill.
- **Host:** XPS-8950

## 2026-10-22T17:56

- **Goal:** Clarify the technology and distribution.
- **Prompt:** Prefers an easy Python GUI (some Tkinter experience); wants users to download a single file that just runs; asked how the toolkit options differ. Buy Me a Coffee username is `pbeens`; final app name is Clip5.
- **Host:** XPS-8950

## 2026-10-22T17:54

- **Goal:** Define the project.
- **Prompt:** Build a small always-on-top desktop app for Windows (and ideally macOS) with five text fields, each with a button that copies its text to the clipboard; contents are editable and always saved (probably JSON in the user profile). Publish a Quarto website; public repo, MIT licence, Buy Me a Coffee link, GitHub Issues for support.
- **Host:** XPS-8950
