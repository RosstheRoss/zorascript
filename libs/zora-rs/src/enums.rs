use serde::{Serialize, Deserialize};
use specta::Type;
mod utils;

/// The region of the game.
#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub enum Region {
		US,
		EU,
		JP,
}

/// Which game
#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub enum Game {
	Ages,
	Seasons,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub enum Animal {
	Ricky = 0x0b,
	Dimitri = 0x0c,
	Moosh = 0x0d,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub enum Child {
	None,
	Curious,
	Shy,
	Hyperactive
}