use std::fmt;

use serde::{Deserialize, Serialize};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use enums::*;

/// Enumerations for the various choices in the games.
pub mod enums;

/// The structure that holds the save information.
///
/// This is the main struct that is used to hold all the information about the save.
/// This is used to serialize and deserialize the save data to and from a file.
///
/// # Example
///
/// ```rust
/// use zora_rs::Save;
///
/// let save = Save {
///     region: Region::US,
///     game: Game::OcarinaOfTime,
///     animal: Animal::Pony,
///     rupees_given: RupeesGiven::No,
///     sleep_method: SleepMethod::No,
///     child: Child::No,
///     animal_choice: AnimalChoice::No,
///     child_question: ChildQuestion::No,
///     hero: "Link".to_string(),
///     child_name: "Child".to_string(),
///     is_linked: false,
///     is_hero_quest: false,
/// };
/// ```
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Save {
    region: Region,
    game: Game,
    animal: Animal,
    rupees_given: RupeesGiven,
    sleep_method: SleepMethod,
    child: Child,
    animal_choice: AnimalChoice,
    child_question: ChildQuestion,
    hero: String,
    child_name: String,
    is_linked: bool,
    is_hero_quest: bool,
}

impl Save {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

/// The cipher used by the non-Japanese versions of the game.
/// "EN" is a misnomer, as it is used by all non-Japanese versions of the game.
///
/// Stolen (with love) from https://github.com/kabili207/zora-sharp/blob/master/src/Secret.cs#L53
pub static EN_CIPHER: [u8; 48] = [
    21, 35, 46, 4, 13, 63, 26, 16, 58, 47, 30, 32, 15, 62, 54, 55, 9, 41, 59, 49, 2, 22, 61, 56,
    40, 19, 52, 50, 1, 11, 10, 53, 14, 27, 18, 44, 33, 45, 37, 48, 25, 42, 6, 57, 60, 23, 51, 24,
];

/// The cipher used by the Japanese version of the game.
///
/// Stolen (lovingly :)) from https://github.com/kabili207/zora-sharp/blob/master/src/Secret.cs#L45
pub static JP_CIPHER: [u8; 48] = [
    0x31, 0x09, 0x29, 0x3b, 0x18, 0x3c, 0x17, 0x33, 0x35, 0x01, 0x0b, 0x0a, 0x30, 0x21, 0x2d, 0x25,
    0x20, 0x3a, 0x2f, 0x1e, 0x39, 0x19, 0x2a, 0x06, 0x04, 0x15, 0x23, 0x2e, 0x32, 0x28, 0x13, 0x34,
    0x10, 0x0d, 0x3f, 0x1a, 0x37, 0x0f, 0x3e, 0x36, 0x38, 0x02, 0x16, 0x3d, 0x2c, 0x0e, 0x1b, 0x12,
];
