// Hide the console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod storage;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use eframe::egui;
use storage::{Config, SLOT_COUNT, WindowRect};

const BUY_ME_A_COFFEE_URL: &str = "https://buymeacoffee.com/pbeens";
const ISSUES_URL: &str = "https://github.com/pbeens/Clip5/issues";

/// How long to wait after the last change before saving.
const SAVE_DELAY: Duration = Duration::from_millis(500);
/// How long a Copy button shows "Copied!".
const COPIED_FEEDBACK: Duration = Duration::from_millis(1200);
/// Every slot is at least this many lines tall. A taller window gives each slot more lines.
/// Text that doesn't fit is cut off with "…" until the slot is clicked, and then scrolls.
const MIN_SLOT_LINES: f32 = 2.0;
/// Space between a slot's border and its text. The wider right margin keeps text clear
/// of the × button.
const SLOT_MARGIN: egui::Margin = egui::Margin {
    left: 6,
    right: 20,
    top: 4,
    bottom: 4,
};
/// Size of the × button that clears a slot.
const CLEAR_SIZE: f32 = 16.0;
const MIN_WIDTH: f32 = 300.0;
/// Width on first launch. The height is fitted to the content.
const DEFAULT_WIDTH: f32 = 340.0;

fn main() -> eframe::Result {
    let dir = storage::data_dir();
    let config = dir.as_deref().map(storage::load).unwrap_or_default();

    let mut viewport = egui::ViewportBuilder::default()
        .with_title(concat!("Clip5 v", env!("CARGO_PKG_VERSION")))
        .with_icon(
            eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon-256.png"))
                .expect("the bundled icon is a valid PNG"),
        )
        .with_min_inner_size([MIN_WIDTH, 0.0])
        .with_maximize_button(false);
    viewport = match config.window {
        Some(window) => viewport
            .with_position([window.x, window.y])
            .with_inner_size([window.width, window.height]),
        // A first guess; Clip5App fits the height to the content on the first frame.
        None => viewport.with_inner_size([DEFAULT_WIDTH, 280.0]),
    };
    if config.always_on_top {
        viewport = viewport.with_always_on_top();
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Clip5",
        options,
        Box::new(|cc| {
            if let Some(window) = cc.winit_window()
                && let Ok(position) = window.outer_position()
            {
                // Everything here is in physical pixels.
                let rect = |x: i32, y: i32, width: u32, height: u32| {
                    egui::Rect::from_min_size(
                        egui::pos2(x as f32, y as f32),
                        egui::vec2(width as f32, height as f32),
                    )
                };
                let size = window.outer_size();
                let frame = rect(position.x, position.y, size.width, size.height);
                // The primary monitor goes first so an off-screen window is moved there.
                let monitors: Vec<egui::Rect> = window
                    .primary_monitor()
                    .into_iter()
                    .chain(window.available_monitors())
                    .map(|monitor| {
                        let (position, size) = (monitor.position(), monitor.size());
                        rect(position.x, position.y, size.width, size.height)
                    })
                    .collect();
                let scale = window.scale_factor() as f32;
                if let Some(moved) = on_screen_position(frame, &monitors, scale) {
                    cc.egui_ctx
                        .send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                            (moved.to_vec2() / scale).to_pos2(),
                        ));
                }
            }
            Ok(Box::new(Clip5App::new(dir, config)))
        }),
    )
}

/// Drops spaces, tabs, and line breaks around pasted text before a slot sees it.
fn trim_pasted_text(events: &mut [egui::Event]) {
    for event in events {
        if let egui::Event::Paste(text) = event {
            *text = text.trim().to_owned();
        }
    }
}

/// Returns a new position for a window whose title bar can't be reached on any monitor
/// (for example, because the monitor it was last on has been disconnected), centred on the
/// first monitor. Returns `None` if the window can stay where it is. `window` and `monitors`
/// are in physical pixels; `scale` is pixels per point.
fn on_screen_position(
    window: egui::Rect,
    monitors: &[egui::Rect],
    scale: f32,
) -> Option<egui::Pos2> {
    let title_bar = egui::Rect::from_min_size(window.min, egui::vec2(window.width(), 30.0 * scale));
    let reachable = monitors.iter().any(|monitor| {
        let visible = monitor.intersect(title_bar);
        visible.width() >= 60.0 * scale && visible.height() >= 10.0 * scale
    });
    let screen = monitors.first()?;
    if reachable {
        return None;
    }
    Some((screen.center() - window.size() / 2.0).max(screen.min))
}

fn set_window_level(ctx: &egui::Context, always_on_top: bool) {
    let level = if always_on_top {
        egui::WindowLevel::AlwaysOnTop
    } else {
        egui::WindowLevel::Normal
    };
    ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(level));
}

struct Clip5App {
    /// Where clip5.json lives, or `None` if the user's config folder can't be found.
    dir: Option<PathBuf>,
    config: Config,
    /// When the config last changed, if there are unsaved changes.
    edited_at: Option<Instant>,
    /// The slot copied most recently, and when.
    copied: Option<(usize, Instant)>,
    save_error: Option<String>,
    /// True on first launch, until the window has been shrunk to fit the content.
    fit_to_content: bool,
    /// The minimum window height last sent to the OS.
    min_height: f32,
}

