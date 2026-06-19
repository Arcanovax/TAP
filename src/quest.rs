use crate::*;

#[derive(Deserialize, Debug)]
pub struct Quest{
    pub npc_id: String,
	pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum Goal {
    Collect { item: String, amount: u32 },
    Talk { dialog: String },
}


pub fn display_quests(game: &mut Game){
    let rect_info: Rect = get_rect_right(vec2(350.0, 150.0), 0.0);
    draw_rectangle(rect_info.x, rect_info.y, rect_info.w, rect_info.h, Color::new(0.0, 0.0, 0.0, 0.40));

    let title_quest_pos = vec2(rect_info.x+5.0, rect_info.y+25.0);
    draw_text("Quests:",title_quest_pos.x , title_quest_pos.y, 35.0, WHITE);

    let mut pos_y = title_quest_pos.y + 30.0;

    for quest in game.quests.iter(){
        let quest_info = format!("- {}: {}", quest.name, quest.description);
        draw_text(quest_info, title_quest_pos.x, pos_y, 25.0, YELLOW);
        pos_y += 20.0;
        for goals in quest.goals.iter(){
            let quest_goal = match goals {
                Goal::Collect { item, amount } => {
                    format!("- Collect {} x{}", item, amount)
                }
                Goal::Talk { dialog } => {
                    format!("- Talk to {}", dialog)
                }
            };
            draw_text(&quest_goal, title_quest_pos.x + 20.0, pos_y, 20.0, WHITE);
            pos_y += 15.0;
        }
        pos_y += 10.0;
       
    }
}