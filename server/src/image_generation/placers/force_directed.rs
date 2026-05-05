//! This placer uses a force-directed algorithm to place nodes on a grid.
//! Every node is attached to every other node by a spring, and the system is 
//! simulated until it reaches a stable state.
//! Nodes that aren't connected in the domain model have a weak spring (of the
//! longest length present in the model + 1), while nodes that are connected
//! have a strong spring.

use std::{collections::HashMap};

use crate::{domain_model::graph::Graph, image_generation::placers::{GridNode, GridPlacements, Vec2}};

const ITERATIONS: usize = 1000;
const fn neighbor_spring_k(iterations: usize) -> f32 {
    if iterations < 100 {
        0.008 * iterations as f32 // Warm up
    } else if iterations < ITERATIONS / 2 {
        0.8
    } else {
        0.0
    }
}
const DESIRED_EXPANSION_PUSH: f32 = 4.0;
const EXPANSION_SPRING_K: f32 = 0.2;
const fn orth_factor(iterations: usize) -> f32 {
    if iterations < ITERATIONS / 2 {
        0.0
    } else {
        0.8
    }
}
const DELTA_TIME: f32 = 0.10;

type EntityID = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Sim {
    highest_desired_dist: f32,
    neighbors: HashMap<EntityID, HashMap<EntityID, f32>>,
    nodes: Vec<SimNode>
}

impl Sim {
    pub fn new(graph: &Graph) -> Sim {
        let angle_delta = 2.3999; // radians
        let mut nodes: Vec<SimNode> = graph.entities.iter().enumerate().map(|(entity, _)| {
            let (sin, cos) = (entity as f32 * angle_delta).sin_cos();
            let radius = 4.0 * (entity as f32).sqrt();
            SimNode::new(entity, radius * cos, radius * sin)
        }).collect();

        if let Some(sim_node) = nodes.first_mut() {
            sim_node.pinned = true;
        }

        for (entity_id, pos) in graph.pins.iter() {
            if let Some(sim_node) = nodes.get_mut(*entity_id) {
                sim_node.pos = *pos;
                sim_node.pinned = true;
            }
        }

        let mut neighbors = HashMap::new();

        for relation in graph.relations.iter() {
            let relation_strength = {
                let text_strength = relation.text.as_ref().map_or(0, |s| s.len()) as f32 * 0.2;
                let weight_strength = (relation.weight.get() as f32 * 0.5 - 1.0).max(0.0);
                2.0 + text_strength + weight_strength
            };

            let node_neighbors = neighbors.entry(relation.entity_1).or_insert_with(HashMap::new);
            node_neighbors.insert(relation.entity_2, relation_strength);

            let other_neighbors = neighbors.entry(relation.entity_2).or_insert_with(HashMap::new);
            other_neighbors.insert(relation.entity_1, relation_strength);
        }

        let highest_desired_dist = neighbors.values()
            .flat_map(HashMap::values)
            .map(|&dist| dist * 2.0)
            .fold(3.0, f32::max);

        for entity_id in nodes.iter().enumerate().map(|(entity_id, _)| entity_id) {
            neighbors.entry(entity_id).or_insert_with(HashMap::new);
        }

        Sim { nodes, neighbors, highest_desired_dist }
    }

    fn step(&mut self, iteration: usize, num_nodes: usize) {
        let mut buffer = self.nodes.clone();
        for node in buffer.iter_mut() {
            if node.pinned { continue }
            node.pos += node.vel * DELTA_TIME;
            node.vel *= 0.95; // damping
        }
        for node in buffer.iter_mut().take(num_nodes) {
            if node.pinned { continue }
            for other in self.nodes.iter().take(num_nodes) {
                if other.entity_id == node.entity_id { continue }

                let offset = other.pos - node.pos;
                if let Some(neighbors) = self.neighbors.get(&node.entity_id) {
                    let acceleration = if let Some(&desired_dist) = neighbors.get(&other.entity_id) {
                        let spring_constant = neighbor_spring_k(iteration);
                        
                        let pull = {
                            if let Some(normalized) = offset.chess_normalized() {
                                let equilibrium = normalized * desired_dist;
                                (offset - equilibrium) * spring_constant * DELTA_TIME
                            } else {
                                Vec2 { x: 0.0, y: 0.0 }
                            }
                        };

                        let align = {
                            // Check which spot next to them we are closest to and try to go there
                            let (mut closest_spot, mut closest_squared_length) = (Vec2 { x: f32::INFINITY, y: f32::INFINITY }, f32::INFINITY);
                            for spot_offset in [
                                Vec2 { x: 0.0, y: 1.0 },
                                Vec2 { x: 0.0, y: -1.0 },
                                Vec2 { x: 1.0, y: 0.0 },
                                Vec2 { x: -1.0, y: 0.0 },
                                Vec2 { x: 1.0, y: 1.0 },
                                Vec2 { x: 1.0, y: -1.0 },
                                Vec2 { x: -1.0, y: 1.0 },
                                Vec2 { x: -1.0, y: -1.0 },
                            ] {
                                let spot_pos = other.pos + spot_offset * desired_dist;
                                let squared_dist = (spot_pos - node.pos).squared_length();
                                if squared_dist < closest_squared_length {
                                    closest_spot = spot_pos;
                                    closest_squared_length = squared_dist;
                                }
                            }
                            let to_orthogonal = closest_spot - node.pos;

                            to_orthogonal * DELTA_TIME * orth_factor(iteration)
                        };

                        pull + align
                    } else {
                        let push = {
                            if let Some(normalized) = offset.taxicab_normalized() {
                                let desired_offset = normalized * DESIRED_EXPANSION_PUSH;
                                if offset.chess_length() < desired_offset.chess_length() {
                                    (offset - desired_offset) * DELTA_TIME * EXPANSION_SPRING_K
                                } else {
                                    Vec2 { x: 0.0, y: 0.0 }
                                }
                            } else {
                                Vec2 { x: 0.0, y: 0.0 }
                            }
                        };

                        push
                    };

                    if other.pinned {
                        node.vel += acceleration * 2.0;
                    } else {
                        node.vel += acceleration;
                    }
                }
            }
        }
        self.nodes = buffer;
    }

