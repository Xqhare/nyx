use std::collections::BTreeMap;

use talos::{codex::Codex, layout::Rect, render::{Canvas, Style}};

use crate::ErrorState;


// Head is split into top and bottom
// Top: left, middle, right; Left: Uptime, Middle: Time, Right: None
// Bottom: Left: Help Text, Right: None
pub fn head(uptime_state: String, time_state: String, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>) -> Option<ErrorState> {

}
