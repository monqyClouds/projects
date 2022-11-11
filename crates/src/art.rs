//! # Art
//!
//! A library for modelling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create  a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Result<SecondaryColor, &'static str> {
        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Blue => Ok(SecondaryColor::Purple),
                PrimaryColor::Yellow => Ok(SecondaryColor::Orange),
                PrimaryColor::Red => Err("chose different primary colors"),
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Blue => Ok(SecondaryColor::Green),
                PrimaryColor::Red => Ok(SecondaryColor::Orange),
                PrimaryColor::Yellow => Err("chose different primary colors"),
            },
            PrimaryColor::Blue => match c2 {
                PrimaryColor::Red => Ok(SecondaryColor::Purple),
                PrimaryColor::Yellow => Ok(SecondaryColor::Green),
                PrimaryColor::Blue => Err("chose different primary colors"),
            },
        }
    }
}
