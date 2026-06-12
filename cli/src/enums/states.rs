#[derive(PartialEq)]
pub enum States {
	Login,
	ServerWait,
	ServerError(String),
	InGame,
	InFight,
	InDiscuss,
	Respawn
}