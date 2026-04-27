use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::domain_model::graph::EntityIndex;

pub mod force_directed;

#[cfg_attr(not(test), allow(unused))]
#[derive(Serialize, Deserialize)]
pub struct GridPlacements {
    nodes: Vec<GridNode>
}

#[cfg_attr(not(test), allow(unused))]
#[derive(Serialize, Deserialize)]
pub struct GridNode {
    entity: EntityIndex,
    position: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

impl std::fmt::Display for GridPlacements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, min_y, max_y) = {
            let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);

            for node in self.nodes.iter() {
                min_x = isize::min(min_x, node.position.x.floor() as isize);
                max_x = isize::max(max_x, node.position.x.ceil() as isize);
                min_y = isize::min(min_y, node.position.y.floor() as isize);
                max_y = isize::max(max_y, node.position.y.ceil() as isize);
            }

            (min_x, max_x, min_y, max_y)
        };

        for x in (min_x-1)..=(max_x+1) {
            for y in (min_y-1)..=(max_y+1) {
                if let Some(node) = self.nodes.iter().find(|node| node.position.x.round() as isize == x && node.position.y.round() as isize == y) {
                    write!(f, "{}", node.entity)?
                } else if x % 2 == 0 && y % 2 == 0 {
                    if x == 0 || y == 0 {
                        write!(f, "*")?
                    } else {
                        write!(f, "`")?
                    }
                } else {
                    write!(f, " ")?
                }
                write!(f, " ")?
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
