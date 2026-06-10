pub enum AttackRes<'a> {
	Hit(String),
	KillTarget(String),
	Peace(&'a str)
}