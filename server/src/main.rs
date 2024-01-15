use std::{ops::Range, ops::RangeFrom};
use server::tokenizer::{*, self};

enum Style {
    Regular,
    Dotted,
    Dashed,
    Bold
}

enum Arrow {
    None,
    Arrow
}
type EntityIndex = usize;
struct Relation {
    text: String,
    entity_1: EntityIndex,
    entity_2: EntityIndex,
    arrow_1: Arrow,
    arrow_2: Arrow,
    mult_1: Multiplicity,
    mult_2: Multiplicity
}

enum Multiplicity {
    Range(Range<usize>),
    Number(usize),
    RangeFrom(RangeFrom<usize>)
}

type ColorHexValue = u32;

struct Entity {
    name: String,
    color: ColorHexValue,
    style: Style
}

fn main() {
    let x = 0..;
    let tokens = tokenizer::tokenize_line("hello".to_string());
}



// fn generate_model(input: String){
//     let entities: Vec<Entity> = Vec::new();
//     let relations: Vec<Relation> = Vec::new(); 

//     for line in input.lines() {
        
//     }
// }
