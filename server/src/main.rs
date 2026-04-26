use std::io::Read;

use server::{domain_model::parser::parse_graph, image_generation::placers::force_directed};

fn main() {
    // For now, take input from stdin and print the resulting graph to stdout
    println!("Enter a graph description (end with Ctrl+D):");
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();

    match parse_graph(&buf) {
        Ok(graph) => {
            println!("Starting simulation...");
            let mut sim = force_directed::Sim::new(&graph);
            println!("Running simulation...");
            sim.run();
            println!("Building Grid...");
            let grid = sim.build_grid();
            println!("Generated grid:\n{}", grid);
        },
        Err(e) => {
            println!("Failed to parse graph: {}", e);
        }
    }
}
