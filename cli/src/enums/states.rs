#[derive(PartialEq)]
pub enum States {
	Login,
	ServerWait,
	InGame,
	InFight,
	InDiscuss,
	Respawn
}