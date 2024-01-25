//! This module is for placing nodes based on a simulation.
//! In this simulation, nodes are placed spirally on a plane,
//! and are pulled together at a speed inversely proportional
//! to their 'mass', which is a measure of how many neighbors
//! they have.
//! 
//! The way the nodes are placed is consistent.
//! i.e. for a given input graph, the initial placement of nodes
//! is the same. 

use std::{collections::HashSet, ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign}};

use crate::domain_model::graph::Graph;

#[derive(Clone, Debug, PartialEq)]
pub struct Sim {
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

        for relation in graph.relations.iter() {
            nodes[relation.entity_1].neighbors.insert(relation.entity_2);
            nodes[relation.entity_2].neighbors.insert(relation.entity_1);
        }

        nodes.retain(|node| !node.neighbors.is_empty());

        Sim { nodes }
    }

    fn push_step(&mut self) {
        let mut buffer = self.nodes.clone();
        let mut pairs_pushed = HashSet::new();
        for node in buffer.iter_mut() {
            for other in self.nodes.iter() {
                if other.entity == node.entity { continue }
                if !pairs_pushed.insert((node.entity, other.entity)) { continue }

                let dist = other.pos - node.pos;
                if dist.taxicab_length() < 1.0 {
                    match dist.taxicab_normalized() {
                        Some(norm) => node.pos = other.pos - norm,
                        None => node.pos = other.pos + Vec2 { x: 0.0, y: 1.0 }
                    }
                }
            }
        }
        self.nodes = buffer;
    }

    fn move_step(&mut self) {
        let mut buffer = self.nodes.clone();
        const MOVES: usize = 1;
        for _ in 0..MOVES {
            for node in buffer.iter_mut() {
                for other in self.nodes.iter() {
                    // Move toward neighbors faster than other nodes
                    if node.neighbors.contains(&other.entity) {
                        node.pos += (other.pos - node.pos) / node.neighbors.len() as f32 * 0.32;
                    } else {
                        node.pos += (other.pos - node.pos) / node.neighbors.len() as f32 * 0.08;
                    }
                }
            }
        }
        self.nodes = buffer;
    }

    pub fn step(&mut self) {
        self.push_step();
        self.move_step();
    }

    pub fn slim(&mut self) {
        let mut buffer = self.nodes.clone();
        for node in buffer.iter_mut() {
            let mut closest = Vec2 { x: f32::INFINITY, y: f32::INFINITY };
            for other in self.nodes.iter() {
                if other.entity == node.entity { continue }
                if (other.pos - node.pos).taxicab_length() < (closest - node.pos).taxicab_length() {
                    closest = other.pos;
                }
            }
            let displacement = closest - node.pos;
            if displacement.taxicab_length() > 1.1 {
                node.pos = closest - unsafe {displacement.greatest_axis().taxicab_normalized_unchecked()};
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

    #[must_use]
    pub fn build_grid(mut self) -> GridPlacements {
        self.push_step();
        self.slim();
        while !self.all_nodes_on_grid() {
            self.push_step();
            self.step_toward_grid();
        }

        let mut nodes: Vec<GridNode> = self.nodes.into_iter().map(|node| 
            GridNode { 
                entity: node.entity, 
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
    pos: Vec2,
    entity: usize,
    neighbors: HashSet<usize>,
}

impl SimNode {
    pub fn new(entity: usize, x: f32, y: f32) -> SimNode {
        SimNode {
            pos: Vec2 { x, y },
            entity,
            neighbors: HashSet::new()
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
        for _ in 0..30 {
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
                    print!(".");
                }
                print!("   ");
            }
            println!("\n");
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
            assert!(farthest_distance <= 1.0, "Entity {} is too far away from everything! ({} units)", node.entity, farthest_distance);
        }
    }
}