    fn step_toward_grid(&mut self) {
        for node in self.nodes.iter_mut() {
            if node.pinned { continue }
            let grid_pos = Vec2 { x: node.pos.x.round(), y: node.pos.y.round() };
            if (node.pos - grid_pos).squared_length() < 0.0001 { 
                node.pos = grid_pos;
                continue;
            }
            node.pos += (grid_pos - node.pos) * DELTA_TIME;
        }
    }

    fn keep_nodes_apart(&mut self) {
        for i in 0..self.nodes.len() {
            for j in (i+1)..self.nodes.len() {
                let offset = self.nodes[j].pos - self.nodes[i].pos;
                if offset.squared_length() < 0.5625 {
                    let push = offset.normalized().unwrap_or(Vec2 { x: 0.0, y: 0.0 }) * 0.75;
                    if self.nodes[i].pinned && self.nodes[j].pinned {
                        continue;
                    } else if self.nodes[i].pinned {
                        self.nodes[j].pos += push * 2.0;
                    } else if self.nodes[j].pinned {
                        self.nodes[i].pos -= push * 2.0;
                    } else {
                        self.nodes[i].pos -= push;
                        self.nodes[j].pos += push;
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        // TODO: Skip pinned nodes
        for nodes in 2..self.nodes.len() + 1 {
            for iteration in 0..ITERATIONS {
                self.step(iteration, nodes);
            }
            for _ in 0..100 {
                self.step_toward_grid();
                self.keep_nodes_apart();
            }
        }
    }

    #[must_use]
    pub fn build_grid(self) -> GridPlacements {
        let nodes: Vec<GridNode> = self.nodes.into_iter().map(|node| 
            GridNode { 
                entity: node.entity_id, 
                position: node.pos
            }
        ).collect();

        GridPlacements { nodes }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimNode {
    entity_id: usize,
    pos: Vec2,
    vel: Vec2,
    pinned: bool,
}

impl SimNode {
    pub fn new(entity_id: usize, x: f32, y: f32) -> SimNode {
        SimNode {
            entity_id,
            pos: Vec2 { x, y },
            vel: Vec2 { x: 0.0, y: 0.0 },
            pinned: false,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::domain_model::graph::test::*;

    use super::*;

    #[test]
    fn generation_is_consistent() {
        let mut graph = Graph::new();
        graph.entities = vec![dummy_entity(), dummy_entity(), dummy_entity()];
        graph.relations = vec![dummy_relation(0, 2), dummy_relation(1, 2)];

        assert_eq!(Sim::new(&graph), Sim::new(&graph));
    }

    #[test]
    fn nodes_stay_apart() {
        let mut graph = Graph::new();
        graph.entities = vec![dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity()];
        graph.relations = vec![
            dummy_relation(0, 2), 
            dummy_relation(1, 2), 
            dummy_relation(3, 2), 
            dummy_relation(3, 4), 
            dummy_relation(4, 2)
        ];

        let mut sim = Sim::new(&graph);
        sim.run();

        let grid = sim.build_grid();

        println!("{}", grid);

        for node in grid.nodes.iter() {
            for other in grid.nodes.iter() {
                if node.entity == other.entity { continue }
                let node_pos = Vec2 { x: node.position.x, y: node.position.y };
                let other_pos = Vec2 { x: other.position.x, y: other.position.y };
                assert!(node_pos != other_pos, "Entities {} and {} are on the same point on the grid! ({:?})", node.entity, other.entity, node_pos);
            }
        }
    }

    #[test]
    fn nodes_stay_together() {
        let mut graph = Graph::new();
        graph.entities = vec![dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity()];
        graph.relations = vec![
            dummy_relation(0, 2), 
            dummy_relation(1, 2), 
            dummy_relation(3, 2), 
            dummy_relation(3, 4), 
            dummy_relation(4, 2), 
            dummy_relation(6, 2), 
            dummy_relation(7, 6), 
            dummy_relation(5, 8), 
            dummy_relation(2, 8), 
            dummy_relation(1, 8), 
            dummy_relation(1, 7), 
        ];

        let mut sim = Sim::new(&graph);
        sim.run();

        let grid = sim.build_grid();

        println!("{}", grid);

        for node in grid.nodes.iter() {
            let mut farthest_distance = f32::INFINITY;
            for other in grid.nodes.iter() {
                if node.entity == other.entity { continue }
                let node_pos = node.position;
                let other_pos = other.position;
                farthest_distance = f32::min(farthest_distance, (other_pos - node_pos).taxicab_length());
            }
            assert!(farthest_distance <= 3.0, "Entity {} is too far away from everything! ({} units)", node.entity, farthest_distance);
        }
    }
}