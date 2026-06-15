use crate::enums::channels::Channels;


#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    None,
    GroupList,
    Talk,
    Drop,
    Take,
	Auth,
	GroupCreate(String),
	GroupJoin(String),
	GroupLeave(String),
	GroupInvite(String),
	SendChat(Channels, String),
	Look,
	Status,
	Move,
	Who,
	Attack,
	Inventory,
	Quest
}