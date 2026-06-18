use std::{fs::OpenOptions, io::Write};

use ratatui::{
	Frame, layout::{
		Alignment, Constraint::{Fill, Length, Percentage}, Direction::
		{
			Horizontal,
			Vertical
		}, HorizontalAlignment::Center, Layout, Rect, Size, Spacing::Overlap
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
		Block, Borders, Gauge, List, ListItem, Paragraph, StatefulWidget, Wrap
	}
};
use tui_widgets::scrollview::{ScrollView, ScrollViewState};

use crate::{enums::{channels::Channels, exits::Exits, focus::Focus}, global_functions::draw_scrollbars::draw_scrollbars, structures::world::World};

pub fn draw_room(world: &mut World, frame: &mut Frame) {

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

	let chat_space = Layout::default()
	.direction(Vertical)
	.margin(1)
	.constraints(vec![Length(3), Fill(1)])
	.split(left_layout[2]);

	let channels = Layout::default()
	.direction(Horizontal)
	.constraints(vec![Fill(1), Fill(1), Fill(1)])
	.split(chat_space[0]);

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

	let mut chat_area = chat_space[1];
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

	let city_name: Paragraph = Paragraph::new(Text::from(Text::from(world.room.room_view.description.as_str())))
	.centered()
	.wrap(Wrap { trim: true })
	.block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::DESCR {Color::LightBlue} else {Color::White})
		.title(world.room.room_view.name.as_str())
		.title_style(Color::Green)
		.bold()

	.title_alignment(Alignment::Center))
	.scroll((world.room.descr_scroll_pos, 0));

	let gauge_color = if world.player.hp < 30 {
		Color::Red
	} else if world.player.hp < 50 {
		Color::Yellow
	} else {
		Color::Green
	};

	let mut npc_items: Vec<ListItem> = vec![ListItem::new(Line::from("Nobody").alignment(Alignment::Center))];

	if world.room.npcs.len() > 0 {
		npc_items = world.room.npcs.iter()
		.map(|npc| ListItem::new(Line::from(npc.as_str()).alignment(Alignment::Center)))
		.collect();
	}

	let npc_list = List::new(npc_items)
	.block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::NPC {Color::LightBlue} else {Color::White})
	.title("You can talk to:")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green)
	.bold());

	let inventory_items: Vec<ListItem> = world.player.inventory.iter()
		.map(|item| ListItem::new(Line::from(format!("{} x{}", item.0.trim_start_matches("item."), item.1)).alignment(Alignment::Center)))
		.collect();

	let items_list = List::new(inventory_items)
	.block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::INVENTORY {Color::LightBlue} else {Color::White})
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

	let exits_items: Vec<ListItem> = world.room.room_view.exits.iter()
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
	.block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::EXITS {Color::LightBlue} else {Color::White})
	.title("You can move to:")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green)
	.bold());

	//Just to see them

	let messages = match world.chat.channel {
		Channels::GLOBAL => world.chat.global_messages.clone(),
		Channels::ROOM => world.chat.room_messages.clone(),
		Channels::GROUP => world.chat.group_messages.clone(),
	};

	lines = Vec::new();
	for mess in messages {
		lines.push(Line::from(mess));
	}

	let chat = Block::new()
	.borders(Borders::ALL)
	.border_style(if world.room.focus == Focus::CHAT {Color::LightBlue} else {Color::White})
	.title("Chat")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green);

	let chat_content = Paragraph::new(Text::from(lines))
	.wrap(Wrap { trim: true })
	.scroll((world.room.chat_scroll_pos, 0));

	// let borders_channels = Block::new()
	// 	.borders(Borders::ALL)
	// 	.border_style(if world.room.focus == Focus::CHANNELS {Color::LightBlue} else {Color::White});

	let global_channel= Paragraph::new("Global")
	.fg(if world.chat.channel == Channels::GLOBAL { Color::LightBlue } else { Color::White })
    .block(
        Block::new()
            .borders(Borders::ALL)
			// .border_style(if world.chat.channel == Channels::GLOBAL { Color::LightBlue } else { Color::Black })
    );

	let room_channel = Paragraph::new("Room")
	.fg(if world.chat.channel == Channels::ROOM { Color::LightBlue } else { Color::White })
	.block(
		Block::new()
		.borders(Borders::ALL)
		// .border_style(if world.chat.channel == Channels::ROOM { Color::LightBlue } else { Color::White })
	);

	let group_channel = Paragraph::new("Group")
	.fg(if world.chat.channel == Channels::GROUP { Color::LightBlue } else { Color::White })
	.block(
		Block::new()
		.borders(Borders::ALL)
		// .border_style(if world.chat.channel == Channels::GROUP { Color::LightBlue } else { Color::Black })
	);

	let output = Block::new()
	.borders(Borders::ALL)
	.border_style(if world.room.focus == Focus::OUTPUT {Color::LightBlue} else {Color::White})
	.title("Output")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green);

	lines = Vec::new();

	for message in &world.output {
		for text_line in message.lines() {
			lines.push(Line::from(text_line));
		}
	}

	let content_height = lines.len() as u16 + 2;
	let content_width = output_area.width;

	let mut scroll_output = ScrollView::new(Size::new(content_width, content_height));
	let output_content = Paragraph::new(lines)
	.block(output)
	.wrap(Wrap { trim: true });

	world.room.text_area.set_block(
		Block::bordered()
		.title("You can write your command here:")
		.title_alignment(Alignment::Center)
		.title_style(Color::Green)
		.bold()
		.border_style(if world.room.focus == Focus::COMMAND {Color::LightBlue} else {Color::White}));

	// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
	// 			let _ = writeln!(file, "ok (State {:?}) : {:#?}", world.state, world.room.focus);}

	frame.render_widget(id, left_layout[0]);
	frame.render_widget(hp_bar, left_layout[1]);
	frame.render_widget(city_name, descr_area);
	frame.render_widget(exits_list, right_layout[2]);
	frame.render_widget(global_channel, channels[0]);
	frame.render_widget(room_channel,  channels[1]);
	frame.render_widget(group_channel, channels[2]);
	frame.render_widget(chat, left_layout[2]);
	frame.render_widget(chat_content, chat_area);
	// frame.render_widget(output_content, output_area);
	scroll_output.render_widget(output_content, Rect::new(0, 0, content_width, content_height));
	frame.render_stateful_widget(scroll_output, output_area, &mut world.room.output_scroll_pos);
	frame.render_widget(&world.room.text_area, main_layout[1]);
	frame.render_stateful_widget(npc_list, lists_layout[0], &mut world.room.npc_list_state);
	frame.render_stateful_widget(items_list, lists_layout[1], &mut world.room.inventory_list_state);
	
}