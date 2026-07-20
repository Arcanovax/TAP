use macroquad::text;

use crate::*;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Quest{
    pub npc_id: String,
	pub quest_id: String,
    pub description: String,
    pub reward: String,
    pub goal: Option<Goal>,
	pub info: Option<QuestInfo>,
	pub progress: String
}


#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct QuestInfo{
	pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
}

pub struct Quests{
    pub all: Vec<Quest>,
	pub is_load: bool,
	pub i: usize
}


#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum Goal {
    Collect {item: String, amount: u32 },
    Talk { dialog: String },
	Retrieve {amount: i32, dialog:String, item:String},
	Answer {answer: String, room: String}
}




pub fn display_quests(game: &mut Game){
    let rect_info: Rect = get_rect_right(vec2(350.0, 150.0), 0.0);
    draw_rectangle(rect_info.x, rect_info.y, rect_info.w, rect_info.h, Color::new(0.0, 0.0, 0.0, 0.40));
	if game.quests.i >= game.quests.all.len() {
		game.quests.i = 0;
	}
	let quest = &game.quests.all[game.quests.i];

	if let Some(info) = &quest.info{
		let rect_name: Rect = get_rect_centered_x(rect_info, vec2(350.0, 25.0), 0.0);
		draw_rectangle(rect_name.x, rect_name.y, rect_name.w, rect_name.h, Color::new(0.0, 0.0, 0.0, 0.80));
		draw_text_center_top(rect_name,&info.name , 30, 20.0);
		let btn_left: Rect = Rect::new(rect_name.x, rect_name.y, 25.0, 25.0);
		if get_button(btn_left, "<", 25, WHITE, game.mouse){
			game.quests.i = (game.quests.i + game.quests.all.len() - 1) % game.quests.all.len();
		}
		let btn_left: Rect = Rect::new(rect_name.x +rect_name.w-25.0, rect_name.y, 25.0, 25.0);
		if get_button(btn_left, ">", 25, WHITE, game.mouse){
			game.quests.i = (game.quests.i + game.quests.all.len() + 1) % game.quests.all.len();
		}
		let goal_coord = vec2(rect_info.x + 20.0, 65.0);
		let current_goal = if let Some(goal) = &quest.goal {
			goal.clone()
		} else {
			info.goals[0].clone()
		};
		match current_goal {
			Goal::Collect { item, amount } => {
				draw_text(format!("Collect:   x{}", amount),goal_coord.x, goal_coord.y , 35.0, WHITE);
				let item_rect = Rect::new(goal_coord.x + 135.0, goal_coord.y - 20.0, 15.0, 15.0);
				if let Some(item) = game.loaded_items.get(&item){
					draw_item_center(item_rect, item);
				}
			}
			Goal::Talk { dialog } => {
				if let Some(npc) = game.loaded_npcs.get(&dialog.split('.')
																	.take(2)
																	.collect::<Vec<&str>>()
																	.join(".")){
					draw_text(format!("Talk to {}", npc.name),goal_coord.x, goal_coord.y , 35.0, WHITE);
				}
				else{
					draw_text(dialog, goal_coord.x, goal_coord.y , 25.0, WHITE);
				}

			}
			Goal::Retrieve {amount, dialog,item} => {
				let npc_id = dialog.split('.').take(2).collect::<Vec<_>>().join(".");
				if let Some(npc) = game.loaded_npcs.get(&npc_id){
					draw_text(format!("Retrive:   x{} to {}", amount, npc.name),goal_coord.x, goal_coord.y , 25.0, WHITE);
					let item_rect = Rect::new(goal_coord.x + 105.0, goal_coord.y - 20.0, 2.5, 2.5);
					if let Some(item) = game.loaded_items.get(&item){
						draw_item_center(item_rect, item);
					}
				}
			}
			Goal::Answer { answer:_, room } => {
				draw_text("Answer with /answer [word]",goal_coord.x, goal_coord.y - 10.0, 25.0, WHITE);
				if let Some((_, room_name)) = room.rsplit_once('.'){
					draw_text(format!("in {}", room_name),goal_coord.x, goal_coord.y + 10.0, 25.0, WHITE);
				}

			}
		};

		draw_text(quest.progress.clone(), rect_info.x + 20.0, 100.0, 30.0, WHITE);
		draw_text("Reward:", rect_info.x + 20.0, 130.0, 35.0, WHITE);
		let reward_rect = Rect::new(rect_info.x + 145.0, 110.0, 15.0, 15.0);
		if let Some(reward) = game.loaded_items.get(&info.reward){
			draw_item_center(reward_rect, reward);
		}
		let btn_descr = Rect::new(rect_info.x + rect_info.w - 65.0, 100.0, 50.0, 40.0);
		draw_rectangle(btn_descr.x, btn_descr.y, btn_descr.w, btn_descr.h, Color::new(0.0, 0.0, 0.0, 0.80));
		draw_text_center(btn_descr, "Info", 25);
		if btn_descr.contains(game.mouse){
			draw_centered_descr(&info.description);
		}
	}

}






