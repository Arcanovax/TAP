use std::{fs::OpenOptions, io::Write};

use ratatui::{
	Frame, layout::{
		Alignment, Constraint::{self, Fill, Length, Percentage}, Direction::
		{
			Horizontal,
			Vertical
		}, HorizontalAlignment::Center, Layout, Margin, Rect, Size
	}, style::{
		Color, Modifier, Style, Stylize
	}, text::{
		Line,
		Span, Text
	}, widgets::{
		Block, Borders, Gauge, List, ListItem, Paragraph, Wrap
	}
};
use tui_widgets::scrollview::{ScrollView};

use crate::{enums::{channels::Channels, exits::Exits, focus::Focus}, structures::world::World};

pub fn draw_room_fight(world: &mut World, frame: &mut Frame) {

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
        Fill(1),
        Length(6)
    ])
    .split(layout[1]);

	let nb_fighters = world.room.fight.fighters.len();
	let fighters_layout = Layout::horizontal(vec![Constraint::Fill(1); nb_fighters]).split(right_layout[2]);

	// ID
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

	frame.render_widget(id, left_layout[0]);

	// CITY DESCRIPTION
	let city_block = Block::bordered()
		.title(world.room.room_view.name.as_str())
		.title_style(Color::Green)
		.bold()
		.title_alignment(Alignment::Center);

	let inner_city = right_layout[0].inner(Margin { horizontal: 1, vertical: 1 });
	let mut content_width = inner_city.width;

	let mut content_height = textwrap::wrap(world.room.room_view.description.as_str(), content_width as usize).len() as u16;

	let mut scroll_output = ScrollView::new(Size::new(content_width, content_height));

	let city_name: Paragraph = Paragraph::new(Text::from(Text::from(world.room.room_view.description.as_str())))
	.centered()
	.wrap(Wrap { trim: true });

	frame.render_widget(city_block, right_layout[0]);
	scroll_output.render_widget(city_name, Rect::new(0, 0, content_width, content_height));
	frame.render_stateful_widget(scroll_output, inner_city, &mut world.room.descr_scroll_pos);

	// HP
	let mut gauge_color = if world.player.hp < 30 {
		Color::Red
	} else if world.player.hp < 50 {
		Color::Yellow
	} else {
		Color::Green
	};

	let mut hp_bar = Gauge::default()
	.block(
		Block::new()
		.borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
		.title(format!("{}/{}HP", world.player.hp, world.player.max_hp))
	.title_alignment(Center))
	.gauge_style(Style::new().fg(gauge_color).on_blue().italic())
	.percent(world.player.hp as u16);

	frame.render_widget(hp_bar, left_layout[1]);

	// ENEMY
	gauge_color = if world.room.fight.target_hp < 30 {
		Color::Red
	} else if world.room.fight.target_hp < 50 {
		Color::Yellow
	} else {
		Color::Green
	};
	
	let percent = world.room.fight.target_hp * 100 / world.room.fight.target_max_hp;
	// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
	// 			let _ = writeln!(file, "ok (State {:?}) : {:#?} et {}", world.room.fight.target_hp, world.room.fight.target_max_hp, percent);}
	hp_bar = Gauge::default()
	.block(
		Block::new()
		.borders(Borders::ALL)
		.title(format!("{} : {}/{}HP", world.room.fight.target_name, world.room.fight.target_hp, world.room.fight.target_max_hp))
	.title_alignment(Center))
	.gauge_style(Style::new().fg(gauge_color).on_blue().italic())
	.percent(percent as u16);

	frame.render_widget(hp_bar, right_layout[1]);


	//FIGHTERS
	let mut counter = 0;
	for (name, hp) in &world.room.fight.fighters {
		gauge_color = if *hp < 30 {
			Color::Red
		} else if *hp < 50 {
			Color::Yellow
		} else {
			Color::Green
		};

		hp_bar = Gauge::default()
		.block(
			Block::new()
			.borders(Borders::ALL)
			.title(format!("{} : {}/{}HP", name, *hp, 100))
		.title_alignment(Center))
		.gauge_style(Style::new().fg(gauge_color).on_blue().italic())
		.percent(*hp as u16);

		frame.render_widget(hp_bar, fighters_layout[counter]);
		counter += 1;
	}
	// EXITS
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
		.title("You can move to:")
		.title_alignment(Alignment::Center)
		.title_style(Color::Green)
		.bold())
	.style(Color::LightCyan)
	.highlight_style(Modifier::REVERSED);

	frame.render_stateful_widget(exits_list, right_layout[3], &mut world.room.exits_list_state);

	// CHAT
	let messages = match world.chat.channel {
		Channels::GLOBAL => world.chat.global_messages.clone(),
		Channels::ROOM => world.chat.room_messages.clone(),
		Channels::GROUP => world.chat.group_messages.clone(),
	};

	let chat = Block::new()
	.borders(Borders::ALL)
	.title("Chat")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green);

	let inner_chat = chat_space[1].inner(Margin { horizontal: 1, vertical: 1 });

	let mut str_lines: String = "".to_string();

	for message in messages {
		for text_line in message.lines() {
			str_lines += text_line;
			str_lines += "\n";
		}
	}

	content_width = inner_chat.width;
	content_height = textwrap::wrap(&str_lines, content_width as usize).len() as u16;
	scroll_output = ScrollView::new(Size::new(content_width, content_height));

	let chat_content = Paragraph::new(Text::from(str_lines))
	.wrap(Wrap { trim: true });

	frame.render_widget(chat, left_layout[2]);
	scroll_output.render_widget(chat_content, Rect::new(0, 0, content_width, content_height));
	frame.render_stateful_widget(scroll_output, inner_chat, &mut world.room.chat_scroll_pos);


	// CHANNELS
	let global_channel= Paragraph::new("Global")
	.fg(if world.chat.channel == Channels::GLOBAL { Color::LightBlue } else { Color::White })
    .block(
        Block::new()
            .borders(Borders::ALL)
    );

	let room_channel = Paragraph::new("Room")
	.fg(if world.chat.channel == Channels::ROOM { Color::LightBlue } else { Color::White })
	.block(
		Block::new()
		.borders(Borders::ALL)
	);

	let group_channel = Paragraph::new("Group")
	.fg(if world.chat.channel == Channels::GROUP { Color::LightBlue } else { Color::White })
	.block(
		Block::new()
		.borders(Borders::ALL)
	);

	frame.render_widget(global_channel, channels[0]);
	frame.render_widget(room_channel,  channels[1]);
	frame.render_widget(group_channel, channels[2]);

	// OUTPUT
	let inner_output = left_layout[3].inner(Margin{ vertical: 1, horizontal: 2 });

	let output = Block::new()
	.borders(Borders::ALL)
	.title("Output")
	.title_alignment(Alignment::Center)
	.title_style(Color::Green);

	str_lines = "".to_string();

	for message in &world.output {
		for text_line in message.lines() {
			str_lines += text_line;
			str_lines += "\n";
		}
	}

	content_width = inner_output.width;
	content_height = textwrap::wrap(&str_lines, content_width as usize).len() as u16;
	scroll_output = ScrollView::new(Size::new(content_width, content_height));

	let output_content = Paragraph::new(str_lines)
	.wrap(Wrap { trim: true });


	frame.render_widget(output, left_layout[3]);
	scroll_output.render_widget(output_content, Rect::new(0, 0, content_width, content_height));
	frame.render_stateful_widget(scroll_output, inner_output, &mut world.room.output_scroll_pos);

	// COMMAND
	world.room.text_area.set_block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::COMMAND {Color::LightBlue} else {Color::White})
		.title("You can write your command here:")
		.title_alignment(Alignment::Center)
		.title_style(Color::Green)
		.bold());


	frame.render_widget(&world.room.text_area, main_layout[1]);
	
}