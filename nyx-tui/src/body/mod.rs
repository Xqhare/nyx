use std::collections::BTreeMap;

use athena::Object;
use talos::{LayoutBuilder, codex::Codex, layout::{Constraint, Direction, Layout, Rect}, render::{Canvas, Style}, widgets::{Block, Text, traits::Widget}};

use crate::body::main::main_body;

mod main;

pub fn draw_body(state: Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>)  {
    let style = style_atlas.get("default").expect("Default Style must exist");
    let ok_style = style_atlas.get("ok").expect("Ok style must exist");
    let error_style = style_atlas.get("error").expect("Error style must exist");
    let body_area = layout.get("body").expect("Top area must exist");
    let mut block = Block::new().with_bg_fill();
    block.style(style.clone());
    block.render(canvas, *body_area, codex);

    top_bar(state.clone(), layout, codex, canvas, *style, *ok_style, *error_style);
    main_body(state, layout, codex, canvas, style_atlas);
}

/// Shamash - Ram - Cpu load avg
fn top_bar(state: Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style: Style, ok_style: Style, error_style: Style) { {
    let shamash_area = layout.get("body_top_left").expect("Top area must exist");
    let (shamash_area_left, shamash_area_right) = {
        let tmp = LayoutBuilder::new().direction(Direction::Horizontal).add_constraint(Constraint::Percentage(50)).add_constraint(Constraint::Percentage(50)).build().split(*shamash_area);
        debug_assert!(tmp.len() == 2);
        (tmp[0], tmp[1])
    };
    let ram_area = layout.get("body_top_middle").expect("Top area must exist");
    let cpu_area = layout.get("body_top_right").expect("Top area must exist");

    let shamash_state = if let Some(value) = state.get("shamash") {
        value.to_string()
    } else {
        "Shamash state not found".to_string()
    };
    let ram_state = if let Some(val) = state.get("free") {
        let obj = val.into_object().expect("Free must be an object");
        let mem = obj.get("mem").expect("Mem must be a key");
        let mem = mem.into_object().expect("Mem must be an object");
        let total = mem.get("total").expect("Total must be a key");
        let used = mem.get("used").expect("Used must be a key");
        let available = mem.get("available").expect("Available must be a key");
        format!("Ram Total: {} - Used: {} - Available: {}", total, used, available)
    } else {
        "Ram state not found".to_string()
    };
    let cpu_state = if let Some(val) = state.get("uptime") {
        let obj = val.into_object().expect("Uptime must be an object");
        let load_avg = obj.get("load_avg").expect("Load avg must be a key");
        let (one, five, fifteen) = {
            let averages = load_avg.into_array().expect("Load avg must be an array");
            debug_assert!(averages.len() == 3);
            (averages[0].to_string(), averages[1].to_string(), averages[2].to_string())
        };
        format!("CPU Load Avg: 1m: {} - 5m: {} - 15m: {}", one, five, fifteen)
    } else {
        "CPU state not found".to_string()
    };

    let mut shamash = Text::new(shamash_state, codex);
    let mut shamash_text = Text::new("Current Network Status: ", codex);
    let mut ram = Text::new(ram_state, codex);
    let mut cpu = Text::new(cpu_state, codex);

    if shamash_state == "Online" {
        shamash.style(ok_style);
    } else {
        shamash.style(error_style);
    }

    shamash_text.style(style.clone());
    ram.style(style.clone());
    cpu.style(style.clone());

    shamash.render(canvas, shamash_area_right, codex);
    shamash_text.render(canvas, shamash_area_left, codex);
    ram.render(canvas, *ram_area, codex);
    cpu.render(canvas, *cpu_area, codex);
}

