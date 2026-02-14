use std::collections::BTreeMap;

use athena::XffValue;
use talos::{codex::Codex, layout::Rect, render::{Canvas, Style}};

use crate::ErrorState;


pub fn draw_main_screen(state: XffValue, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>) -> Option<ErrorState> {
    
}
