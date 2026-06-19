use ratatui::{Frame, layout::Constraint::{Length, Percentage}, style::Style};

use crate::{global_functions::estimate_height::estimate_height, structures::{popup::Popup, world::World}};

pub fn pop_popup(world: &mut World, content: &str, frame: &mut Frame) {
	let popup = Popup::default()
        .content(content)
        .style(Style::new().yellow())
        .border_style(Style::new().red());

        let popup_area = frame.area().centered(
            Percentage(50),
        Length(estimate_height(frame.area(), &popup.content.to_string(), true)));
        frame.render_widget(popup, popup_area);
}