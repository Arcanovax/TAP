mod npc_kind;

pub struct NPC {
	name: String,
	dialogue: Vec<String>,
	kind: NPCKind
}