use std::collections::BTreeMap;

use athena::XffValue;
use talos::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
};

use crate::{ErrorState, body::draw_body, head::draw_head};

pub fn draw_main_screen(
    gui_run_dur: String,
    state: &XffValue,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) -> Option<ErrorState> {
    let state = if let Some(value) = state.as_object() {
        value
    } else {
        return Some(ErrorState {
            message: "State must be an object - Internal fatal Error".to_string(),
        });
    };
    draw_head(gui_run_dur, state, layout, codex, canvas, style_atlas);
    draw_body(state, layout, codex, canvas, style_atlas);
    None
}
