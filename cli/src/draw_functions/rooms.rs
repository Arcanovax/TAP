use std::{collections::VecDeque, fs::OpenOptions, io::Write};

use ratatui::{
	Frame, layout::{
		Alignment, Constraint::{Fill, Length, Percentage}, Direction::
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

use crate::{enums::{channels::Channels, focus::Focus, npc_kind::NPCKind}, structures::world::World};

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
		.border_style(if world.room.focus == Focus::DESCR {Color::LightBlue} else {Color::White})
		.title(world.room.room.name.as_str())
		.title_style(Color::Green)
		.bold()
		.title_alignment(Alignment::Center);

	let inner_city = right_layout[0].inner(Margin { horizontal: 1, vertical: 1 });
	let mut content_width = inner_city.width;

	let mut content_height = textwrap::wrap(world.room.room.description.as_str(), content_width as usize).len() as u16;

	let mut scroll_output = ScrollView::new(Size::new(content_width, content_height));

	let city_name: Paragraph = Paragraph::new(Text::from(Text::from(world.room.room.description.as_str())))
	.centered()
	.wrap(Wrap { trim: true });

	frame.render_widget(city_block, right_layout[0]);
	scroll_output.render_widget(city_name, Rect::new(0, 0, content_width, content_height));
	frame.render_stateful_widget(scroll_output, inner_city, &mut world.room.descr_scroll_pos);

	// HP
	let gauge_color = if world.player.hp < 30 {
		Color::Red
	} else if world.player.hp < 50 {
		Color::Yellow
	} else {
		Color::Green
	};

	let hp_bar = Gauge::default()
	.block(
		Block::new()
		.borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
		.title(format!("{}/{}HP", world.player.hp, world.player.max_hp))
	.title_alignment(Center))
	.gauge_style(Style::new().fg(gauge_color).on_blue().italic())
	.percent(world.player.hp as u16);

	frame.render_widget(hp_bar, left_layout[1]);

	// NPCS
	let mut npcs_list: Vec<ListItem> = vec![ListItem::new(Line::from("Nobody").alignment(Alignment::Center))];

	if world.room.npcs.len() > 0 {
		npcs_list = Vec::new();
		for elem in &world.room.npcs {
			if let Some(npc) = world.list_npcs.get(elem){
				npcs_list.push(ListItem::new(
					Line::from(npc.name.clone())
					.alignment(Alignment::Center)
					.style(match npc.kind {NPCKind::Citizen => {Color::White}, NPCKind::Enemy { .. } => {Color::Red}, NPCKind::Merchant { .. } => {Color::Yellow}} )));
			} else {
				npcs_list.push(ListItem::new(
					Line::from(elem.clone())
					.alignment(Alignment::Center)));
			}
		}
	}

	let npc_list = List::new(npcs_list)
	.block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::NPC {Color::LightBlue} else {Color::White})
		.title("You can talk to:")
		.title_alignment(Alignment::Center)
		.title_style(Color::Green)
		.bold())
	.style(Color::LightCyan)
	.highlight_style(Modifier::REVERSED);

	frame.render_stateful_widget(npc_list, lists_layout[0], &mut world.room.npc_list_state);

	// INVENTORY
	let inventory_items: Vec<ListItem> = world.player.inventory.iter()
		.map(|item| ListItem::new(Line::from(format!("{} x{}", item.0.trim_start_matches("item."), item.1)).alignment(Alignment::Center)))
		.collect();

	let items_list = List::new(inventory_items)
	.block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::INVENTORY {Color::LightBlue} else {Color::White})
		.title("In your bag:")
		.title_alignment(Alignment::Center)
		.title_style(Color::Green)
		.bold())
	.style(Color::LightCyan)
	.highlight_style(Modifier::REVERSED);

	frame.render_stateful_widget(items_list, lists_layout[1], &mut world.room.inventory_list_state);

	// EXITS
	let exits_items: Vec<ListItem> = world.room.room.exits.iter()
	.map(|(dir, dest)| {
		ListItem::new(Line::from(format!("{dir} => {dest}")).alignment(Alignment::Center))
		})
	.collect();

	let exits_list = List::new(exits_items)
	.block(
		Block::bordered()
		.border_style(if world.room.focus == Focus::EXITS {Color::LightBlue} else {Color::White})
		.title("You can move to:")
		.title_alignment(Alignment::Center)
		.title_style(Color::Green)
		.bold())
	.style(Color::LightCyan)
	.highlight_style(Modifier::REVERSED);

	frame.render_stateful_widget(exits_list, right_layout[2], &mut world.room.exits_list_state);

	// CHAT
	let messages = match world.chat.channel {
		Channels::GLOBAL => world.chat.global_messages.clone(),
		Channels::ROOM => world.chat.room_messages.clone(),
		Channels::GROUP => {
			if !world.group.in_group {
				if world.group.invitation.len() == 0 {
					let mut my_vec = VecDeque::new();
					my_vec.push_back("Not yet in a group.".to_string());
					my_vec.clone()
				} else {
					let mut invites: VecDeque<String> = VecDeque::new();
					for invitation in &world.group.invitation {
						invites.push_back(format!("{} invites you. Send 'GROUP JOIN {}' if you want to join.", invitation.sender, invitation.sender));
					}
					invites
				}
			} else {
				world.chat.group_messages.clone()
			}
		},
	};

	let chat = Block::new()
	.borders(Borders::ALL)
	.border_style(if world.room.focus == Focus::CHAT {Color::LightBlue} else {Color::White})
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
	.border_style(if world.room.focus == Focus::OUTPUT {Color::LightBlue} else {Color::White})
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
		.title("You can write your command here:")
		.title_alignment(Alignment::Center)
		.title_style(Color::Green)
		.bold()
		.border_style(if world.room.focus == Focus::COMMAND {Color::LightBlue} else {Color::White}));

	// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
	// 			let _ = writeln!(file, "ok (State {:?}) : {:#?}", world.state, world.room.focus);}

	frame.render_widget(&world.room.text_area, main_layout[1]);
	
}