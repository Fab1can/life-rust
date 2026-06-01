use macroquad::rand;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum ElementKind {
    SUGAR,
    SALT,
    WATER,
    ACID,
    BASE,
    OXIDE,
    FLUORIDE,
    CARBONATE,
    NITRATE,
    PHOSPHATE,
}

#[derive(Debug, Clone)]
pub struct Element {
    x: u16,
    y: u16,
    kind: ElementKind
}

impl Element {
    pub fn new(x: u16, y: u16, kind: ElementKind) -> Self {
        Element { x, y, kind }
    }

    pub fn get_x(&self) -> &u16 {
        &self.x
    }

    pub fn get_y(&self) -> &u16 {
        &self.y
    }

    pub fn get_kind(&self) -> ElementKind {
        self.kind
    }
}

pub fn random_element(max_x: u16, max_y: u16) -> Element {
    Element {
        x: rand::gen_range(0, max_x),
        y: rand::gen_range(0, max_y),
        kind: random_element_kind()
    }
}

pub fn random_element_kind() -> ElementKind {
    let list: Vec<ElementKind> = ElementKind::iter().collect();
    list[rand::gen_range(0, list.len())]
}