use std::collections::BTreeMap;

use athena::Object;
use talos::{
    LayoutBuilder,
    codex::Codex,
    layout::{Constraint, Direction, Rect},
    render::{Canvas, Style},
    widgets::{Block, Text, traits::Widget},
};

use crate::body::main::main_body;

mod main;

const CRITICAL_UPTIME_THRESHOLD: f64 = 90.0;

pub fn draw_body(
    state: &Object,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style_atlas: &BTreeMap<String, Style>,
) {
    let style = style_atlas
        .get("default")
        .expect("Default Style must exist");
    let ok_style = style_atlas.get("ok").expect("Ok style must exist");
    let error_style = style_atlas.get("error").expect("Error style must exist");
    let body_area = layout.get("body").expect("Top area must exist");
    let mut block = Block::new().with_bg_fill();
    block.style(*style);
    block.render(canvas, *body_area, codex);

    top_bar(
        state,
        layout,
        codex,
        canvas,
        *style,
        *ok_style,
        *error_style,
    );
    main_body(state, layout, codex, canvas, style_atlas);
}

/// Shamash - Ram - Cpu load avg
/// Lasa alltime - Lasa current yearly - Lasa current monthly uptime
fn top_bar(
    state: &Object,
    layout: &BTreeMap<String, Rect>,
    codex: &Codex,
    canvas: &mut Canvas,
    style: Style,
    ok_style: Style,
    error_style: Style,
) {
    let shamash_area = layout
        .get("body_top_top_left")
        .expect("Top area must exist");
    let (shamash_area_left, shamash_area_right) = {
        let tmp = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(50))
            .add_constraint(Constraint::Percentage(50))
            .build()
            .split(*shamash_area);
        debug_assert!(tmp.len() == 2);
        (tmp[0], tmp[1])
    };
    let ram_area = layout
        .get("body_top_top_middle")
        .expect("Top area must exist");
    let cpu_area = layout
        .get("body_top_top_right")
        .expect("Top area must exist");
    let lasa_alltime_area = layout
        .get("body_top_bottom_left")
        .expect("Top area must exist");
    let lasa_current_year_area = layout
        .get("body_top_bottom_middle")
        .expect("Top area must exist");
    let lasa_current_month_area = layout
        .get("body_top_bottom_right")
        .expect("Top area must exist");

    let shamash_state = if let Some(value) = state.get("shamash") {
        value.to_string()
    } else {
        "Shamash state not found".to_string()
    };
    let ram_state = if let Some(val) = state.get("free") {
        let obj = val.as_object().expect("Free must be an object");
        let mem = obj.get("mem").expect("Mem must be a key");
        let mem = mem.as_object().expect("Mem must be an object");
        let total = mem.get("total").expect("Total must be a key");
        let used = mem.get("used").expect("Used must be a key");
        let available = mem.get("available").expect("Available must be a key");
        format!(
            "Ram Total: {} - Used: {} - Available: {}",
            total, used, available
        )
    } else {
        "Ram state not found".to_string()
    };
    let cpu_state = if let Some(val) = state.get("uptime") {
        let obj = val.as_object().expect("Uptime must be an object");
        let load_avg = obj.get("load_avg").expect("Load avg must be a key");
        let (one, five, fifteen) = {
            let averages = load_avg.as_array().expect("Load avg must be an array");
            debug_assert!(averages.len() == 3);
            (
                averages[0].to_string(),
                averages[1].to_string(),
                averages[2].to_string(),
            )
        };
        format!(
            "CPU Load Avg: 1m: {} - 5m: {} - 15m: {}",
            one, five, fifteen
        )
    } else {
        "CPU state not found".to_string()
    };

    let mut all_time_critical = false;
    let mut current_year_critical = false;
    let mut current_month_critical = false;
    let (lasa_alltime, lasa_current_year, lasa_current_month) =
        if let Some(value) = state.get("lasa") {
            let lasa_state = value.as_object().expect("Lasa must be an object");
            let percentage_alltime = lasa_state
                .get("all_time")
                .expect("Alltime must be a key")
                .as_object()
                .unwrap()
                .get("uptime_percent")
                .expect("uptime_percent must be a key")
                .into_number()
                .expect("uptime percent must be printable")
                .into_f64()
                .expect("uptime percent must be printable");
            let year = lasa_state
                .get("current_year")
                .expect("Year must be a key")
                .as_object()
                .unwrap()
                .get("year")
                .expect("year must be a key")
                .into_string()
                .expect("year must be printable");
            let percentage_year = lasa_state
                .get("current_year")
                .expect("Year must be a key")
                .as_object()
                .unwrap()
                .get("uptime_percent")
                .expect("uptime_percent must be a key")
                .into_number()
                .expect("uptime percent must be printable")
                .into_f64()
                .expect("uptime percent must be printable");
            let month = lasa_state
                .get("current_month")
                .expect("Month must be a key")
                .as_object()
                .unwrap()
                .get("month")
                .expect("month must be a key")
                .into_string()
                .expect("month must be printable");
            let percentage_month = lasa_state
                .get("current_month")
                .expect("Month must be a key")
                .as_object()
                .unwrap()
                .get("uptime_percent")
                .expect("uptime_percent must be a key")
                .into_number()
                .expect("uptime percent must be printable")
                .into_f64()
                .expect("uptime percent must be printable");
            if percentage_alltime < CRITICAL_UPTIME_THRESHOLD {
                all_time_critical = true;
            }
            if percentage_year < CRITICAL_UPTIME_THRESHOLD {
                current_year_critical = true;
            }
            if percentage_month < CRITICAL_UPTIME_THRESHOLD {
                current_month_critical = true;
            }
            (
                format!("Lasa uptime monitor - All time: {}%", percentage_alltime),
                format!("Year {}: {}%", year, percentage_year),
                format!("Month {}: {}%", month, percentage_month),
            )
        } else {
            (
                "Lasa not installed".to_string(),
                "".to_string(),
                "".to_string(),
            )
        };

    let mut shamash = Text::new(shamash_state.clone(), codex);
    let mut shamash_text = Text::new("Current Network Status: ", codex);
    let mut ram = Text::new(ram_state, codex);
    let mut cpu = Text::new(cpu_state, codex);
    let mut lasa_alltime_text = Text::new(lasa_alltime, codex);
    let mut lasa_current_year_text = Text::new(lasa_current_year, codex);
    let mut lasa_current_month_text = Text::new(lasa_current_month, codex);

    if all_time_critical {
        lasa_alltime_text.style(error_style);
    } else {
        lasa_alltime_text.style(ok_style);
    }
    if current_year_critical {
        lasa_current_year_text.style(error_style);
    } else {
        lasa_current_year_text.style(ok_style);
    }
    if current_month_critical {
        lasa_current_month_text.style(error_style);
    } else {
        lasa_current_month_text.style(ok_style);
    }

    if shamash_state == "Online" {
        shamash.style(ok_style);
    } else {
        shamash.style(error_style);
    }

    shamash_text.style(style);
    ram.style(style);
    cpu.style(style);

    shamash.render(canvas, shamash_area_right, codex);
    shamash_text.render(canvas, shamash_area_left, codex);
    ram.render(canvas, *ram_area, codex);
    cpu.render(canvas, *cpu_area, codex);
    lasa_alltime_text.render(canvas, *lasa_alltime_area, codex);
    lasa_current_year_text.render(canvas, *lasa_current_year_area, codex);
    lasa_current_month_text.render(canvas, *lasa_current_month_area, codex);
}
