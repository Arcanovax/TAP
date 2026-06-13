use ratatui::{layout::Rect, widgets::ScrollbarState};

#[derive(Debug)]
pub struct ScrollBar {
    // active: bool,
    pub area: Rect,
    pub pos: u16,
    pub state: ScrollbarState,
    pub focus: bool
}

// impl ScrollBar {
//     pub fn new() -> Self {
//         ScrollBar {
//             active: false,
//             area: Rect::new(0, 0, 0, 0),
//             pos: 0,
//             focus: false
//         }
//     }
// }