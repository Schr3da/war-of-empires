use std::default::Default;

use crate::maps::{World, templates};
use crate::theme::ThemeMode;

#[derive(Copy, Clone, Debug)]
pub struct Settings {
    pub mode: ThemeMode,
}

#[derive(Clone)]
pub struct Game {
    pub world: World,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            world: World::new(templates::TEMPLE_MAP, 32),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            mode: ThemeMode::LIGHT,
        }
    }
}

#[derive(Default, Clone)]
pub struct State {
    pub settings: Settings,
    pub game: Game,
}
