use ratatui::{Frame, layout::Rect};

pub fn estimate_height(frame: Rect, content: &str) -> u16 {

    let popup_width = (frame.width * 50) / 100;

    let text_width = popup_width.saturating_sub(2).max(1);

    let wrapped_lines = textwrap::wrap(content, text_width as usize);

    wrapped_lines.len() as u16 + 2

}