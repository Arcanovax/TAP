use ratatui::{Frame, layout::Constraint::{Length, Percentage}, style::Style};

use crate::{global_functions::estimate_height::estimate_height, structures::popup::Popup};

pub fn draw_wait(frame: &mut Frame) {
    let wait_popup: Popup = Popup::default()
    .border_style(Style::new().green())
    .content("Waiting for the server.")
    .style(Style::new().yellow());

     let popup_area = frame.area().centered(
            Percentage(50),
        Length(estimate_height(frame.area(), &wait_popup.content.to_string(), true)));
    frame.render_widget(wait_popup, popup_area);
}