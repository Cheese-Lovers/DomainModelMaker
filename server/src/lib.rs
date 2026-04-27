pub mod domain_model;
pub mod image_generation;

use wasm_bindgen::prelude::*;

use crate::{domain_model::parser::parse_graph, image_generation::placers::force_directed};

#[wasm_bindgen]
pub fn generate_graph(input: &str) -> JsValue {
    let result = match parse_graph(input) {
        Ok(graph) => {
            let mut sim = force_directed::Sim::new(&graph);
            sim.run();
            let grid = sim.build_grid();

            Ok((graph, grid))
        },
        Err(e) => {
            Err(e)
        }
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}