use crate::*;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Quest{
    pub npc_id: String,
	pub quest_id: String,
    pub description: String,
    pub reward: String,
    pub goal: Option<Goal>,
}

pub struct Quests{
    pub all: Vec<Quest>,
	pub is_load: bool
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

    let title_quest_pos = vec2(rect_info.x+5.0, rect_info.y+25.0);
    draw_text("Quests:",title_quest_pos.x , title_quest_pos.y, 35.0, WHITE);

    let mut pos_y = title_quest_pos.y + 30.0;

    for quest in game.quests.all.iter(){
        let quest_info = format!("- {}", quest.quest_id);
        draw_text(quest_info, title_quest_pos.x, pos_y, 25.0, YELLOW);
        pos_y += 20.0;
		let quest_goal = match quest.goal.clone() {
			Some(Goal::Collect { item, amount }) => {
				format!("- Collect {} x{}", item, amount)
			}
			Some(Goal::Talk { dialog }) => {
				format!("- Talk to {}", dialog)
			}
			Some(Goal::Retrieve {amount, dialog,item}) => {
				format!("- Retrieve {} {}", amount,item)
			}
			_ => String::new()
		};
		if quest_goal.is_empty(){
			draw_text(&quest.description, title_quest_pos.x + 20.0, pos_y, 20.0, WHITE);
		}
		else{
			draw_text(&quest_goal, title_quest_pos.x + 20.0, pos_y, 20.0, WHITE);
		}
        pos_y += 20.0;

    }
}
