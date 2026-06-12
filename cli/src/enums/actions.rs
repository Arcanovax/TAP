
#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    None,
    GroupList,
	Auth,
	GroupCreate(String),
	GroupJoin(String),
	SendChat(String, String),
	Look,
	Status,
	Move
}