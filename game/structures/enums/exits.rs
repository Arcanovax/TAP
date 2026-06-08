use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Exits {
	North { toward: String },
	South { toward: String },
	East { toward: String },
	West { toward: String },
}