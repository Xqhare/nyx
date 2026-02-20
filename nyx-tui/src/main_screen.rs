use std::collections::BTreeMap;

use athena::XffValue;
use talos::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
};

use crate::{ErrorState, head::draw_head};

pub fn draw_main_screen(
    state: XffValue,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) -> Option<ErrorState> {
    let state = if let Some(value) = state.into_object() {
        value
    } else {
        return Some(ErrorState {
            message: "State must be an object - Internal fatal Error".to_string(),
        });
    };
    draw_head(state, layout, codex, canvas, style_atlas);
    None
}
