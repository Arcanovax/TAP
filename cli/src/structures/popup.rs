use derive_setters::Setters;
use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::Style, text::Text, widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap
    }
};

#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    #[setters(into)]
    pub content: Text<'a>,
    border_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .borders(Borders::ALL);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}