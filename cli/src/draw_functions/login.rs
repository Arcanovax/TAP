use std::{fs::OpenOptions, io::Write};

use ratatui::{Frame, layout::{Constraint::{Length, Percentage}, Direction::Vertical, Layout}, style::Style, widgets::{Block, Paragraph, Wrap}};
use ratatui::prelude::Stylize;
use tui_widgets::big_text::{BigText, PixelSize};
use crate::{draw_functions::popup::pop_popup, structures::{world::World}};

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
	
	let mess_len = world.message.len();

    if mess_len < descr.len() {
        world.counter += 1;
    }

    if mess_len >= descr.len() {
        world.counter = 0;
    } else if world.counter % 2 == 0 {
        world.message.push(descr.chars().nth(mess_len).unwrap());
    }

	// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
	// 			let _ = writeln!(file, "len {:?} counter {:#?}", mess_len, world.counter);}

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
		pop_popup(world, "You don't need to click anywhere (except to remove this pop-up). Just write your name and press enter.
		You can do it. I believe in you adventurer!", frame);
	}

	if world.error {
		pop_popup(world, &world.message_error.clone(), frame);
	}
}