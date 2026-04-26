//! This placer uses a force-directed algorithm to place nodes on a grid.
//! Every node is attached to every other node by a spring, and the system is 
//! simulated until it reaches a stable state.
//! Nodes that aren't connected in the domain model have a weak spring (of the
//! longest length present in the model + 1), while nodes that are connected
//! have a strong spring.

use std::{collections::HashMap, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use crate::domain_model::graph::Graph;

const STRONG_SPRING_K: f32 = 0.10;
const WEAK_SPRING_K: f32 = 0.01;

type EntityID = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Sim {
    highest_desired_dist: f32,
    neighbors: HashMap<EntityID, HashMap<EntityID, f32>>,
    nodes: Vec<SimNode>
}

impl Sim {
    pub fn new(graph: &Graph) -> Sim {
        let angle_delta = 1.0; // radians
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
            node_neighbors.insert(relation.entity_2, 2.0 * relation.weight.get() as f32);

            let other_neighbors = neighbors.entry(relation.entity_2).or_insert_with(HashMap::new);
            other_neighbors.insert(relation.entity_1, 2.0 * relation.weight.get() as f32);
        }

        let highest_desired_dist = neighbors.values()
            .flat_map(HashMap::values)
            .fold(0.0, |a, b| f32::max(a, *b));

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
        }
        for node in buffer.iter_mut() {
            for other in self.nodes.iter() {
                if other.entity_id == node.entity_id { continue }

                let offset = other.pos - node.pos;
                if let Some(neighbors) = self.neighbors.get(&node.entity_id) {
                    let (spring_constant, desired_dist) = {
                        if let Some(&desired_dist) = neighbors.get(&other.entity_id) {
                            (STRONG_SPRING_K, desired_dist)
                        } else {
                            (WEAK_SPRING_K, self.highest_desired_dist * 2.0)
                        }
                    };
                    let equilibrium = unsafe { offset.chess_normalized_unchecked() } * (desired_dist);
                    node.vel += (offset - equilibrium) * spring_constant;
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

    #[must_use]
    pub fn build_grid(mut self) -> GridPlacements {
        while !self.all_nodes_on_grid() {
            self.step_toward_grid();
            self.keep_nodes_apart();
        }

        let mut nodes: Vec<GridNode> = self.nodes.into_iter().map(|node| 
            GridNode { 
                entity: node.entity_id, 
                x: node.pos.x.round() as isize, 
                y: node.pos.y.round() as isize
            }
        ).collect();

        // Center around the origin
        let (avg_x, avg_y) = {
            let (mut sum_x, mut sum_y) = (0, 0);
            for node in nodes.iter() {
                sum_x += node.x;
                sum_y += node.y;
            }
            (sum_x / nodes.len() as isize, sum_y / nodes.len() as isize)
        };
        for node in nodes.iter_mut() {
            node.x -= avg_x;
            node.y -= avg_y;
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

#[cfg_attr(not(test), allow(unused))]
pub struct GridPlacements {
    nodes: Vec<GridNode>
}

#[cfg_attr(not(test), allow(unused))]
pub struct GridNode {
    x: isize,
    y: isize,
    entity: usize
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec2 {
    x: f32,
    y: f32
}

#[allow(unused)]
impl Vec2 {
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn taxicab_length(&self) -> f32 {
        self.x.abs() + self.y.abs()
    }

    pub fn chess_length(&self) -> f32 {
        f32::max(self.x.abs(), self.y.abs())
    }

    pub fn normalized(&self) -> Option<Self> {
        let sqr_len = self.squared_length();
        if sqr_len < f32::EPSILON {
            None
        } else {
            Some(*self / f32::sqrt(sqr_len))
        }
    }

    pub unsafe fn normalized_unchecked(&self) -> Self {
        *self / f32::sqrt(self.squared_length())
    }

    pub fn taxicab_normalized(&self) -> Option<Self> {
        let taxicab_len = self.taxicab_length();
        if taxicab_len < f32::EPSILON {
            None
        } else {
            Some(*self / taxicab_len)
        }
    }

    pub unsafe fn taxicab_normalized_unchecked(&self) -> Self {
        *self / self.taxicab_length()
    }

    pub fn chess_normalized(&self) -> Option<Self> {
        let chess_len = self.chess_length();
        if chess_len < f32::EPSILON {
            None
        } else {
            Some(*self / chess_len)
        }
    }

    pub unsafe fn chess_normalized_unchecked(&self) -> Self {
        *self / self.chess_length()
    }

    pub fn greatest_axis(&self) -> Self {
        if self.x.abs() > self.y.abs() {
            Self { x: self.x, y: 0.0 }
        } else {
            Self { x: 0.0, y: self.y }
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2 { x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use crate::domain_model::graph::test::*;

    use super::*;

    fn run_sim(mut sim: Sim) -> GridPlacements {
        for _ in 0..100 {
            sim.step();
        }

        println!("Finished simulation. Building grid...");

        let grid = sim.build_grid();

        println!("Grid built.");

        draw_grid(&grid);

        grid
    }

    fn draw_grid(grid: &GridPlacements) {
        let (min_x, max_x, min_y, max_y) = {
            let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);

            for node in grid.nodes.iter() {
                min_x = isize::min(min_x, node.x);
                max_x = isize::max(max_x, node.x);
                min_y = isize::min(min_y, node.y);
                max_y = isize::max(max_y, node.y);
            }

            (min_x, max_x, min_y, max_y)
        };

        for x in (min_x-1)..=(max_x+1) {
            for y in (min_y-1)..=(max_y+1) {
                if let Some(node) = grid.nodes.iter().find(|node| node.x == x && node.y == y) {
                    print!("{}", node.entity)
                } else if x == 0 || y == 0 {
                    print!("*");
                } else {
                    print!("`");
                }
                print!(" ");
            }
            println!();
        }
    }

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

        let grid = run_sim(Sim::new(&graph));

        let mut taken_points: HashSet<(isize, isize)> = HashSet::new();

        for node in grid.nodes.iter() {
            assert!(taken_points.insert((node.x, node.y)), "Point {:?} is already on the grid, but entity {} tried to go there.", (node.x, node.y), node.entity);
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

        let grid = run_sim(Sim::new(&graph));

        for node in grid.nodes.iter() {
            let mut farthest_distance = f32::INFINITY;
            for other in grid.nodes.iter() {
                if node.entity == other.entity { continue }
                let node_pos = Vec2 { x: node.x as f32, y: node.y as f32 };
                let other_pos = Vec2 { x: other.x as f32, y: other.y as f32 };
                farthest_distance = f32::min(farthest_distance, (other_pos - node_pos).taxicab_length());
            }
            assert!(farthest_distance <= 3.0, "Entity {} is too far away from everything! ({} units)", node.entity, farthest_distance);
        }
    }
}