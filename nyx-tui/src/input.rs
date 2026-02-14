use athena::XffValue;
use nyx_backend::error::NyxResult;
use talos::{Talos, input::{Event, KeyCode, KeyEvent}};


pub fn input_handler(talos: &mut Talos) -> NyxResult<Option<XffValue>> {
    if let Some(events) = talos.poll_input()? {
        for event in events {
            match event {
                Event::KeyEvent(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                })
                | Event::KeyEvent(KeyEvent {
                    code: KeyCode::Char('Q'),
                    ..
                })
                | Event::KeyEvent(KeyEvent { code: KeyCode::Esc, .. }) => {return Ok(Some(XffValue::Null))},
                Event::KeyEvent(KeyEvent { code: KeyCode::Char('c'), .. }) => {return Ok(Some(XffValue::from("c")))},
                _ => {},
            }
        }
    }
    Ok(None)
}
