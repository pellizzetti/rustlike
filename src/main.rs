#[macro_use]
extern crate lazy_static;

mod dungeon;
mod map;

use bracket_terminal::prelude::*;

use dungeon::generate_dungeon;
use map::Map;

bracket_terminal::add_wasm_support!();

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;
const MAP_WIDTH: u32 = 80;
const MAP_HEIGHT: u32 = 45;
const MAP_MAX_ROOMS: u32 = 30;
const ROOM_MAX_SIZE: u32 = 10;
const ROOM_MIN_SIZE: u32 = 6;

struct Player {
    x: u32,
    y: u32,
}

struct State {
    player: Player,
    map: Map,
}

impl State {
    fn new() -> State {
        let (map, first_room) = generate_dungeon(
            MAP_WIDTH,
            MAP_HEIGHT,
            MAP_MAX_ROOMS,
            ROOM_MAX_SIZE,
            ROOM_MIN_SIZE,
        );
        let player = Player {
            x: first_room.x as u32,
            y: first_room.y as u32,
        };
        State {
            player: player,
            map: map,
        }
    }
    fn move_player(&mut self, x: i8, y: i8) {
        let delta_x = (self.player.x as i8 + x) as u32;
        let delta_y = (self.player.y as i8 + y) as u32;

        let tile_idx = self.map.map_idx(delta_x, delta_y);
        let tile = self.map.tiles[tile_idx];
        if tile.walkable {
            self.player.x = delta_x;
            self.player.y = delta_y;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        self.map.render(ctx);
        ctx.print(self.player.x, self.player.y, "@");

        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Up => self.move_player(0, -1),
                VirtualKeyCode::Down => self.move_player(0, 1),
                VirtualKeyCode::Left => self.move_player(-1, 0),
                VirtualKeyCode::Right => self.move_player(1, 0),

                _ => {}
            },
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Rustlike")
        .with_fps_cap(60.0)
        .with_dimensions(CONSOLE_WIDTH, CONSOLE_HEIGHT)
        .with_tile_dimensions(10u32, 16u32)
        .with_resource_path("resources/")
        .with_font("terminal_10x16.png", 10u32, 16u32)
        .with_simple_console_no_bg(CONSOLE_WIDTH, CONSOLE_HEIGHT, "terminal_10x16.png")
        .build()?;

    let game_state = State::new();

    main_loop(context, game_state)
}