impl Clip5App {
    fn new(dir: Option<PathBuf>, config: Config) -> Self {
        let save_error = dir
            .is_none()
            .then(|| "Can't find your user folder, so changes won't be saved.".to_owned());
        Self {
            dir,
            fit_to_content: config.window.is_none(),
            config,
            edited_at: None,
            copied: None,
            save_error,
            min_height: 0.0,
        }
    }

    /// Shows the slots and returns the smallest height they fit in.
    fn slots_ui(&mut self, ui: &mut egui::Ui) -> f32 {
        let top = ui.max_rect().top();
        if let Some(error) = &self.save_error {
            ui.colored_label(ui.visuals().error_fg_color, error);
        }

        let now = Instant::now();
        let line_height = ui.text_style_height(&egui::TextStyle::Body);
        let spacing = ui.spacing().item_spacing.y;
        // The extra 2 points make room for the slot's border.
        let frame_height = SLOT_MARGIN.sum().y + 2.0;
        let min_slot_height = line_height * MIN_SLOT_LINES + frame_height;
        // Share any extra window height between the slots.
        let gaps = spacing * (SLOT_COUNT - 1) as f32;
        let slot_height = ((ui.available_height() - gaps) / SLOT_COUNT as f32).max(min_slot_height);
        let max_lines = ((slot_height - frame_height) / line_height).floor() as usize;
        let min_content_height =
            ui.cursor().top() - top + min_slot_height * SLOT_COUNT as f32 + gaps;

        for i in 0..SLOT_COUNT {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let text = &mut self.config.slots[i];
                let copied = matches!(self.copied, Some((slot, at)) if slot == i && now - at < COPIED_FEEDBACK);
                let label = if copied { "Copied!" } else { "Copy" };
                let button = egui::Button::new(label).min_size(egui::vec2(64.0, 0.0));
                if ui.add_enabled(!text.is_empty(), button).clicked() {
                    ui.ctx().copy_text(text.clone());
                    self.copied = Some((i, now));
                }

                let id = egui::Id::unique(("slot", i));
                let editing = ui.memory(|memory| memory.has_focus(id));
                let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
                    let mut job = egui::text::LayoutJob::simple(
                        text.as_str().to_owned(),
                        egui::TextStyle::Body.resolve(ui.style()),
                        ui.visuals().widgets.inactive.text_color(),
                        wrap_width,
                    );
                    if !editing {
                        // Show only the lines that fit, ending in "…" if there's more.
                        job.wrap.max_rows = max_lines;
                    }
                    ui.fonts_mut(|fonts| fonts.layout_job(job))
                };

                let area = egui::ScrollArea::vertical()
                    .id_salt(i)
                    .max_height(slot_height)
                    .min_scrolled_height(slot_height)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        let edit = egui::TextEdit::multiline(text)
                            .id(id)
                            .desired_rows(max_lines)
                            .desired_width(f32::INFINITY)
                            .min_size(egui::vec2(0.0, slot_height))
                            .margin(SLOT_MARGIN)
                            .hint_text(format!("Slot {}", i + 1))
                            .layouter(&mut layouter);
                        if ui.add(edit).changed() {
                            self.edited_at = Some(now);
                        }
                    });

                if !text.is_empty() {
                    // A small × in the slot's bottom-right corner clears it.
                    let corner = area.inner_rect.right_bottom() - egui::vec2(2.0, 2.0);
                    let rect = egui::Rect::from_min_max(corner - egui::Vec2::splat(CLEAR_SIZE), corner);
                    let clear = egui::Button::new(egui::RichText::new("×").size(14.0)).frame(false);
                    if ui.put(rect, clear).on_hover_text("Clear this slot").clicked() {
                        text.clear();
                        self.edited_at = Some(now);
                    }
                }
            });
        }

        // Repaint when "Copied!" should turn back into "Copy".
        if let Some((_, at)) = self.copied {
            let shown = now - at;
            if shown < COPIED_FEEDBACK {
                ui.ctx().request_repaint_after(COPIED_FEEDBACK - shown);
            }
        }
        min_content_height
    }

    fn footer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let coffee = egui::Button::new(
                egui::RichText::new("Buy Me a Coffee").color(egui::Color32::BLACK),
            )
            .fill(egui::Color32::from_rgb(0xFF, 0xDD, 0x00));
            if ui.add(coffee).clicked() {
                ui.ctx()
                    .open_url(egui::OpenUrl::new_tab(BUY_ME_A_COFFEE_URL));
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.hyperlink_to("Report an issue", ISSUES_URL);
                if ui
                    .checkbox(&mut self.config.always_on_top, "On top")
                    .on_hover_text("Keep Clip5 above other windows")
                    .changed()
                {
                    set_window_level(ui.ctx(), self.config.always_on_top);
                    self.edited_at = Some(Instant::now());
                }
            });
        });
    }

    /// Keeps the window at least tall enough for the slots, and on first launch shrinks it
    /// to exactly that. `shortfall` is how much taller the slots need the window to be
    /// (negative if there is room to spare).
    fn fit_window(&mut self, ctx: &egui::Context, shortfall: f32) {
        let inner = ctx.input(|i| i.viewport().inner_rect);
        let Some(inner) = inner else {
            ctx.request_repaint();
            return;
        };
        let min_height = inner.height() + shortfall;
        if (min_height - self.min_height).abs() > 0.5 {
            self.min_height = min_height;
            ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(egui::vec2(
                MIN_WIDTH, min_height,
            )));
        }
        if self.fit_to_content {
            self.fit_to_content = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                inner.width(),
                min_height,
            )));
        }
    }

    /// Records the window's position and size so the next launch can restore them.
    fn track_window(&mut self, ctx: &egui::Context) {
        let (outer, inner, minimized) = ctx.input(|i| {
            let viewport = i.viewport();
            (viewport.outer_rect, viewport.inner_rect, viewport.minimized)
        });
        let (Some(outer), Some(inner)) = (outer, inner) else {
            return;
        };
        if minimized == Some(true) {
            return;
        }
        let window = Some(WindowRect {
            x: outer.min.x.round(),
            y: outer.min.y.round(),
            width: inner.width().round(),
            height: inner.height().round(),
        });
        if self.config.window != window {
            self.config.window = window;
            self.edited_at = Some(Instant::now());
        }
    }

    /// Saves once nothing has changed for `SAVE_DELAY`.
    fn save_when_due(&mut self, ctx: &egui::Context) {
        if let Some(at) = self.edited_at {
            let waited = at.elapsed();
            if waited >= SAVE_DELAY {
                self.save_now();
            } else {
                ctx.request_repaint_after(SAVE_DELAY - waited);
            }
        }
    }

    fn save_now(&mut self) {
        if self.edited_at.take().is_none() {
            return;
        }
        let Some(dir) = &self.dir else { return };
        self.save_error = storage::save(dir, &self.config)
            .err()
            .map(|err| format!("Couldn't save your clips: {err}"));
    }
}

