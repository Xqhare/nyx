use std::collections::BTreeMap;

use athena::Object;
use talos::{codex::Codex, layout::Rect, render::{Canvas, Style}};

use crate::{ErrorState, head::draw::head};

mod draw;

pub fn draw_head(state: Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>) -> Option<ErrorState> {
    let uptime = if let Some(value) = state.get("uptime") {
        if let Some(inner_value) = value.into_object() {
            inner_value
        } else {
        return error_uptime();
        }
    } else {
        return error_uptime();
    };
    let uptime_state = if let Some(value) = uptime.get("up") {
        value.to_string()
    } else {
        return error_uptime();
    };
    let time_state = if let Some(value) = uptime.get("time") {
        value.to_string()
    } else {
        return error_uptime();
    };
    if let Some(error) = head(uptime_state, time_state, layout, codex, canvas, style_atlas) {
        return Some(error);
    };
    None
}

pub fn error_uptime() -> Option<ErrorState> {
    Some(ErrorState { message: "Calling `uptime` failed".to_string() })
}

