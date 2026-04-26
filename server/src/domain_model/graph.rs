use std::{num::NonZeroUsize, ops::{Range, RangeFrom}};

pub enum Style {
    Regular,
    Dotted,
    Dashed,
    Bold
}

pub enum Arrow {
    None,
    Arrow
}

type EntityIndex = usize;

pub struct Relation {
    pub text: String,
    pub weight: NonZeroUsize,
    pub entity_1: EntityIndex,
    pub entity_2: EntityIndex,
    pub arrow_1: Arrow,
    pub arrow_2: Arrow,
    pub mult_1: Multiplicity,
    pub mult_2: Multiplicity
}

pub enum Multiplicity {
    Range(Range<usize>),
    Number(usize),
    RangeFrom(RangeFrom<usize>)
}

type ColorHexValue = u32;

pub struct Entity {
    pub name: String,
    pub color: ColorHexValue,
    pub style: Style
}

#[derive(Default)]
pub struct Graph {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>, // a relation is None if the parser could not recognize the statement (should be the same length as raw)
    pub raw: String, // the raw input that makes up the graph
}

impl Graph {
    pub fn new() -> Self {
        Self {..Default::default()}
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn dummy_entity() -> Entity {
        Entity {
            name: "".to_string(),
            color: 0xff00ffff,
            style: Style::Regular
        }
    }

    pub fn dummy_relation(entity_1: usize, entity_2: usize) -> Relation {
        Relation {
            text: "".to_string(),
            weight: NonZeroUsize::new(1).unwrap(),
            entity_1,
            entity_2,
            arrow_1: Arrow::None,
            arrow_2: Arrow::Arrow,
            mult_1: Multiplicity::Number(1),
            mult_2: Multiplicity::RangeFrom(1..)
        }
    }
}