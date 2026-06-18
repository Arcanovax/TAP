use ratatui::{Frame, layout::{Constraint::{Length, Percentage}, Direction::Vertical, Layout}, style::Style, widgets::{Block, Paragraph, Wrap}};
use ratatui::prelude::Stylize;
use tui_widgets::big_text::{BigText, PixelSize};
use crate::{global_functions::estimate_height::estimate_height, structures::{popup::Popup, world::World}};

pub fn login_draw(world: &mut World, frame:&mut Frame) {
    let layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Percentage(50),
        Percentage(25),
        Percentage(25),
    ])
    .split(frame.area());

    let title = BigText::builder()
    .pixel_size(PixelSize::HalfHeight)
    .style(Style::new().yellow())
    .lines(vec![
        "The".red().into(),
        "Answer".blue().into(),
        "Protocol".into()
    ])
    .centered()
    .build();

    let descr = "Be prepared to enter in a new fantastic and amazing 
        world where everything is possible. The first amazing and fantastic thing you 
        can do is to choose your username and press enter. Unbelievable, isn't it?
            Good luck adventurer!";

    if world.message.len() < descr.len() {
        world.counter += 1;
    }

    if world.message.len() < descr.len() && world.counter % 2 == 0 {
        world.message.push(descr.chars().nth(world.message.len()).unwrap());
    }

    let presentation = Paragraph::new(world.message.clone())
    .wrap(Wrap {trim: true})
    .centered();

    let username = Paragraph::new(world.input.as_str())
    .centered()
    .block(
        Block::bordered());

    let username_area = layout[2].centered(
        Percentage(30),
        Length(3)
    );

    frame.render_widget(title, layout[0]);
    frame.render_widget(presentation, layout[1]);
    frame.render_widget(username, username_area);

    if world.click && !world.error{
        let popup = Popup::default()
        .content("You don't need to click anywhere (except to remove this pop-up). Just write your name and press enter.
        You can do it. I believe in you adventurer!")
        .style(Style::new().yellow())
        .border_style(Style::new().red());

        let popup_area = frame.area().centered(
            Percentage(50),
        Length(estimate_height(frame.area(), &popup.content.to_string(), true)));
        frame.render_widget(popup, popup_area);
    }

	if world.error {
        let popup = Popup::default()
        .content(world.message_error.as_str())
        .style(Style::new().yellow())
        .border_style(Style::new().red());

        let popup_area = frame.area().centered(
            Percentage(50),
        Length(estimate_height(frame.area(), &popup.content.to_string(), true)));
        frame.render_widget(popup, popup_area);
    }
}