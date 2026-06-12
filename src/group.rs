
use crate::*;
const RECT_MENU: Vec2 = vec2(250.0, 300.0);
const RECT_ICON: Vec2 = vec2(40.0, 40.0);



pub struct Invitation {
	pub sender: String,
	pub group_name: String
}


pub struct Group {
	pub is_active: bool,
	pub in_group: bool,
	pub typed: String,
	pub chat_is_active: bool,
	pub invitation: Option<Invitation>,
	pub name: String,
	pub list: String,
	pub invite_state: (String,Color)
}

impl Group {
	pub fn new() -> Self {
		Self {
			is_active: false,
			in_group: false,
			typed: String::new(),
			chat_is_active: false,
			invitation: None,
			name: String::new(),
			list: String::new(),
			invite_state: (String::new(),Color::default())
			}
	}
}

fn send_group_invite(game: &mut Game){
	let rq: String = format!("GROUP INVITE {}\n",game.group.typed);
	game.tx_to_serv.try_send(rq).ok();
	game.pending_action = PendingAction::GroupInvite(game.group.typed.clone());
	game.group.typed = String::new();
}



pub fn draw_group(game: &mut Game){
	let mouse = mouse_position();
	let rect: Rect = get_rect_right(RECT_MENU, screen_height());
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));



	if !game.group.in_group {
		let btn: Rect = Rect::new(rect.x+(rect.w/2.0-(100.0)),rect.y+10.0, 200.0, 40.0);
		if get_button(btn, "Create group", 30, mouse) {
			game.tx_to_serv.try_send("GROUP CREATE\n".to_string()).ok();
			game.pending_action = PendingAction::GroupCreate(game.player.name.clone());
		}


		if let Some(invitation) = game.group.invitation.as_ref(){
			let invit_rect: Rect = Rect::new(rect.x ,rect.y+ 60.0, rect.w, 60.0);
			let text: String = format!("{} invate you in {}",invitation.sender, invitation.group_name);
			draw_text_center(invit_rect, &text, 18);
			let btn_weight = 100.0;
			let space: f32 = 15.0;
			let join_btn: Rect = Rect::new(invit_rect.x + space,invit_rect.y + 50.0, btn_weight, 20.0);
			let deny_btn: Rect = Rect::new(invit_rect.x + invit_rect.w - btn_weight - space ,invit_rect.y + 50.0, btn_weight, 20.0);
			if get_button(join_btn, "Join", 20, mouse) {
				let rq: String = format!("GROUP JOIN {}\n",invitation.sender);
				game.tx_to_serv.try_send(rq).ok();
				game.pending_action = PendingAction::GroupJoin(invitation.sender.clone());
			}
			if get_button(deny_btn, "Deny", 20, mouse) {
				game.group.invitation = None;
			}
		}
	}
	else {
		let text: String = format!("{}'s Group", game.group.name);
		draw_text_center_top(rect, &text, 30, 20.0);

		if game.group.list.is_empty(){
			game.tx_to_serv.try_send("GROUP LIST\n".to_string()).ok();
			game.pending_action = PendingAction::GroupList;
		}
		else {
			match serde_json::from_str::<Vec<String>>(&game.group.list.to_string()) {
				Ok(players) => {
					for (i, player) in players.iter().enumerate() {
						draw_text(player,rect.x,rect.y + 60.0 + i as f32 * 35.0,40.0,WHITE,);
					}
				}
				Err(e) => {println!("{}", e)}
				}
		}
		let input_rect: Rect = Rect::new(rect.x+(rect.w/2.0-(120.0)),rect.y + rect.h - 100.0, 150.0, 35.0);

		draw_text("Invite a player:", input_rect.x, input_rect.y-2.5, 20.0, WHITE);
		let input_hovered: bool = input_text(input_rect, &mut game.group.typed,game.focus == InputFocus::GroupMenu,  mouse);
		if input_hovered && is_mouse_button_pressed(MouseButton::Left)  {
			game.focus = InputFocus::GroupMenu;
			game.group.chat_is_active = true;
		}
		else if !input_hovered && is_mouse_button_pressed(MouseButton::Left) {
			game.group.chat_is_active = false;
			game.focus = InputFocus::Game;
		}
		if is_key_pressed(KeyCode::Enter) {
			send_group_invite(game);
			game.focus = InputFocus::Game;
		}
		let invite_btn: Rect = Rect::new(input_rect.x + input_rect.w,input_rect.y, 70.0, 35.0);
		if get_button(invite_btn, "Invite", 25, mouse) && !game.group.typed.is_empty(){
			send_group_invite(game);
		}
		draw_text(game.group.invite_state.0.clone(), input_rect.x, input_rect.y+input_rect.h+15.0, 25.0, game.group.invite_state.1);

	}
}

pub fn draw_icon(game: &mut Game, mouse: (f32, f32)){
	let mut icon: Rect = get_rect_bottom(RECT_ICON, screen_width());
	icon.x -= 10.0;
	icon.y -= 10.0;
	let hovered = icon.contains(Vec2::new(mouse.0, mouse.1));
	let bg = if hovered { Color::new(0.3, 0.3, 0.3, 0.75) }
	else { Color::new(0.10, 0.10, 0.10, 0.75) };
	draw_rectangle(icon.x, icon.y, icon.w, icon.h, bg);
	draw_text("GR", icon.x+ 6.0, icon.y + 22.5, 30.0, WHITE);
	if hovered && is_mouse_button_pressed(MouseButton::Left) {
		game.group.is_active = true
	}
}

pub fn handle_group(game: &mut Game) {
	let mouse = mouse_position();
	if !game.group.is_active {
		draw_icon(game, mouse);
		if is_key_pressed(KeyCode::F) && game.focus == InputFocus::Game {
			game.group.is_active = true;
			game.tx_to_serv.try_send("GROUP LIST\n".to_string()).ok();
			game.pending_action = PendingAction::GroupList;
		}
	}
	else{

		if is_key_pressed(KeyCode::F) && game.group.chat_is_active == false{
			game.group.is_active = false;
			game.focus = InputFocus::Game;
		}
		draw_group(game);
	}
}
