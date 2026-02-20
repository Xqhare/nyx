use std::collections::BTreeMap;

use athena::Object;
use talos::{codex::Codex, layout::Rect, render::{Canvas, Style}};


// PS, Docker | Df
pub fn main_body(state: Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>)  {}

// PS, Docker
fn main_left(layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>)  {}

// Df
fn main_right(layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>)  {}
