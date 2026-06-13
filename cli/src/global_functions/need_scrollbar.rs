
use ratatui::{Frame, layout::Rect, style::{Color, Style}, widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState}};


pub fn need_scrollbar(
    frame: &mut Frame,
    area: Rect,
    text_height: usize,
    scroll_pos: u16,
    is_focused: bool) -> bool {

  let max_scroll = text_height.saturating_sub(area.height as usize);

    if max_scroll > 0 {
        let color = if is_focused { Color::Blue } else { Color::White };
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .style(Style::default().fg(color));

        let mut scroll_state = ScrollbarState::new(max_scroll)
            .position(scroll_pos as usize);

        // On applique une marge pour ne pas chevaucher les bordures du bloc si tu en as
        frame.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 0 }),
            &mut scroll_state,
        );
        return true;
    }
    
    false
}