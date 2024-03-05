use std::fmt::Display;

use bevy::{
    ecs::{component::Component, system::Resource},
    render::color::Color,
};

#[derive(Component, Clone)]
pub(crate) enum Race {
    Red,
    Green,
    Blue,
    Yellow,
    Violet,
}

impl Race {
    pub fn all() -> [Self; 5] {
        [
            Self::Red,
            Self::Green,
            Self::Blue,
            Self::Yellow,
            Self::Violet,
        ]
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Race::Red => "red",
            Race::Green => "green",
            Race::Blue => "blue",
            Race::Yellow => "yellow",
            Race::Violet => "violet",
        };
        write!(f, "{str}")
    }
}

impl From<&Race> for usize {
    fn from(value: &Race) -> Self {
        match value {
            Race::Red => 0,
            Race::Green => 1,
            Race::Blue => 2,
            Race::Yellow => 3,
            Race::Violet => 4,
        }
    }
}

impl From<Race> for Color {
    fn from(value: Race) -> Self {
        match value {
            Race::Red => Color::CRIMSON,
            Race::Green => Color::DARK_GREEN,
            Race::Blue => Color::BLUE,
            Race::Yellow => Color::YELLOW,
            Race::Violet => Color::VIOLET,
        }
    }
}

#[derive(Clone, Copy, Resource)]
pub(crate) struct Forces {
    forces: [[f32; 5]; 5],
}

impl Forces {
    pub fn new() -> Self {
        Self {
            forces: [
                [0.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

    pub fn set_force(&mut self, from: &Race, to: &Race, attraction: f32) {
        self.forces[usize::from(from)][usize::from(to)] = attraction;
    }

    pub fn get_force(self, from: &Race, to: &Race) -> f32 {
        (6.6743 * 10.0) * self.forces[usize::from(from)][usize::from(to)]
    }
}

impl Display for Forces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.forces)
    }
}

#[derive(Component)]
pub(crate) struct Velocity2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub(crate) struct Position2D {
    pub x: f32,
    pub y: f32,
}
