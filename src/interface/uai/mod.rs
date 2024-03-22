//! The Universal Ataxx Interface (UAI).
//!
//! A derivative of the UCI protocol for the game Ataxx.
//! Loose specification available at <https://ataxx.org/#comm>.

#[cfg(feature = "game_ataxx")]
pub mod client;
pub mod command;
