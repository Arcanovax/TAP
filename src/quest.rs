use crate::*;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Quest{
    pub npc_id: String,
	pub quest_id: String,
    pub description: String,
    pub reward: String,
    pub goal: Option<Goal>,
	pub info: Option<QuestInfo>
}


#[derive(Deserialize, Debug, PartialEq)]
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
	Retrieve {amount: i32, dialog:String, item:String}
}




pub fn display_quests(game: &mut Game){
    let rect_info: Rect = get_rect_right(vec2(350.0, 150.0), 0.0);
    draw_rectangle(rect_info.x, rect_info.y, rect_info.w, rect_info.h, Color::new(0.0, 0.0, 0.0, 0.40));
	let quest = &game.quests.all[game.quests.i];
	
	if let Some(info) = &quest.info{
		let rect_name: Rect = get_rect_centered_x(rect_info, vec2(350.0, 25.0), 0.0);
		draw_rectangle(rect_name.x, rect_name.y, rect_name.w, rect_name.h, Color::new(0.0, 0.0, 0.0, 0.80));
		draw_text_center(rect_name,&info.name , 30);
		let btn_left: Rect = Rect::new(rect_name.x, rect_name.y, 25.0, 25.0);
		if get_button(btn_left, "<", 25, WHITE, game.mouse){
			game.quests.i = (game.quests.i + game.quests.all.len() - 1) % game.quests.all.len();
		}
		let btn_left: Rect = Rect::new(rect_name.x +rect_name.w-25.0, rect_name.y, 25.0, 25.0);
		if get_button(btn_left, ">", 25, WHITE, game.mouse){
			game.quests.i = (game.quests.i + game.quests.all.len() + 1) % game.quests.all.len();
		}
	}
    
}

    // for quest in game.quests.all.iter(){
	// 	if let Some(info) = &quest.info{
	// 		 let quest_info = format!("- {}", info.name);
    //     	draw_text(quest_info, title_quest_pos.x, pos_y, 25.0, YELLOW);
	// 		 pos_y += 20.0;
	// 	let quest_goal = match quest.goal.clone() {
	// 		Some(Goal::Collect { item, amount }) => {
	// 			format!("- Collect {} x{}", item, amount)
	// 		}
	// 		Some(Goal::Talk { dialog }) => {
	// 			format!("- Talk to {}", dialog)
	// 		}
	// 		Some(Goal::Retrieve {amount, dialog,item}) => {
	// 			format!("- Retrieve {} {}", amount,item)
	// 		}
	// 		_ => String::new()
	// 	};

	// 	draw_text(&quest_goal, title_quest_pos.x + 20.0, pos_y, 20.0, WHITE);
    //     pos_y += 20.0;
	// 	}
       
       

    

