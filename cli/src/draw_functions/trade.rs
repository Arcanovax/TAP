use std::{collections::VecDeque, fs::OpenOptions, io::Write};

use ratatui::{
    Frame,
    layout::{
        Alignment,
        Constraint::{Fill, Length, Percentage},
        Direction::{Horizontal, Vertical},
        HorizontalAlignment::Center,
        Layout, Margin, Rect, Size,
    },
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
};
use tui_widgets::scrollview::ScrollView;

use crate::{
    enums::{channels::Channels, focus::Focus, npc_kind::NPCKind},
    structures::world::World,
};

pub fn draw_trade(world: &mut World, frame: &mut Frame, inventory: Vec<String>) {
    let main_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Fill(1), Length(3)])
        .split(frame.area());

    let layout = Layout::default()
        .direction(Horizontal)
        .constraints(vec![Percentage(50), Percentage(50)])
        .split(main_layout[0]);

    let left_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Length(5), Length(3), Fill(1), Percentage(35)])
        .split(layout[0]);

    let chat_space = Layout::default()
        .direction(Vertical)
        .margin(1)
        .constraints(vec![Length(3), Fill(1), Length(3)])
        .split(left_layout[2]);

    let channels = Layout::default()
        .direction(Horizontal)
        .constraints(vec![Fill(1), Fill(1), Fill(1)])
        .split(chat_space[0]);

    let right_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Length(10), Fill(1), Fill(1), Length(6)])
        .split(layout[1]);

    let lists_layout = Layout::default()
        .direction(Horizontal)
        .constraints(vec![Percentage(50), Percentage(50)])
        .split(right_layout[1]);

    // ID
    let mut lines = vec![
        Line::from(Span::styled(
            world.player.name.as_str(),
            Style::default().fg(Color::Green).bold(),
        )),
        Line::default(),
    ];

    lines.push(Line::from(vec![
        Span::styled("Gold:", Style::default().fg(Color::Yellow)),
        Span::raw(world.player.gold.to_string()),
    ]));

    let id: Paragraph = Paragraph::new(Text::from(Text::from(lines)))
        .centered()
        .block(Block::new().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

    frame.render_widget(id, left_layout[0]);

    // CITY DESCRIPTION
    let city_block = Block::bordered()
        .border_style(if world.room.focus == Focus::DESCR {
            Color::LightBlue
        } else {
            Color::White
        })
        .title(world.room.room.name.as_str())
        .title_style(Color::Green)
        .bold()
        .title_alignment(Alignment::Center);

    let inner_city = right_layout[0].inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    let mut content_width = inner_city.width;

    let mut content_height =
        textwrap::wrap(world.room.room.description.as_str(), content_width as usize).len() as u16;

    let mut scroll_output = ScrollView::new(Size::new(content_width, content_height));

    let city_name: Paragraph =
        Paragraph::new(Text::from(Text::from(world.room.room.description.as_str())))
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

    // if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
    // 			let _ = writeln!(file, "ok (State {:?}) :", world.player.hp);}
    let hp_bar = Gauge::default()
        .block(
            Block::new()
                .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                .title(format!("{}/{}HP", world.player.hp, world.player.max_hp))
                .title_alignment(Center),
        )
        .gauge_style(Style::new().fg(gauge_color).on_blue().italic())
        .percent(world.player.hp as u16);

    frame.render_widget(hp_bar, left_layout[1]);

    // ITEMS TO SELL
    let mut items_list: Vec<ListItem> = vec![ListItem::new(
        Line::from("Nothing").alignment(Alignment::Center),
    )];

    if world.player.inventory.len() > 0 {
        items_list = Vec::new();
        for (item, quantity) in &world.player.inventory {
            if let Some(item_obj) = world.list_items.get(item) {
                items_list.push(ListItem::new(
                    Line::from(format!(
                        "{} x{} ({} golds)",
                        item_obj.name.clone(),
                        quantity,
                        item_obj.price
                    ))
                    .alignment(Alignment::Center),
                ));
            } else {
                items_list.push(ListItem::new(
                    Line::from(format!("{} x{} (Unknown price)", item.clone(), quantity))
                        .alignment(Alignment::Center),
                ));
            }
        }
    }

    let displayed_list = List::new(items_list)
        .block(
            Block::bordered()
                .border_style(if world.room.focus == Focus::SELL {
                    Color::LightBlue
                } else {
                    Color::White
                })
                .title("YOU CAN SELL:")
                .title_alignment(Alignment::Center)
                .title_style(Color::Green)
                .bold(),
        )
        .style(Color::LightCyan)
        .highlight_style(Modifier::REVERSED);

    frame.render_stateful_widget(
        displayed_list,
        lists_layout[0],
        &mut world.room.sell_list_state,
    );

    // ITEMS TO BUY
    items_list = Vec::new();

    for item in &inventory {
        if let Some(item_obj) = world.list_items.get(item) {
            items_list.push(ListItem::new(
                Line::from(format!(
                    "{} ({} golds)",
                    item_obj.name.clone(),
                    item_obj.price
                ))
                .alignment(Alignment::Center),
            ));
        } else {
            items_list.push(ListItem::new(
                Line::from(format!("{} (Unknown price)", item.clone()))
                    .alignment(Alignment::Center),
            ));
        }
    }

    let buy_list = List::new(items_list)
        .block(
            Block::bordered()
                .border_style(if world.room.focus == Focus::BUY {
                    Color::LightBlue
                } else {
                    Color::White
                })
                .title("YOU CAN BUY:")
                .title_alignment(Alignment::Center)
                .title_style(Color::Green)
                .bold(),
        )
        .style(Color::LightCyan)
        .highlight_style(Modifier::REVERSED);

    frame.render_stateful_widget(buy_list, lists_layout[1], &mut world.room.buy_list_state);

    // EXITS
    let exits_items: Vec<ListItem> = world
        .room
        .room
        .exits
        .iter()
        .map(|(dir, dest)| {
            ListItem::new(Line::from(format!("{dir} => {dest}")).alignment(Alignment::Center))
        })
        .collect();

    let exits_list = List::new(exits_items)
        .block(
            Block::bordered()
                .border_style(if world.room.focus == Focus::EXITS {
                    Color::LightBlue
                } else {
                    Color::White
                })
                .title("(F6) YOU CAN MOVE TO:")
                .title_alignment(Alignment::Center)
                .title_style(Color::Green)
                .bold(),
        )
        .style(Color::LightCyan)
        .highlight_style(Modifier::REVERSED);

    frame.render_stateful_widget(
        exits_list,
        right_layout[3],
        &mut world.room.exits_list_state,
    );

	//DETAILS
    let details_block = Block::bordered()
        .title("ITEM DETAILS")
        .title_alignment(Alignment::Center)
        .title_style(Color::Green)
        .bold();

    let details_content = {
        match world.room.focus {
            Focus::BUY => {
                if let Some(selected_item) = inventory
                    .iter()
                    .nth(world.room.buy_list_state.selected().unwrap_or(0))
                {
                    if let Some(item) = world.list_items.get(selected_item) {
                        Paragraph::new(Text::from(format!("({}) {}",selected_item, item)))
                    } else {
                        Paragraph::new(Text::from("Can't find details about this item."))
                    }
                } else {
                    Paragraph::new(Text::from("Can't find details about this item."))
                }
            }
            Focus::SELL => {
                if let Some((selected_item, ..)) = world
                    .player
                    .inventory
                    .iter()
                    .nth(world.room.sell_list_state.selected().unwrap())
                {
                    if let Some(detailled_item) = world.list_items.get(selected_item) {
                        Paragraph::new(Text::from(format!("({}) {}",selected_item, detailled_item)))
                    } else {
                        Paragraph::new(Text::from("Can't find details about this item."))
                    }
                } else {
                    Paragraph::new(Text::from("Can't find details about this item."))
                }
            }
            _ => Paragraph::new(Text::from("Nothing selected")),
        }
    };
    frame.render_widget(
        details_content
            .block(details_block)
            .wrap(Wrap { trim: true })
            .centered(),
        right_layout[2],
    );

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
                        invites.push_back(format!(
                            "{} invites you. Send 'GROUP JOIN {}' if you want to join.",
                            invitation.sender, invitation.sender
                        ));
                    }
                    invites
                }
            } else {
                world.chat.group_messages.clone()
            }
        }
    };

    let chat = Block::new()
        .borders(Borders::ALL)
        .border_style(if world.room.focus == Focus::CHAT {
            Color::LightBlue
        } else {
            Color::White
        })
        .title("CHAT")
        .title_alignment(Alignment::Center)
        .title_style(Color::Green)
		.bold();

    let inner_chat = chat_space[1].inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let mut str_lines: String = "".to_string();

    let mut formatted_lines: Vec<Line> = Vec::new();

    for message in &messages {
        for text_line in message.lines() {
			str_lines += text_line;
            str_lines += "\n";
			let color = {
				if text_line.starts_with("[me]") {
					Color::Yellow
				} else {
					Color::White
				}
			};
            formatted_lines.push(Line::from(text_line).style(color));
        }
    }

    content_width = inner_chat.width;
    content_height = textwrap::wrap(&str_lines, content_width as usize).len() as u16;
    scroll_output = ScrollView::new(Size::new(content_width, content_height));

    let chat_content = Paragraph::new(Text::from(formatted_lines)).wrap(Wrap { trim: true });

    frame.render_widget(chat, left_layout[2]);
    scroll_output.render_widget(chat_content, Rect::new(0, 0, content_width, content_height));
    frame.render_stateful_widget(scroll_output, inner_chat, &mut world.room.chat_scroll_pos);

    // CHANNELS
    let global_channel = Paragraph::new("(F7) Global")
        .fg(if world.chat.channel == Channels::GLOBAL {
            Color::LightBlue
        } else {
            Color::White
        })
        .block(Block::new().borders(Borders::ALL));

    let room_channel = Paragraph::new("(F8) Room")
        .fg(if world.chat.channel == Channels::ROOM {
            Color::LightBlue
        } else {
            Color::White
        })
        .block(Block::new().borders(Borders::ALL));

    let group_channel = Paragraph::new("(F9) Group")
        .fg(if world.chat.channel == Channels::GROUP {
            Color::LightBlue
        } else {
            Color::White
        })
        .block(Block::new().borders(Borders::ALL));

    frame.render_widget(global_channel, channels[0]);
    frame.render_widget(room_channel, channels[1]);
    frame.render_widget(group_channel, channels[2]);

    // OUTPUT
    let inner_output = left_layout[3].inner(Margin {
        vertical: 1,
        horizontal: 2,
    });

    let output = Block::new()
        .borders(Borders::ALL)
        .border_style(if world.room.focus == Focus::OUTPUT {
            Color::LightBlue
        } else {
            Color::White
        })
        .title("OUTPUT")
        .title_alignment(Alignment::Center)
        .title_style(Color::Green)
		.bold();

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

    let output_content = Paragraph::new(str_lines).wrap(Wrap { trim: true });

    if world.room.focus != Focus::OUTPUT {
        world.room.output_scroll_pos.scroll_to_bottom();
    }
    frame.render_widget(output, left_layout[3]);
    scroll_output.render_widget(
        output_content,
        Rect::new(0, 0, content_width, content_height),
    );
    frame.render_stateful_widget(
        scroll_output,
        inner_output,
        &mut world.room.output_scroll_pos,
    );

    // CHAT TEXT AREA
    world.room.chat_text_area.set_block(
        Block::bordered()
            .title("(F2) SEND MESSAGES HERE:")
            .title_alignment(Alignment::Center)
            .title_style(Color::Green)
            .bold()
            .border_style(if world.room.focus == Focus::CHATTEXT {
                Color::LightBlue
            } else {
                Color::White
            }),
    );

    frame.render_widget(&world.room.chat_text_area, chat_space[2]);

    // COMMAND
    world.room.text_area.set_block(
        Block::bordered()
            .title("(F1) YOU CAN WRITE YOUR COMMANDS HERE:")
            .title_alignment(Alignment::Center)
            .title_style(Color::Green)
            .bold()
            .border_style(if world.room.focus == Focus::COMMAND {
                Color::LightBlue
            } else {
                Color::White
            }),
    );

    frame.render_widget(&world.room.text_area, main_layout[1]);
}
