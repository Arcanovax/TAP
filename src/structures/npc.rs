use crate::structures::enums::npc_kind::NPCKind;

pub struct NPC {
	name: String,
	dialogue: Vec<String>,
	kind: NPCKind
}