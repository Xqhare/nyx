use std::collections::BTreeMap;

use talos::{
    codex::Codex,
    render::{Canvas, Style},
    widgets::{Text, traits::Widget},
};

pub fn render_error(
    error_state: &ErrorState,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) {
    let msg = format!(
        "----------------------- \n Error Encountered \n <><><><><><><><><> \n {} \n <><><><><><><><><> \n Press 'q'to exit | Press 'c' to clear and try to continue \n -----------------------",
        error_state.message
    );
    let mut error_message = Text::new(msg, codex).align_center().align_vertically();
    error_message.style(
        *style_atlas
            .get("error")
            .expect("style atlas must have error"),
    );
    error_message.render(canvas, canvas.size_rect(), codex);
}

pub struct ErrorState {
    pub message: String,
}
