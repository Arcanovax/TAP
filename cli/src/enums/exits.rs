use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Exits {
	North { toward: String },
	South { toward: String },
	East { toward: String },
	West { toward: String },
}