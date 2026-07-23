use std::{fs::OpenOptions, io::Write};

use ratatui::{
    Frame,
    layout::{
        Alignment,
        Constraint::{self, Fill, Length, Percentage},
        Direction::{Horizontal, Vertical},
        Flex,
        HorizontalAlignment::Center,
        Layout, Margin, Rect, Size,
    },
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
};
use tui_widgets::scrollview::ScrollView;

use crate::{
    enums::{channels::Channels, focus::Focus, item_kind::ItemKind},
    structures::world::World,
};

pub fn draw_room_fight(world: &mut World, frame: &mut Frame) {
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
        .constraints(vec![Length(10), Fill(1), Length(1), Fill(1), Length(5)])
        .split(layout[1]);

    let inner_buttons = right_layout[4].inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let buttons_layout = Layout::default()
        .direction(Horizontal)
        .constraints(vec![Fill(1); 3])
        .spacing(1)
        .split(inner_buttons);

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

    // CITY.room DESCRIPTION
    let city_block = Block::bordered()
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
                .title_alignment(Center),
        )
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

    // if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
    // 		let _ = writeln!(file, "ok (State {:#?}) : {:#?}", world.room.fight.target_hp, world.room.fight.target_max_hp);}

    let percent = (world.room.fight.target_hp * 100 / world.room.fight.target_max_hp).min(100);
    hp_bar = Gauge::default()
        .block(
            Block::bordered()
                .title(format!(
                    "{} : {}/{}HP",
                    world.room.fight.target_name,
                    world.room.fight.target_hp,
                    world.room.fight.target_max_hp
                ))
                .title_style(Color::Red)
                .bold()
                .title_alignment(Center),
        )
        .gauge_style(Style::new().fg(gauge_color).on_blue().italic())
        .percent(percent as u16);

    frame.render_widget(hp_bar, right_layout[1]);

    //VS
    let vs = Paragraph::new(Text::from("VERSUS")).centered();
    frame.render_widget(vs, right_layout[2]);

    //FIGHTERS
    let nb_fighters = world.room.fight.fighters.len();
    let mut counter = 0;
    let fighters_on_raw = 2;
    let mut raw_counter = 0;
    if nb_fighters > fighters_on_raw {
        let nb_raws = nb_fighters.div_ceil(fighters_on_raw);
        let fighters_raws =
            Layout::vertical(vec![Constraint::Fill(1); nb_raws]).split(right_layout[3]);
        let mut copy_nb_fighters = nb_fighters.clone();
        while raw_counter < nb_raws {
            let fighters_layout = Layout::horizontal(vec![
                Constraint::Fill(1);
                copy_nb_fighters.min(fighters_on_raw)
            ])
            .split(fighters_raws[raw_counter]);
            for x in 0..copy_nb_fighters.min(fighters_on_raw) {
                let (name, hp) = world.room.fight.fighters.iter().nth(counter).unwrap();
                gauge_color = if *hp < 30 {
                    Color::Red
                } else if *hp < 50 {
                    Color::Yellow
                } else {
                    Color::Green
                };

                hp_bar = Gauge::default()
                    .block(
                        Block::bordered()
                            .title(format!("{} : {}/{}HP", name, *hp, 100))
                            .title_style(Color::Green)
                            .bold()
                            .title_alignment(Center),
                    )
                    .gauge_style(Style::new().fg(gauge_color).on_blue().italic())
                    .percent(*hp as u16);
                frame.render_widget(hp_bar, fighters_layout[x]);
                counter += 1;
            }
            copy_nb_fighters = copy_nb_fighters.saturating_sub(fighters_on_raw);
            raw_counter += 1;
        }
    } else {
        let fighters_layout =
            Layout::horizontal(vec![Constraint::Fill(1); nb_fighters]).split(right_layout[3]);
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
                    Block::bordered()
                        .title(format!("{} : {}/{}HP", name, *hp, 100))
                        .title_style(Color::Green)
                        .bold()
                        .title_alignment(Center),
                )
                .gauge_style(Style::new().fg(gauge_color).on_blue().italic())
                .percent(*hp as u16);

            frame.render_widget(hp_bar, fighters_layout[counter]);
            counter += 1;
        }
    }

    //BUTTONS
    let buttons_block = Block::bordered()
        .title("Allowed actions (on click):")
        .title_alignment(Alignment::Center)
        .title_style(Color::Green)
        .bold();

    frame.render_widget(buttons_block, right_layout[4]);

    let buttons = ["ATTACK", "BAG", "FLEE"];
    for (i, button) in buttons.iter().enumerate() {
        let button_block = Block::new().bg(match *button {
            "ATTACK" => Color::Red,
            "BAG" => Color::LightBlue,
            "FLEE" => Color::Gray,
            _ => Color::White,
        });

        let [text_button] = Layout::vertical([Constraint::Length(1)])
            .flex(Flex::Center)
            .areas(buttons_layout[i]);

        let par = Paragraph::new(Text::from(*button).centered());
        world
            .room
            .fight
            .buttons
            .insert(button.to_string(), buttons_layout[i]);
        frame.render_widget(button_block, buttons_layout[i]);
        frame.render_widget(par, text_button);
    }

    // CHAT
    let messages = match world.chat.channel {
        Channels::GLOBAL => world.chat.global_messages.clone(),
        Channels::ROOM => world.chat.room_messages.clone(),
        Channels::GROUP => world.chat.group_messages.clone(),
    };

    let chat = Block::new()
        .borders(Borders::ALL)
        .border_style(if world.room.focus == Focus::CHAT {
            Color::LightBlue
        } else {
            Color::White
        })
        .title("Chat")
        .title_alignment(Alignment::Center)
        .title_style(Color::Green);

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
            .title("(F2) Send messages here:")
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
            .border_style(if world.room.focus == Focus::COMMAND {
                Color::LightBlue
            } else {
                Color::White
            })
            .title("(F1) You can write your command here:")
            .title_alignment(Alignment::Center)
            .title_style(Color::Green)
            .bold(),
    );

    frame.render_widget(&world.room.text_area, main_layout[1]);

    //BAG
    if world.room.fight.bag {
        world.room.focus = Focus::BAG;
        let bag_area = frame.area().centered(Percentage(30), Length(10));
        let border_bag = Block::new()
            .borders(Borders::ALL)
            .border_style(Color::LightBlue)
            .title("You can use :")
            .title_alignment(Alignment::Center)
            .title_style(Color::Green);

        let mut bag_content: Vec<ListItem> = Vec::new();
        world.room.bag = Vec::new();

        for (item, _) in &world.player.inventory {
            let item_kind = {
                let item_name = world.list_items.get(item).unwrap();
                item_name.kind.clone()
            };
            match item_kind {
                ItemKind::Potion { .. } => {
                    bag_content.push(ListItem::new(
                        Line::from(item.as_str()).alignment(Alignment::Center),
                    ));
                    world.room.bag.push(item.clone());
                }
                _ => {}
            }
        }
        let content_size = bag_content.len();

        if content_size == 0 {
            frame.render_widget(Clear, bag_area);
            frame.render_widget(
                Paragraph::new(
                    Text::from("Nothing to use (Enter to quit)").alignment(Alignment::Center),
                )
                .block(border_bag),
                bag_area,
            );
        } else {
            let bag = List::new(bag_content)
                .block(border_bag)
                .highlight_symbol(">> ");
            frame.render_widget(Clear, bag_area);
            frame.render_stateful_widget(bag, bag_area, &mut world.room.bag_state);
        }
    }
}
