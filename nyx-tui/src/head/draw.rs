use std::collections::BTreeMap;

use horae::Utc;
use talos::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
    widgets::{Area, Text, traits::Widget},
};

pub struct HeadState {
    pub uptime_state: String,
    pub time_state: String,
    pub update_dur: String,
    pub gui_run_dur: String,
}

// Head is split into top and bottom
// Top: left, middle, right; Left: Uptime, Middle: FPS and update times, Right: Time
// Bottom: Left: App Name, Right: Help
pub fn head(
    state: HeadState,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) {
    let style = style_atlas.get("head").expect("style atlas must have head");
    let mut head_area = Area::new();

    head_area.style(*style);
    head_area.render(
        canvas,
        *layout.get("head").expect("layout must have head"),
        codex,
    );

    draw_top(
        state,
        layout,
        codex,
        canvas,
        style_atlas,
    );
    draw_bottom(layout, codex, canvas, style_atlas);
}

fn calc_fps_ups(gui_run_dur: String, update_dur: String) -> (String, String) {
    // Both strings should be valid usize
    let gui_run_usize = if let Ok(value) = gui_run_dur.parse::<usize>() {
        value
    } else {
        return na();
    };
    let update_usize = if let Ok(value) = update_dur.parse::<usize>() {
        value
    } else {
        return na();
    };

    // Both are durations in microseconds - gui for fps, update for ups

    let fps = 1_000_000 / gui_run_usize;
    let ups = 1_000_000 / update_usize;

    return (fps.to_string(), ups.to_string());

    fn na() -> (String, String) {
        (String::from("N/A"), String::from("N/A"))
    }
}

fn draw_top(
    state: HeadState,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) {
    let style = style_atlas.get("head").expect("style atlas must have head");
    let cell_amount = {
        let rect = canvas.size_rect();
        rect.width.saturating_mul(rect.height)
    };
    let mut uptime = Text::new(
        format!(
            "System Uptime: {} | Cell amount: {}",
            state.uptime_state, cell_amount
        ),
        codex,
    );
    uptime.style(*style);

    let (fps, ups) = calc_fps_ups(state.gui_run_dur, state.update_dur);

    let mut middle = Text::new(
        format!("Frames per second: {} | Updates per second: {}", fps, ups),
        codex,
    )
    .align_center();
    middle.style(*style);

    let now = Utc::now();
    let mut time = Text::new(
        format!("Local time: {} | UTC time: {}", state.time_state, now),
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