impl eframe::App for Clip5App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Switching to another app ends editing, so the slot goes back to showing "…".
        if ui.input(|i| i.viewport().focused) == Some(false) {
            ui.memory_mut(|memory| {
                if let Some(id) = memory.focused() {
                    memory.surrender_focus(id);
                }
            });
        }
        ui.input_mut(|input| trim_pasted_text(&mut input.events));

        egui::Panel::bottom("footer").show(ui, |ui| self.footer(ui));
        let shortfall = egui::CentralPanel::default()
            .show(ui, |ui| {
                let available = ui.max_rect().height();
                self.slots_ui(ui) - available
            })
            .inner;
        self.fit_window(ui.ctx(), shortfall);
        self.track_window(ui.ctx());
        self.save_when_due(ui.ctx());
    }

    /// Saves any change made just before closing.
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: f32, y: f32, width: f32, height: f32) -> egui::Rect {
        egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, height))
    }

    const LAPTOP: egui::Rect =
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1920.0, 1080.0));

    #[test]
    fn pasted_text_is_trimmed() {
        let mut events = vec![
            egui::Event::Paste("   hello world \t\r\n".to_owned()),
            egui::Event::Text("  typed  ".to_owned()),
        ];
        trim_pasted_text(&mut events);
        assert_eq!(events[0], egui::Event::Paste("hello world".to_owned()));
        // Typed text is left alone, so spaces between words still work.
        assert_eq!(events[1], egui::Event::Text("  typed  ".to_owned()));
    }

    #[test]
    fn window_on_screen_stays_put() {
        assert_eq!(
            on_screen_position(rect(100.0, 100.0, 340.0, 285.0), &[LAPTOP], 1.0),
            None
        );
    }

    #[test]
    fn window_on_second_monitor_stays_put() {
        let left_monitor = rect(-2560.0, 0.0, 2560.0, 1440.0);
        let window = rect(-1000.0, 200.0, 340.0, 285.0);
        assert_eq!(
            on_screen_position(window, &[LAPTOP, left_monitor], 1.0),
            None
        );
    }

    #[test]
    fn window_on_disconnected_monitor_is_centred() {
        let window = rect(-1000.0, 200.0, 340.0, 285.0);
        assert_eq!(
            on_screen_position(window, &[LAPTOP], 1.0),
            Some(egui::pos2(790.0, 397.5))
        );
    }

    #[test]
    fn window_with_title_bar_above_screen_is_moved() {
        // The body is visible, but the title bar (the only way to drag it) is not.
        let window = rect(100.0, -40.0, 340.0, 285.0);
        assert!(on_screen_position(window, &[LAPTOP], 1.0).is_some());
    }

    #[test]
    fn barely_visible_window_is_moved() {
        let window = rect(1900.0, 100.0, 340.0, 285.0);
        assert!(on_screen_position(window, &[LAPTOP], 1.0).is_some());
    }
}
