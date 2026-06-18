use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Exits {
	North { toward: String },
	South { toward: String },
	East { toward: String },
	West { toward: String },
}

// impl Display for Exits {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "")
// 	}
// }