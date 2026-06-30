use crate::enums::channels::Channels;


#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    None,
    GroupList,
    Gold,
	Npcs,
	Flee,
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
	ClientStatus,
	Move,
	Who,
	Attack(String),
	Inventory,
	Quest,
	Items
}