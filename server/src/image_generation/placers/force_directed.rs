//! This placer uses a force-directed algorithm to place nodes on a grid.
//! Every node is attached to every other node by a spring, and the system is 
//! simulated until it reaches a stable state.
//! Nodes that aren't connected in the domain model have a weak spring (of the
//! longest length present in the model + 1), while nodes that are connected
//! have a strong spring.

use std::collections::HashMap;

use crate::{domain_model::graph::Graph, image_generation::placers::{GridNode, GridPlacements, Vec2}};

const STRONG_SPRING_K: f32 = 0.20;
const WEAK_SPRING_K: f32 = 0.00;

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
            let radius = (entity as f32).sqrt();
            SimNode::new(entity, radius * cos, radius * sin)
        }).collect();

        if let Some(sim_node) = nodes.first_mut() {
            sim_node.pinned = true;
        }

        let mut neighbors = HashMap::new();

        for relation in graph.relations.iter() {
            let node_neighbors = neighbors.entry(relation.entity_1).or_insert_with(HashMap::new);
            node_neighbors.insert(relation.entity_2, (relation.text.len() as f32 * 0.25).max(2.0) + 1.0 * (relation.weight.get() - 1) as f32);

            let other_neighbors = neighbors.entry(relation.entity_2).or_insert_with(HashMap::new);
            other_neighbors.insert(relation.entity_1, (relation.text.len() as f32 * 0.25).max(2.0) + 1.0 * (relation.weight.get() - 1) as f32);
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

    fn step(&mut self) {
        let mut buffer = self.nodes.clone();
        for node in buffer.iter_mut() {
            if node.pinned { continue }
            node.pos += node.vel;
            node.vel *= 0.5; // damping
            node.vel.y += 0.05; // gravity
        }
        for node in buffer.iter_mut() {
            for other in self.nodes.iter() {
                if other.entity_id == node.entity_id { continue }

                let offset = other.pos - node.pos;
                if let Some(neighbors) = self.neighbors.get(&node.entity_id) {
                    if let Some(&desired_dist) = neighbors.get(&other.entity_id) {
                        let (spring_constant, desired_dist) = (STRONG_SPRING_K, desired_dist);
                        let equilibrium = unsafe { offset.chess_normalized_unchecked() } * (desired_dist);
                        node.vel += (offset - equilibrium) * spring_constant;
                    } else {
                        let desired_dist = 1.5;
                        let equilibrium = unsafe { offset.chess_normalized_unchecked() } * (desired_dist);
                        node.vel -= equilibrium / 2.0f32.powf(1.0 + offset.squared_length().sqrt());
                    };
                }
            }
        }
        self.nodes = buffer;
    }

    fn step_toward_grid(&mut self) {
        for node in self.nodes.iter_mut() {
            let grid_pos = Vec2 { x: node.pos.x.round(), y: node.pos.y.round() };
            node.pos += (grid_pos - node.pos) * 0.2;
        }
    }
    
    fn all_nodes_on_grid(&self) -> bool {
        !self.nodes.iter()
            .any(|node| 
                (node.pos.x - node.pos.x.round()).abs() >= 0.01 || 
                (node.pos.y - node.pos.y.round()).abs() >= 0.01
            )
    }

    fn keep_nodes_apart(&mut self) {
        for i in 0..self.nodes.len() {
            for j in (i+1)..self.nodes.len() {
                let offset = self.nodes[j].pos - self.nodes[i].pos;
                if offset.squared_length() < 0.5625 {
                    let push = unsafe { offset.normalized_unchecked() } * 0.75;
                    self.nodes[i].pos -= push;
                    self.nodes[j].pos += push;
                }
            }
        }
    }

    pub fn run(&mut self) {
        for _ in 0..100 {
            self.step();
        }
    }

    #[must_use]
    pub fn build_grid(mut self) -> GridPlacements {
        while !self.all_nodes_on_grid() {
            self.step_toward_grid();
            self.keep_nodes_apart();
        }

        let mut nodes: Vec<GridNode> = self.nodes.into_iter().map(|node| 
            GridNode { 
                entity: node.entity_id, 
                position: Vec2 {
                    x: node.pos.x.round(), 
                    y: node.pos.y.round()
                }
            }
        ).collect();

        // Center around the origin
        let (avg_x, avg_y) = {
            let (mut sum_x, mut sum_y) = (0.0, 0.0);
            for node in nodes.iter() {
                sum_x += node.position.x;
                sum_y += node.position.y;
            }
            ((sum_x / nodes.len() as f32).floor(), (sum_y / nodes.len() as f32).floor())
        };
        for node in nodes.iter_mut() {
            node.position.x -= avg_x;
            node.position.y -= avg_y;
        }

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
        graph.entities = vec![dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity(), dummy_entity()];
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