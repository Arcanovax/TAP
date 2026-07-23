use std::time::Instant;

use ratatui::{
    Frame,
    layout::{
        Constraint::{Length, Percentage},
        Direction::Vertical,
        Layout, Margin, Rect,
    },
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Clear, Gauge, Paragraph, Wrap},
};

use crate::{global_functions::estimate_height::estimate_height, structures::world::World};

pub fn escape_handling(
    world: &mut World,
    frame: &mut Frame,
    step: u32,
    cancelled_instant: Option<Instant>,
) {
    let block_popup = Block::bordered().border_style(Style::new().yellow());
    let mut content = Vec::new();
    let mut popup_area: Rect = Rect::new(0, 0, 0, 0);

    match cancelled_instant {
        Some(instant) => {
            popup_area = frame.area().centered(Percentage(50), Length(4));

            let inner_popup = popup_area.inner(Margin {
                horizontal: 1,
                vertical: 1,
            });
            let layouts = Layout::default()
                .direction(Vertical)
                .constraints(vec![Length(1); 2])
                .split(inner_popup);

            let elapsed = instant.elapsed().as_millis();
            let percent = (elapsed * 100 / 2000).min(100);

            let gauge = Gauge::default()
                .gauge_style(Style::new().fg(Color::Yellow))
                .percent(percent as u16);

            content = vec![Line::from("You're so easily influenced!").centered()];

            frame.render_widget(Clear, popup_area);
            frame.render_widget(block_popup.clone(), popup_area);
            frame.render_widget(Paragraph::new(Text::from(content)), layouts[0]);
            frame.render_widget(gauge, layouts[1]);
        }
        None => {
            let mut lines = String::from("");
            match step {
                1 => {
                    content = vec![
                        Line::from("Are you sure you want to quit?"),
                        Line::from("ESCAPE => Confirm"),
                        Line::from("Enter => Cancel"),
                    ];

                    for (i, line) in content.iter().enumerate() {
                        if i > 0 {
                            lines += "\n";
                        }
                        lines.push_str(&line.to_string());
                    }
                    popup_area = frame.area().centered(
                        Percentage(50),
                        Length(estimate_height(frame.area(), &lines)),
                    );
                }
                2 => {
                    content = vec![
                        Line::from("Really? After everything we've been through?"),
                        Line::from("ESCAPE => Confirm"),
                        Line::from("Enter => Cancel"),
                    ];
                    for (i, line) in content.iter().enumerate() {
                        if i > 0 {
                            lines += "\n";
                        }
                        lines.push_str(&line.to_string());
                    }
                    popup_area = frame.area().centered(
                        Percentage(50),
                        Length(estimate_height(frame.area(), &lines)),
                    );
                }
                3 => {
                    content = vec![
                        Line::from("How dare you?"),
                        Line::from("ESCAPE => I dare"),
                        Line::from("Enter => Sorry"),
                    ];
                    for (i, line) in content.iter().enumerate() {
                        if i > 0 {
                            lines += "\n";
                        }
                        lines.push_str(&line.to_string());
                    }
                    popup_area = frame.area().centered(
                        Percentage(50),
                        Length(estimate_height(frame.area(), &lines)),
                    );
                }
                4 => {
                    content = vec![
                        Line::from("If you quit, 4 kittens will DIE."),
                        Line::from("ESCAPE => Ok"),
                        Line::from("Enter => Oh no!"),
                    ];
                    for (i, line) in content.iter().enumerate() {
                        if i > 0 {
                            lines += "\n";
                        }
                        lines.push_str(&line.to_string());
                    }
                    popup_area = frame.area().centered(
                        Percentage(50),
                        Length(estimate_height(frame.area(), &lines)),
                    );
                }
                5 => {
                    content = vec![
                        Line::from(
                            "Before you go, would you like to make a donation to help protect orphaned lettuces?",
                        ),
                        Line::from("ESCAPE => Make a donation and leave"),
                        Line::from("Enter => Sorry, I don't have any money on me."),
                    ];
                    for (i, line) in content.iter().enumerate() {
                        if i > 0 {
                            lines += "\n";
                        }
                        lines.push_str(&line.to_string());
                    }
                    popup_area = frame.area().centered(
                        Percentage(50),
                        Length(estimate_height(frame.area(), &lines)),
                    );
                }
                6 => {
                    content = vec![
                        Line::from("This joke has gone on long enough, don't you think?"),
                        Line::from("ESCAPE => For sure! Please, let me out!"),
                        Line::from("Enter => For sure! But it made me laugh."),
                    ];
                    for (i, line) in content.iter().enumerate() {
                        if i > 0 {
                            lines += "\n";
                        }
                        lines.push_str(&line.to_string());
                    }
                    popup_area = frame.area().centered(
                        Percentage(50),
                        Length(estimate_height(frame.area(), &lines)),
                    );
                }
                7 => world.quit = true,
                _ => {}
            }
            frame.render_widget(Clear, popup_area);
            frame.render_widget(
                Paragraph::new(Text::from(content))
                    .block(block_popup)
                    .centered()
                    .wrap(Wrap { trim: true }),
                popup_area,
            );
        }
    }
}
