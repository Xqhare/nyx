mod input;
mod main_screen;
use std::collections::BTreeMap;

use athena::XffValue;
use talos::{Talos, layout::Rect, render::{Bright, Colour, Normal, Style}, widgets::{Text, traits::Widget}};

use crate::{input::input_handler, main_screen::draw_main_screen};

/// Entry function for TUI
///
/// Will return `Some(XffValue::None)` if the program should exit
/// Will return `None` if the program should continue
///
/// Will not error
pub fn draw_state(state: XffValue, talos: &mut Talos) -> Option<XffValue> {
    let mut error_state: Option<ErrorState> = None;
    match input_handler(talos) {
        Ok(Some(cmd)) => match cmd {
            XffValue::Null => {
                return Some(XffValue::Null);
            },
            XffValue::String(cmd) => {
                match cmd.as_str() {
                    "q" => {
                        return Some(XffValue::Null);
                    },
                    "c" => {
                        error_state = None;
                    },
                    _ => {
                        error_state = Some(ErrorState {
                            message: format!("Command: {}", cmd),
                        });
                    }
                }
            }
            _ => {
                error_state = Some(ErrorState {
                    message: format!("Command: {}", cmd),
                });
            }
        },
        Err(e) => {
            error_state = Some(ErrorState {
                message: format!("Error: {}", e),
            });
        },
        _ => { 
            //No input 
        },
    }

    let style_atlas = make_style_atlas();

    talos.begin_frame();
    let (canvas, codex) = talos.render_ctx();

    let layout = layouter(canvas.size_rect());

    if let Some(error_st) = draw_main_screen(state, &layout, codex, canvas, &style_atlas) {
        error_state = Some(error_st);
    }

    if let Some(err) = error_state {
        let msg = format!(
            "----------------------- \n Error Encountered \n <><><><><><><><><> \n {} \n <><><><><><><><><> \n Press 'q'to exit | Press 'c' to clear \n -----------------------",
            err.message);
        let mut error_message = Text::new(msg, codex).align_center().align_vertically();
        error_message.style(style_atlas.get("error").expect("style atlas should have error").clone());
        error_message.render(canvas, canvas.size_rect(), codex);
    }

    None
}

fn layouter(rect: Rect) -> BTreeMap<String, Rect> {
    BTreeMap::from([
        ("canvas".to_string(), rect),
    ])
    
}

fn make_style_atlas() -> BTreeMap<String, Style> {
    BTreeMap::from([
        ("default".to_string(), Style::builder()
            .set_fg(Colour::Bright(Bright::Yellow))
            .set_bg(Colour::Normal(Normal::Black))
            .build()
        ),
        ("error".to_string(), Style::builder()
            .set_bg(Colour::Bright(Bright::Red))
            .set_fg(Colour::Bright(Bright::Black))
            .set_bold(true)
            .set_underline(true)
            .build()
        ),
    ])
}

struct ErrorState {
    pub message: String,
}
