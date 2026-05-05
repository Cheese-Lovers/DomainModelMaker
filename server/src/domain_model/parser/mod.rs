mod tokenizer;
mod statementizer;

use core::fmt;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{domain_model::{graph::{Entity, Graph, Style}, parser::{statementizer::{ParseStatementError, Statement, StatementParsingIterator}, tokenizer::{ParseTokenError, Token, TokenParsingIterator}}}, image_generation::placers::Vec2};


#[derive(Serialize, Deserialize)]
pub enum ParseGraphError {
    TokenizationFailed(ParseTokenError),
    StatementizationFailed(ParseStatementError),
}

impl fmt::Display for ParseGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseGraphError::TokenizationFailed(e) => write!(f, "Tokenization failed: {}", e),
            ParseGraphError::StatementizationFailed(e) => write!(f, "Statementization failed: {}", e),
        }
    }
}

pub fn parse_graph(input: &str) -> Result<Graph, ParseGraphError> {
    let raw = input.to_string();
    
    let tokens = {
        TokenParsingIterator::new(input)
            .map(|t| {println!("{t:?}"); t})
            .collect::<Result<Vec<Token>, ParseTokenError>>()
            .map_err(ParseGraphError::TokenizationFailed)
    }?;

    let statements = {
        StatementParsingIterator::new(tokens.iter())
            .collect::<Result<Vec<Statement>, ParseStatementError>>()
            .map_err(ParseGraphError::StatementizationFailed)
    }?;

    // Add entities as they appear, so that the entity ids are in a deterministic order
    let entities = {
        let mut entity_appearances = HashMap::new();
        let mut entity_names = HashSet::new();

        let mut appearance = 0;

        for statement in statements.iter() {
            match statement {
                Statement::NewRelation { entity_1, entity_2, .. } => {
                    entity_appearances.entry(entity_1).or_insert(appearance);
                    appearance += 1;
                    entity_appearances.entry(entity_2).or_insert(appearance);
                    appearance += 1;
                    entity_names.insert(entity_1);
                    entity_names.insert(entity_2);
                },
                Statement::Pin { entity, .. } => {
                    entity_appearances.entry(entity).or_insert(appearance);
                    appearance += 1;
                    entity_names.insert(entity);
                }
            }
        }

        let mut entities = Vec::new();

        for entity_name in entity_names.iter() {
            // TODO: Handle customization
            entities.push(Entity {
                name: (*entity_name).clone(),
                color: 0xff0000,
                style: Style::Regular
            });
        }

        entities.sort_by_key(|e| entity_appearances.get(&e.name).unwrap());

        entities
    };

    let entity_name_to_id: HashMap<String, usize> = entities.iter().enumerate().map(|(id, entity)| (entity.name.clone(), id)).collect();

    let relations = {
        let mut relations = Vec::new();

        for statement in statements.iter() {
            if let Statement::NewRelation { text, weight, entity_1, entity_2, arrow_1, arrow_2, mult_1, mult_2 } = statement {
                relations.push(crate::domain_model::graph::Relation {
                    text: text.clone(),
                    weight: *weight,
                    entity_1: *entity_name_to_id.get(entity_1).unwrap(),
                    entity_2: *entity_name_to_id.get(entity_2).unwrap(),
                    arrow_1: *arrow_1,
                    arrow_2: *arrow_2,
                    mult_1: mult_1.clone(),
                    mult_2: mult_2.clone()
                });
            }
        }

        relations
    };

    let pins = {
        let mut pins = HashMap::new();

        for statement in statements.iter() {
            if let Statement::Pin { entity, x, y } = statement {
                pins.insert(*entity_name_to_id.get(entity).unwrap(), Vec2 { x: *x, y: *y });
            }
        }

        pins
    };

    Ok(Graph { entities, relations, pins, raw })
}