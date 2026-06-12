use ratatui::{
	Frame, layout::{
		Constraint::Percentage,
		Direction::
		{
			Horizontal,
			Vertical
		},
		Layout
	},
	prelude::Stylize,
	style::{
		Color,
		Style
	},
	text::{
		Line,
		Span
	},
	widgets::{
		Block,
		Gauge
	}
};
use tui_widgets::big_text::{BigText, PixelSize};

use crate::structures::world::World;

pub fn draw_room(world: &mut World, frame: &mut Frame) {
	let layout = Layout::default()
    .direction(Horizontal)
    .constraints(vec![
        Percentage(50),
        Percentage(50),
    ])
    .split(frame.area());

	let left_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Percentage(5),
        Percentage(5),
        Percentage(5),
        Percentage(40),
        Percentage(35),
        Percentage(10)
    ])
    .split(layout[0]);

	let right_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Percentage(10),
        Percentage(20),
        Percentage(40),
        Percentage(30)
    ])
    .split(layout[1]);

	let list_layout = Layout::default()
    .direction(Horizontal)
    .constraints(vec![
        Percentage(50),
        Percentage(50)
    ])
    .split(right_layout[2]);

	let username = BigText::builder()
    .pixel_size(PixelSize::Sextant)
    .style(Style::new().yellow())
    .lines(vec![
        world.player.name.as_str().green().into()
    ])
    .centered()
    .build();

	let hp_bar = Gauge::default()
	.block(Block::bordered().title(format!("{}/{}HP", world.player.hp, world.player.max_hp)))
	.gauge_style(Style::new().black().on_red().italic())
	.percent(world.player.hp as u16);

	let gold = Line::from(vec![
		Span::styled("Gold: ", Style::default().fg(Color::Yellow)),
		Span::styled(world.player.inventory.get("item.gold").unwrap_or(&0).to_string(), Style::default().fg(Color::Yellow)),
	]);
	frame.render_widget(username, left_layout[0]);
	frame.render_widget(hp_bar, left_layout[1]);
	frame.render_widget(gold, left_layout[2]);
}