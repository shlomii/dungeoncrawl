#![warn(clippy::pedantic)]

mod map;
mod map_builder;
mod player;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

enum GameMode {
    Menu,
}
struct State {
    mode: GameMode,
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);
        Self {
            mode: GameMode::Menu,
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, "Welcome to the dungeon");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        /*  if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        } */
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
        /*
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
        }
        */
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;

    main_loop(ctx, State::new())
}
