use std::collections::BTreeMap;

use athena::{Object, XffValue};
use talos::{codex::Codex, layout::Rect, render::{Canvas, Style}, widgets::{Area, Block, Text, stateful::Table, traits::Widget}};


// PS, Docker | Df
pub fn main_body(state: Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style_atlas: &BTreeMap<String, Style>)  {
    let style = style_atlas.get("default").expect("Default Style must exist");
    let header_style = style_atlas.get("highlight").expect("Header style must exist");
    main_left(&state, layout, codex, canvas, *style, *header_style);
    main_right(&state, layout, codex, canvas, *style, *header_style);
}

// PS
fn main_left(state: &Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style: Style, header_style: Style) {
    let ps_state = state.get("ps").expect("Ps state not found").as_object().expect("Ps state must be an object");
    
    left_top(ps_state, layout, codex, canvas, style, header_style);
}

fn left_top(ps_state: &Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style: Style, header_style: Style) {
    let area = layout.get("body_bottom_left_top").expect("Top area must exist");

    let mut block = Block::new().title("Processes", codex, false);
    block.style(style);
    block.render(canvas, *area, codex);

    let area = block.inner(*area);

    let mut rows: Vec<Vec<Text>> = Vec::new();
    rows.push(vec![Text::new("User", codex), Text::new("PID", codex), Text::new("MEM%", codex), Text::new("CPU%", codex), Text::new("Name", codex)]);

    for (_, value) in ps_state.iter() {
        let value = value.as_object().expect("Value must be an object");
        let user = value.get("USER").expect("User must exist").to_string();
        let pid = value.get("PID").expect("Pid must exist").to_string();
        let mem = value.get("%MEM").expect("Mem must exist").to_string();
        let cpu = value.get("%CPU").expect("Cpu must exist").to_string();
        let name = value.get("COMMAND").expect("Name must exist").to_string();
        rows.push(vec![Text::new(user, codex), Text::new(pid, codex), Text::new(mem, codex), Text::new(cpu, codex), Text::new(name, codex)]);
    }

    let mut table = Table::new()
        .with_header_style(header_style)
        .with_header_row(0)
        .with_rows(rows.iter_mut().map(|r| r.iter_mut()));

    table.style(style);
    table.render(canvas, area, codex);
}

fn right_bottom(dockerstate: &XffValue, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style: Style, header_style: Style)  {
    let area = layout.get("body_bottom_right_bottom").expect("area must exist");

    let mut block = Block::new().title("Docker", codex, false);
    block.style(style);
    block.render(canvas, *area, codex);

    let area = block.inner(*area);

    if *dockerstate == XffValue::Null {
        let mut dock_area = Area::new();
        dock_area.style(style);
        dock_area.render(canvas, area, codex);
        let mut dock_text = Text::new("Docker not running", codex).align_center().align_vertically();
        dock_text.style(style);
        dock_text.render(canvas, area, codex);
        return
    } else if dockerstate.is_string() {
        let mut dock_area = Area::new();
        dock_area.style(style);
        dock_area.render(canvas, area, codex);
        let mut dock_text = Text::new(dockerstate.to_string(), codex).align_center().align_vertically();
        dock_text.style(style);
        dock_text.render(canvas, area, codex);
        return
    }
    
    let mut rows: Vec<Vec<Text>> = Vec::new();
    rows.push(vec![Text::new("ID", codex), Text::new("Name", codex), Text::new("State", codex), Text::new("Image", codex)]);

    let docker_state = dockerstate.as_object().expect("Docker state must be an object");
    for (_, value) in docker_state.iter() {
        let value = value.as_object().expect("Value must be an object");
        let id = value.get("ID").expect("Id must exist").to_string();
        let name = value.get("Names").expect("Name must exist").to_string();
        let state = value.get("State").expect("State must exist").to_string();
        let image = value.get("Image").expect("Image must exist").to_string();
        rows.push(vec![Text::new(id, codex), Text::new(name, codex), Text::new(state, codex), Text::new(image, codex)]);
    }

    let mut table = Table::new()
        .with_rows(rows.iter_mut().map(|r| r.iter_mut()))
        .with_header_row(0)
        .with_header_style(header_style);

    table.style(style);
    table.render(canvas, area, codex);
}

// Df
fn main_right(state: &Object, layout: &BTreeMap<String, Rect>, codex: &Codex, canvas: &mut Canvas, style: Style, header_style: Style) {
    let df_state = state.get("df").expect("Df state not found").as_object().expect("Df state must be an object");
    let area = layout.get("body_bottom_right_top").expect("Top area must exist");

    let mut block = Block::new().title("Disk Usage", codex, false);
    block.style(style);
    block.render(canvas, *area, codex);

    let area = block.inner(*area);

    let mut rows: Vec<Vec<Text>> = Vec::new();
    rows.push(vec![Text::new("Filesystem", codex), Text::new("Size", codex), Text::new("Used", codex), Text::new("Avail", codex), Text::new("Use%", codex), Text::new("Mounted on", codex)]);

    for (_, value) in df_state.iter() {
        let value = value.as_object().expect("Value must be an object");
        let filesystem = value.get("Filesystem").expect("Filesystem must exist").to_string();
        let size = value.get("Size").expect("Size must exist").to_string();
        let used = value.get("Used").expect("Used must exist").to_string();
        let avail = value.get("Avail").expect("Avail must exist").to_string();
        let usep = value.get("Use%").expect("Usep must exist").to_string();
        let mounted_on = value.get("Mounted on").expect("Mounted on must exist").to_string();
        rows.push(vec![Text::new(filesystem, codex), Text::new(size, codex), Text::new(used, codex), Text::new(avail, codex), Text::new(usep, codex), Text::new(mounted_on, codex)]);
    }

    let mut table = Table::new()
        .with_header_row(0)
        .with_header_style(header_style)
        .with_rows(rows.iter_mut().map(|r| r.iter_mut()));

    table.style(style);
    table.render(canvas, area, codex);
    let docker_state = if let Some(val) = state.get("docker") {
        val
    } else {
        &XffValue::Null
    };
    right_bottom(docker_state, layout, codex, canvas, style, header_style);
}
