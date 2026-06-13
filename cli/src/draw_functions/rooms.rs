use ratatui::{
	Frame, layout::{
		Alignment, Constraint::{Fill, Length, Percentage}, Direction::
		{
			Horizontal,
			Vertical
		}, HorizontalAlignment::Center, Layout
	},
	style::{
		Color,
		Style, Stylize
	},
	text::{
		Line,
		Span, Text
	},
	widgets::{
		Block, Gauge, List, ListItem, ListState, Paragraph, Wrap
	}
};

use crate::{global_functions::draw_scrollbars::draw_scrollbars, structures::{chat, world::World}};

pub fn draw_room(world: &mut World, frame: &mut Frame) {

	let layout = Layout::default()
    .direction(Horizontal)
    .constraints(vec![
        Percentage(50),
        Percentage(50),
    ])
	.spacing(1)
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

	let lists_layout = Layout::default()
    .direction(Horizontal)
    .constraints(vec![
        Percentage(50),
        Percentage(50)
    ])
    .split(right_layout[2]);

	let npcs_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Length(1),
        Fill(1)
    ])
    .split(lists_layout[0]);

	let items_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Length(1),
        Fill(1)
    ])
    .split(lists_layout[1]);

	let mut chat_area = left_layout[2];
    let mut output_area = left_layout[3];
    let mut descr_area = right_layout[1];

	draw_scrollbars(frame, &mut chat_area, &mut descr_area, &mut output_area, world);

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

	let npcs_title = Line::from(Span::from("You can talk to :")
	.bold())
	.centered()
	.style(Color::Green);

	let items_title = Line::from(Span::from("You can take :")
	.bold())
	.centered()
	.style(Color::Green);

	let mut npc_list_state = ListState::default();
	let npc_items: Vec<ListItem> = world.room.npc.iter()
		.map(|npc| ListItem::new(Line::from(npc.as_str()).alignment(Alignment::Center)))
		.collect();
	let npc_list = List::new(npc_items);

	let mut items_list_state = ListState::default();
	let item_items: Vec<ListItem> = world.room.items.iter()
		.map(|item| ListItem::new(Line::from(item.as_str()).alignment(Alignment::Center)))
		.collect();
	let items_list = List::new(item_items);

	let hp_bar = Gauge::default()
	.block(Block::new().title(format!("{}/{}HP", world.player.hp, world.player.max_hp))
	.title_alignment(Center))
	.gauge_style(Style::new().fg(gauge_color).on_blue().italic())
	.percent(world.player.hp as u16);

	frame.render_widget(id, left_layout[0]);
	frame.render_widget(hp_bar, left_layout[1]);
	frame.render_widget(city_name, right_layout[0]);
	frame.render_widget(city_description, descr_area);
	frame.render_widget(npcs_title, npcs_layout[0]);
	frame.render_widget(items_title, items_layout[0]);
	frame.render_stateful_widget(npc_list, npcs_layout[1], &mut npc_list_state);
	frame.render_stateful_widget(items_list, items_layout[1], &mut items_list_state);
	
}