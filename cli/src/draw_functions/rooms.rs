use lipsum::lipsum;
use ratatui::{
	Frame, layout::{
		Constraint::{Length, Percentage}, Direction::
		{
			Horizontal,
			Vertical
		}, HorizontalAlignment::Center, Layout
	},
	style::{
		Color,
		Style
	},
	text::{
		Line,
		Span, Text
	},
	widgets::{
		Block, Gauge, Paragraph, Scrollbar, ScrollbarOrientation, Wrap
	}
};

use crate::{global_functions::draw_scrollbars::draw_scrollbars, structures::world::World};

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
        Length(2),
        Percentage(5),
        Percentage(40),
        Percentage(35),
        Percentage(10)
    ])
    .split(layout[0]);

	let right_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Length(2),
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

	draw_scrollbars(frame, &left_layout, &right_layout, world);

	let mut lines = vec![
		Line::from(Span::styled(world.player.name.as_str(), Style::default().fg(Color::Green).bold())),
	];
	lines.push(Line::from(vec![
		Span::styled("Gold: ", Style::default().fg(Color::Yellow)),
		Span::raw(world.player.inventory.get("item.gold").unwrap_or(&0).to_string()),
	]));

	let id: Paragraph = Paragraph::new(Text::from(Text::from(lines))).centered();

	lines = vec![
		Line::from(Span::raw("You've just entered in:")),
		Line::from(Span::styled(world.room.name.as_str(), Style::default().fg(Color::Green).bold()))
	];

	let city_name: Paragraph = Paragraph::new(Text::from(Text::from(lines))).centered();
	let city_description: Paragraph = Paragraph::new(world.room.description.as_str())
		.centered()
		.wrap(Wrap { trim: true })
		.scroll((world.room.descr_scroll_pos, 0));

	let gauge_color = if world.player.hp < 30 {
		Color::Red
	} else if world.player.hp < 50 {
		Color::Yellow
	} else {
		Color::Green
	};

	let hp_bar = Gauge::default()
	.block(Block::new().title(format!("{}/{}HP", world.player.hp, world.player.max_hp))
	.title_alignment(Center))
	.gauge_style(Style::new().fg(gauge_color).on_blue().italic())
	.percent(world.player.hp as u16);

	frame.render_widget(id, left_layout[0]);
	frame.render_widget(hp_bar, left_layout[1]);
	frame.render_widget(city_name, right_layout[0]);
	frame.render_widget(city_description, right_layout[1]);
	
}