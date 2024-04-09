use serde::{Deserialize, Serialize};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

/// The region of the game.
///
/// Currently only the US version is supported, though the EU version will probably work too.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Region {
    US,
    // EU,
    // JP,
}

/// Which game
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Game {
    Ages,
    Seasons,
}

/// The choice of animal companion
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Animal {
    Ricky = 0x0b,
    Dimitri = 0x0c,
    Moosh = 0x0d,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RupeesGiven {
    /// One Rupee
    One = 0,
    /// Ten Rupees
    Ten = 2,
    /// Fifty Rupees
    Fifty = 5,
    // 150 Rupees
    OneFifty = 8,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SleepMethod {
    Sing = 0,
    Play = 10,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Child {
    None,
    Curious,
    Shy,
    Hyperactive,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnimalChoice {
    None = 0,
    Ricky = 0x0b,
    Dimitri = 0x0c,
    Moosh = 0x0d,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChildQuestion {
    NoEgg = 0,
    YesChicken = 4,
}