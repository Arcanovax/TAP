use crate::*;
use macroquad::prelude::*;

fn draw_name_input(rect: Rect, game: &mut Game) {
    let input_hovered = rect.contains(game.mouse);
    let input_bg = if input_hovered {
        Color::new(0.2, 0.2, 0.2, 1.0)
    } else {
        Color::new(0.1, 0.1, 0.1, 1.0)
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, input_bg);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, GRAY);

    if is_key_pressed(KeyCode::Backspace) {
        game.player.name.pop();
    }
    if is_key_pressed(KeyCode::Enter) && !game.player.name.is_empty() {
        let msg: String = format!("connect {}\n", game.player.name);
        game.tx_to_serv.try_send(msg).ok();
        game.pending_action = crate::PendingAction::Auth
    }

    while let Some(character) = get_char_pressed() {
        if character.is_ascii_graphic() {
            if game.player.name.len() < 12 {
                game.player.name.push(character);
            }
        }
    }

    let mut display_name = game.player.name.clone();
    if get_time() % 1.0 < 0.5 {
        display_name.push('_');
    }

    if game.player.name.is_empty() && !input_hovered {
        draw_text("Type Name...", rect.x + 10.0, rect.y + 35.0, 30.0, DARKGRAY);
    } else {
        draw_text(&display_name, rect.x + 10.0, rect.y + 35.0, 30.0, YELLOW);
    }
}

pub fn handle_starter(game: &mut Game) {
    let mouse = game.mouse;
    let menu_name: &str = "The answer protocol";

    let title_size = measure_text(menu_name, None, 120, 1.0);
    let title_pos = Vec2::new(
        (screen_width() - title_size.width) / 2.0,
        (screen_height() + title_size.height) / 9.0,
    );
    draw_text(menu_name, title_pos.x, title_pos.y + 60.0, 120.0, WHITE);

    let current_y = title_pos.y + 200.0;

    let input_rect = Rect::new((screen_width()) / 2.0 - 100.0, current_y, 200.0, 50.0);
    draw_name_input(input_rect, game);

    let btn_valid = Rect::new(
        (screen_width()) / 2.0 - 100.0,
        current_y + 70.0,
        200.0,
        40.0,
    );
    if get_button(btn_valid, "Continue", 30, WHITE, mouse) {
        let msg: String = format!("connect {}\n", game.player.name);
        game.tx_to_serv.try_send(msg).ok();
        game.pending_action = crate::PendingAction::Auth
    }

    let rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
    draw_text_bottom(rect, "Made by: mthetcha, relaforg, bfitte", 30, 0.0);
    let state = if game.is_connected {
        "Connected"
    } else {
        "Offline"
    };
    draw_text_bottom(rect, state, 30, screen_width() - 120.0);
}
