use talos::{
    LayoutBuilder,
    layout::{Constraint, Direction, Rect},
    render::{Bright, Colour, Extended, Normal, Style, TrueColour},
};

use std::collections::BTreeMap;

pub fn layouter(rect: Rect) -> BTreeMap<String, Rect> {
    let canvas = rect;
    let main_layout = LayoutBuilder::new()
        .direction(Direction::Vertical)
        .add_constraint(Constraint::Percentage(25))
        .add_constraint(Constraint::Percentage(75))
        .build()
        .split(canvas);
    debug_assert!(main_layout.len() == 2);
    let head = main_layout[0];
    let (head_top, head_bottom) = {
        let head_layout = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Percentage(50))
            .add_constraint(Constraint::Percentage(50))
            .build()
            .split(head);
        debug_assert!(head_layout.len() == 2);
        (head_layout[0], head_layout[1])
    };
    let (head_top_left, head_top_middle, head_top_right) = {
        let head_top_layout = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(30))
            .add_constraint(Constraint::Percentage(30))
            .add_constraint(Constraint::Percentage(40))
            .build()
            .split(head_top);
        debug_assert!(head_top_layout.len() == 3);
        (head_top_layout[0], head_top_layout[1], head_top_layout[2])
    };
    let (head_bottom_help_text, head_bottom_rest) = {
        let head_bottom_layout = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(50))
            .add_constraint(Constraint::Percentage(50))
            .build()
            .split(head_bottom);
        debug_assert!(head_bottom_layout.len() == 2);
        (head_bottom_layout[0], head_bottom_layout[1])
    };
    let body = main_layout[1];
    let (body_top, body_bottom) = {
        let body_layout = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(10))
            .add_constraint(Constraint::Percentage(90))
            .margin(1)
            .build()
            .split(body);
        debug_assert!(body_layout.len() == 2);
        (body_layout[0], body_layout[1])
    };
    let (body_top_left, body_top_middle, body_top_right) = {
        let body_top_layout = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Min(1))
            .add_constraint(Constraint::Min(1))
            .add_constraint(Constraint::Min(1))
            .build()
            .split(body_top);
        debug_assert!(body_top_layout.len() == 3);
        (body_top_layout[0], body_top_layout[1], body_top_layout[2])
    };
    let (body_bottom_left, body_bottom_right) = {
        let body_bottom_layout = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(70))
            .add_constraint(Constraint::Percentage(30))
            .build()
            .split(body_bottom);
        debug_assert!(body_bottom_layout.len() == 2);
        (body_bottom_layout[0], body_bottom_layout[1])
    };
    let (body_bottom_left_top, body_bottom_left_bottom) = {
        let body_bottom_left_layout = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(70))
            .add_constraint(Constraint::Percentage(30))
            .build()
            .split(body_bottom_left);
        debug_assert!(body_bottom_left_layout.len() == 2);
        (body_bottom_left_layout[0], body_bottom_left_layout[1])
    };
    let (body_bottom_right_top, body_bottom_right_bottom) = {
        let body_bottom_right_layout = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(30))
            .add_constraint(Constraint::Percentage(70))
            .build()
            .split(body_bottom_right);
        debug_assert!(body_bottom_right_layout.len() == 2);
        (body_bottom_right_layout[0], body_bottom_right_layout[1])
    };
    BTreeMap::from([
        ("canvas".to_string(), rect),
        ("main_layout".to_string(), main_layout[0]),
        ("head".to_string(), head),
        ("head_top".to_string(), head_top),
        ("head_top_left".to_string(), head_top_left),
        ("head_top_middle".to_string(), head_top_middle),
        ("head_top_right".to_string(), head_top_right),
        ("head_bottom".to_string(), head_bottom),
        ("head_bottom_left".to_string(), head_bottom_help_text),
        ("head_bottom_rest".to_string(), head_bottom_rest),
        ("body".to_string(), body),
        ("body_top".to_string(), body_top),
        ("body_top_left".to_string(), body_top_left),
        ("body_top_middle".to_string(), body_top_middle),
        ("body_top_right".to_string(), body_top_right),
        ("body_bottom".to_string(), body_bottom),
        ("body_bottom_left".to_string(), body_bottom_left),
        ("body_bottom_left_top".to_string(), body_bottom_left_top),
        (
            "body_bottom_left_bottom".to_string(),
            body_bottom_left_bottom,
        ),
        ("body_bottom_right".to_string(), body_bottom_right),
        ("body_bottom_right_top".to_string(), body_bottom_right_top),
        (
            "body_bottom_right_bottom".to_string(),
            body_bottom_right_bottom,
        ),
    ])
}

pub fn make_style_atlas() -> BTreeMap<String, Style> {
    BTreeMap::from([
        (
            "default".to_string(),
            Style::builder()
                .set_fg(Colour::Bright(Bright::Yellow))
                .set_bg(Colour::Normal(Normal::Black))
                .build(),
        ),
        (
            "head".to_string(),
            Style::builder()
                .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                    200, 200, 200,
                ))))
                .set_bg(Colour::Normal(Normal::Black))
                .build(),
        ),
        (
            "highlight".to_string(),
            Style::builder()
                .set_fg(Colour::Bright(Bright::Green))
                .set_bg(Colour::Bright(Bright::Black))
                .build(),
        ),
        (
            "error".to_string(),
            Style::builder()
                .set_bg(Colour::Bright(Bright::Red))
                .set_fg(Colour::Bright(Bright::Black))
                .set_bold(true)
                .set_underline(true)
                .build(),
        ),
    ])
}
