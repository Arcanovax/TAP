use ratatui::{layout::Rect, text};
use std::{fs::OpenOptions, io::Write};

pub fn estimate_height(frame: Rect, content: &str, popup:bool) -> u16 {

    let mut window_width = frame.width;
    let mut text_width = window_width.max(1);
    if popup {
        window_width = (frame.width * 50) / 100;
        text_width = window_width.saturating_sub(2).max(1);
    }

    let wrapped_lines = textwrap::wrap(content, text_width as usize);
    // if content.contains("cou") {
    //     if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
    //         let _ = writeln!(file, "ok (State {:?}) : {:#?}", wrapped_lines.len(), content);}

    // }
    if popup {
        wrapped_lines.len() as u16 + 2
    } else {
        wrapped_lines.len() as u16
    }

}