use std::{collections::HashMap, num::NonZeroUsize, ops::{Range, RangeFrom}};

use serde::{Deserialize, Serialize};

use crate::image_generation::placers::Vec2;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Style {
    Regular,
    Dotted,
    Dashed,
    Bold
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Arrow {
    None,
    Arrow
}

pub type EntityIndex = usize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    pub text: Option<String>,
    pub weight: NonZeroUsize,
    pub entity_1: EntityIndex,
    pub entity_2: EntityIndex,
    pub arrow_1: Arrow,
    pub arrow_2: Arrow,
    pub mult_1: Multiplicity,
    pub mult_2: Multiplicity
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Multiplicity {
    None,
    #[serde(rename_all = "camelCase")]
    Range(Range<usize>),
    #[serde(rename_all = "camelCase")]
    Number(usize),
    #[serde(rename_all = "camelCase")]
    RangeFrom(RangeFrom<usize>)
}

type ColorHexValue = u32;

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub color: ColorHexValue,
    pub style: Style
}

#[derive(Default, Serialize, Deserialize)]
pub struct Graph {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>,
    pub pins: HashMap<EntityIndex, Vec2>,
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
            text: None,
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