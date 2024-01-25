use std::{ops::Range, ops::RangeFrom};
use crate::domain_model::tokenizer::{self, tokenize_line};

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
    text: String,
    pub entity_1: EntityIndex,
    pub entity_2: EntityIndex,
    arrow_1: Arrow,
    arrow_2: Arrow,
    mult_1: Multiplicity,
    mult_2: Multiplicity
}

pub enum Multiplicity {
    Range(Range<usize>),
    Number(usize),
    RangeFrom(RangeFrom<usize>)
}

type ColorHexValue = u32;

pub struct Entity {
    name: String,
    color: ColorHexValue,
    style: Style
}

pub struct Graph {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>, // a relation is None if the parser could not recognize the statement (should be the same length as raw)
    raw: Vec<String>, // the raw input lines that make up the graph
}

impl Graph {
    pub fn new() -> Self {
        Self {
            entities: Vec::<Entity>::new(),
            relations: Vec::<Relation>::new(),
            raw: Vec::<String>::new()
        }
    }
}

fn generate_graph (input: String) -> Option<Graph> {
    let mut output: Graph = Graph::new();
    
    for line in input.lines() {
        output.raw.push(line.to_string());
        let tokens = tokenize_line(line.to_string());
    }

    None // remove this
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
            entity_1,
            entity_2,
            arrow_1: Arrow::None,
            arrow_2: Arrow::Arrow,
            mult_1: Multiplicity::Number(1),
            mult_2: Multiplicity::RangeFrom(1..)
        }
    }
}