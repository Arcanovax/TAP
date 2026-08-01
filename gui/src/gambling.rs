use crate::*;

const GAME_SIZE: Vec2 = vec2(100.0, 125.0);
pub struct Games {
    pub dice: Dice,
    pub slot: Slot,
}

pub struct Dice {
    pub picks: [i32; 10],
    pub result: Slot,
}

pub struct Slot {
    pub result: String,
    pub color: Color,
}

impl Games {
    pub fn new() -> Self {
        Self {
            dice: Dice {
                picks: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                result: Slot {
                    result: "-----".to_string(),
                    color: WHITE,
                },
            },
            slot: Slot {
                result: "-----".to_string(),
                color: WHITE,
            },
        }
    }
}

fn get_games_rect(pos_x: f32, pos_y: f32) -> Rect {
    return Rect::new(
        pos_x - (GAME_SIZE.x / 2.0),
        pos_y - (GAME_SIZE.y / 2.0),
        GAME_SIZE.x,
        GAME_SIZE.y,
    );
}

fn draw_slotmachine(game: &mut Game) {
    let rect = get_games_rect(180.0, 275.0);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.0, 0.0, 0.0, 0.85),
    );
    draw_text_center_top(rect, "Slot Machine", 17, 12.5);
    let start_button = get_rect_centered_x(rect, vec2(75.0, 35.0), 25.0);
    if get_button(start_button, "Start", 20, WHITE, game.mouse) {
        game.tx_to_serv.try_send("SLOT_MACHINE \n".to_string()).ok();
        game.pending_action = PendingAction::SlotMachine;
    }
    draw_text_center_top_c(
        rect,
        &game.gambling.slot.result.to_string(),
        30,
        100.0,
        game.gambling.slot.color,
    );
}

fn draw_dice(game: &mut Game) {
    let rect = Rect::new(230.0, 390.0, 150.0, 250.0);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.0, 0.0, 0.0, 0.85),
    );
    draw_text_center_top(rect, "Dice", 20, 15.0);
    let start_button = get_rect_centered_x(rect, vec2(75.0, 35.0), 25.0);
    for i in 0..10 {
        let col = i % 5;
        let row = i / 5;

        let x = 5.0 + rect.x + col as f32 * 30.0;
        let y = rect.y + 115.0 + row as f32 * 70.0;

        let rect_bet = Rect::new(x, y, 20.0, 60.0);
        get_input_bet(game, rect_bet, i);
    }
    if get_button(start_button, "Start", 20, WHITE, game.mouse) {
        let mut cmd = String::from("DICES");
        for i in 0..10 {
            let count = game.gambling.dice.picks[i];
            if count > 0 {
                cmd.push(' ');
                cmd.push_str(&(count).to_string());
            }
        }

        cmd.push('\n');
        game.tx_to_serv.try_send(cmd).ok();
        game.pending_action = PendingAction::Dices;
    }
    draw_text_center_top_c(
        rect,
        &game.gambling.dice.result.result,
        30,
        90.0,
        game.gambling.dice.result.color,
    );
}

fn get_input_bet(game: &mut Game, rect: Rect, i: usize) -> i32 {
    let btn_add = Rect::new(rect.x, rect.y, 20.0, 20.0);
    if get_button(btn_add, "+", 20, WHITE, game.mouse) {
        game.gambling.dice.picks[i] = (game.gambling.dice.picks[i] + 1).clamp(0, 10);
    }
    let count_rect = Rect::new(rect.x, rect.y + 20.0, 20.0, 20.0);
    draw_text_center(count_rect, &game.gambling.dice.picks[i].to_string(), 25);
    let btn_add = Rect::new(rect.x, rect.y + 40.0, 20.0, 20.0);
    if get_button(btn_add, "-", 20, WHITE, game.mouse) {
        game.gambling.dice.picks[i] = (game.gambling.dice.picks[i] - 1).clamp(0, 10);
    }
    return 1;
}

pub fn handle_games(game: &mut Game) {
    if let Some(map) = game.map_data.clone() {
        let slot_place = vec2(45.0, 95.0);
        let range = 20.0;
        let is_next: bool = (game.player.x - slot_place.x).abs() <= range
            && (game.player.y - slot_place.y).abs() <= range;

        if map.room.id == "room.game_room" && is_next {
            draw_slotmachine(game);
        }

        let dice_place = vec2(85.0, 155.0);
        let range = 25.0;
        let is_next: bool = (game.player.x - dice_place.x).abs() <= range
            && (game.player.y - dice_place.y).abs() <= range;
        if map.room.id == "room.game_room" && is_next {
            draw_dice(game);
        }
    }
}

pub fn parse_dices_data(answer: &str) -> Option<DicesData> {
    let mut gold = None;
    let mut draw = None;

    for part in answer.split(' ') {
        if let Some(value) = part.strip_prefix("gold=") {
            gold = value.parse::<i32>().ok();
        } else if let Some(value) = part.strip_prefix("draw=") {
            draw = Some(value.to_string());
        }
    }

    Some(DicesData {
        gold: gold?,
        _draw: draw?,
    })
}
