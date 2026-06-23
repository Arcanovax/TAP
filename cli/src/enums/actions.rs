use crate::enums::channels::Channels;


#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    None,
    GroupList,
	Npcs,
    Talk(String),
    Drop,
    Take,
	Auth,
	GroupCreate(String),
	GroupJoin(String),
	GroupLeave(String),
	GroupInvite(String),
	SendChat(String, String),
	ClientLook,
	Look,
	Status,
	Move,
	Who,
	Attack(String),
	Inventory,
	Quest,
	Items
}