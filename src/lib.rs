#![doc = include_str!("../README.md")]

extern crate log;
extern crate serde;
extern crate serde_json;

pub mod app;
pub mod entity;
pub mod events;
pub mod input;
pub mod math;
pub mod window;

#[macro_use]
pub(crate) mod util;

#[cfg(test)]
pub(crate) mod tests;
