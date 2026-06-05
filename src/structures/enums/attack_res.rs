pub enum AttackRes<'a> {
	Hit(String),
	KillTarget(String),
	KillPlayer(String),
	Peace(&'a str),
	NotFound(&'a str),
}