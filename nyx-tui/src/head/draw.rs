use std::collections::BTreeMap;

use horae::Utc;
use talos::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
    widgets::{Text, traits::Widget},
};

// Head is split into top and bottom
// Top: left, middle, right; Left: Uptime, Middle: FPS and update times, Right: Time
// Bottom: Left: App Name, Right: Help
pub fn head(
    uptime_state: String,
    time_state: String,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) {
    draw_top(uptime_state, time_state, layout, codex, canvas, style_atlas);
    draw_bottom(layout, codex, canvas, style_atlas);
}

fn draw_top(
    uptime_state: String,
    time_state: String,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) {
    let style = style_atlas.get("head").expect("style atlas must have head");
    let mut uptime = Text::new(format!("System Uptime: {}", uptime_state), codex);
    uptime.style(*style);

    let mut middle = Text::new(format!("FPS: {} | Update Time: {}", 0, 0), codex).align_center();
    middle.style(*style);

    let now = Utc::now();
    let mut time = Text::new(
        format!("Local time: {} | UTC time: {}", time_state, now),
        codex,
    );
    time.style(*style);

    uptime.render(
        canvas,
        *layout
            .get("head_top_left")
            .expect("layout must have head_top_left"),
        codex,
    );
    middle.render(
        canvas,
        *layout
            .get("head_top_middle")
            .expect("layout must have head_top_middle"),
        codex,
    );
    time.render(
        canvas,
        *layout
            .get("head_top_right")
            .expect("layout must have head_top_right"),
        codex,
    );
}

const APP_NAME: &str = "Nyx Ystem Xplorer";
const HELP_TEXT: &str = "Welcome to Nyx! One of the system explorers ever made!\nTo exit, press 'q', 'Q', or hit escape";

fn draw_bottom(
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) {
    let style = style_atlas.get("head").expect("style atlas must have head");
    let mut name = Text::new(APP_NAME, codex).align_center();
    name.style(*style);

    let mut help = Text::new(HELP_TEXT, codex);
    help.style(*style);

    name.render(
        canvas,
        *layout
            .get("head_bottom_left")
            .expect("layout must have head_top_middle"),
        codex,
    );
    help.render(
        canvas,
        *layout
            .get("head_bottom_rest")
            .expect("layout must have head_bottom_left"),
        codex,
    );
}
