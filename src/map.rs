#[warn(clippy::pedantic)]
use crate::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    timer: std::time::SystemTime,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            timer: SystemTime::now(),
        }
    }
    pub fn render(&mut self, ctx: &mut BTerm) {
        let sys_time = SystemTime::now()
            .duration_since(self.timer)
            .expect("Clock issue???");
        //println!("{:?}", sys_time);
        self.timer = SystemTime::now();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }
    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
