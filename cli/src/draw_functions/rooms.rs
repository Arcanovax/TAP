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
		Block, Borders, Gauge, List, ListItem, ListState, Paragraph, TitlePosition, Wrap
	}
};

use crate::{enums::exits::Exits, global_functions::{draw_scrollbars::draw_scrollbars, estimate_height::estimate_height}, structures::world::World};

pub fn draw_room(world: &mut World, frame: &mut Frame) {

	// let layout = Layout::default()
    // .direction(Horizontal)
    // .constraints(vec![
    //     Percentage(50),
    //     Percentage(50),
    // ])
	// // .spacing(1)
    // .split(frame.area());

	let main_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Fill(1),
        Length(3),
    ])
    .split(frame.area());

	let layout = Layout::default()
    .direction(Horizontal)
    .constraints(vec![
        Percentage(50),
        Percentage(50),
    ])
    .split(main_layout[0]);

	let left_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Length(5),
        Length(3),
        Fill(1),
        Percentage(35)
    ])
    .split(layout[0]);

	let right_layout = Layout::default()
    .direction(Vertical)
    .constraints(vec![
        Length(10),
        Fill(1),
        Length(6)
    ])
    .split(layout[1]);

	let lists_layout = Layout::default()
    .direction(Horizontal)
    .constraints(vec![
        Percentage(50),
        Percentage(50)
    ])
    .split(right_layout[1]);

	let mut chat_area = left_layout[2];
    let mut output_area = left_layout[3];
    let mut descr_area = right_layout[0];

	draw_scrollbars(frame, &mut chat_area, &mut descr_area, &mut output_area, world);

	let mut lines = vec![
		Line::from(Span::styled(world.player.name.as_str(), Style::default().fg(Color::Green).bold())),
		Line::default(),
	];
	lines.push(Line::from(vec![
		Span::styled("Gold:", Style::default().fg(Color::Yellow)),
		Span::raw(world.player.inventory.get("item.gold").unwrap_or(&0).to_string()),
	]));

	let id: Paragraph = Paragraph::new(Text::from(Text::from(lines)))
	.centered()
	.block(Block::new().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

	let city_name: Paragraph = Paragraph::new(Text::from(Text::from(world.room.description.as_str())))
	.centered()
	.wrap(Wrap { trim: true })
	.block(
		Block::bordered()
		.title(world.room.name.as_str())
		.title_style(Color::Green)
	.title_alignment(Alignment::Center))
	.scroll((world.room.descr_scroll_pos, 0));

	let gauge_color = if world.player.hp < 30 {
		Color::Red
	} else if world.player.hp < 50 {
		Color::Yellow
	} else {
		Color::Green
	};

	let mut npc_list_state = ListState::default();
	let npc_items: Vec<ListItem> = world.room.npc.iter()
		.map(|npc| ListItem::new(Line::from(npc.as_str()).alignment(Alignment::Center)))
		.collect();
	let npc_list = List::new(npc_items)
	.block(Block::bordered()
	.title("You can talk to:")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green)
	.bold());

	let mut items_list_state = ListState::default();
	let item_items: Vec<ListItem> = world.room.items.iter()
		.map(|item| ListItem::new(Line::from(item.as_str()).alignment(Alignment::Center)))
		.collect();
	let items_list = List::new(item_items)
	.block(Block::bordered()
	.title("You can take:")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green)
	.bold());

	let hp_bar = Gauge::default()
	.block(
		Block::new()
		.borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
		.title(format!("{}/{}HP", world.player.hp, world.player.max_hp))
	.title_alignment(Center))
	.gauge_style(Style::new().fg(gauge_color).on_blue().italic())
	.percent(world.player.hp as u16);

	let exits_items: Vec<ListItem> = world.room.exits.iter()
	.map(|exit| {
		match exit {
			Exits::East { toward } => ListItem::new(Line::from("East => ".to_string() + toward).alignment(Alignment::Center)),
			Exits::North { toward } => ListItem::new(Line::from("North => ".to_string() + toward).alignment(Alignment::Center)),
			Exits::West { toward } => ListItem::new(Line::from("West => ".to_string() + toward).alignment(Alignment::Center)),
			Exits::South { toward } => ListItem::new(Line::from("South => ".to_string() + toward).alignment(Alignment::Center))
		}
		})
	.collect();

	let exits_list = List::new(exits_items)
	.block(Block::bordered()
	.title("You can move to:")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green)
	.bold());

	//Just to see them

	let chat = Block::new()
	.borders(Borders::ALL)
	.title("Chat")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green);

	let output = Block::new()
	.borders(Borders::ALL)
	.title("Output")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green);

	let command = Block::new()
	.borders(Borders::ALL)
	.title("You can tap your commands here:")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green);

	frame.render_widget(id, left_layout[0]);
	frame.render_widget(hp_bar, left_layout[1]);
	frame.render_widget(city_name, descr_area);
	frame.render_widget(exits_list, right_layout[2]);
	frame.render_widget(chat, left_layout[2]);
	frame.render_widget(output, left_layout[3]);
	frame.render_widget(command, main_layout[1]);
	frame.render_stateful_widget(npc_list, lists_layout[0], &mut npc_list_state);
	frame.render_stateful_widget(items_list, lists_layout[1], &mut items_list_state);
	
}