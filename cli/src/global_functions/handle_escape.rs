use crate::{
    enums::{focus::Focus, states::States},
    structures::world::World,
};

pub fn handle_escape(world: &mut World) {
    match &world.state {
        States::Quit(step, prev_state, cancelled_instant) => {
            world.state = States::Quit(*step + 1, Box::new(*prev_state.clone()), *cancelled_instant)
        }
        States::InFight { .. } => {
            if world.room.focus == Focus::BAG {
                world.room.fight.bag = false;
                world.room.focus = Focus::COMMAND;
                world.room.bag = Vec::new();
            }
        }
        States::Trade(..) => {
            world.room.focus = Focus::COMMAND;
            world.state = States::Idle;
        }
        _ => world.state = States::Quit(1, Box::new(world.state.clone()), None),
    }
}
