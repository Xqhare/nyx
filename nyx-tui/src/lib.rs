mod input;
mod head;
mod main_screen;
mod util;
mod error;

use athena::XffValue;
use talos::Talos;
use crate::{error::{ErrorState, render_error}, input::input_handler, main_screen::draw_main_screen, util::{layouter, make_style_atlas}};

/// Entry function for TUI
///
/// Will return `Some(XffValue::Null)` if the program should exit
/// Will return `None` if the program should continue
///
/// Will not error
pub fn draw_state(state: XffValue, talos: &mut Talos) -> Option<XffValue> {
    if state.is_null() {
        // No state = nothing to draw
        let _ = talos.present();
        return None;
    };

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

    if let Some(error) = error_state {
        render_error(&error, codex, canvas, &style_atlas);
    }

    let _ = talos.present();

    None
}

