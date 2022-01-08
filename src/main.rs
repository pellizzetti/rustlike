#[macro_use]
extern crate lazy_static;

use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;
const MAP_WIDTH: u32 = 80;
const MAP_HEIGHT: u32 = 45;

lazy_static! {
    static ref FLOOR_TILE: Tile =
        Tile::new(true, Graphic::new(' ', (255, 255, 255), (50, 50, 150)),);
    static ref WALL_TILE: Tile = Tile::new(false, Graphic::new('#', (255, 255, 255), (0, 0, 100)),);
}

#[derive(Copy, Clone)]
struct Graphic {
    ch: char,
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
}

impl Graphic {
    pub fn new(ch: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Graphic {
        Graphic {
            ch: ch,
            fg: fg,
            bg: bg,
        }
    }
}

#[derive(Copy, Clone)]
struct Tile {
    walkable: bool,
    graphic: Graphic,
}

impl Tile {
    pub fn new(walkable: bool, graphic: Graphic) -> Tile {
        Tile {
            walkable: walkable,
            graphic: graphic,
        }
    }
}

struct Map {
    width: u32,
    height: u32,
    tiles: Box<[Tile]>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let map_size = (width * height) as usize;
        let mut tiles = vec![*FLOOR_TILE; map_size].into_boxed_slice();

        // Create some walls for testing collision
        {
            let walls = &mut tiles[330..360];
            for x in 0..10 {
                walls[x] = *WALL_TILE;
            }
        }

        Map {
            width: width,
            height: height,
            tiles: tiles,
        }
    }
    pub fn map_idx(&self, x: u32, y: u32) -> usize {
        ((y * self.width) + x) as usize
    }
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.map_idx(x, y);
                ctx.set(
                    x,
                    y,
                    self.tiles[index].graphic.fg,
                    self.tiles[index].graphic.bg,
                    to_cp437(self.tiles[index].graphic.ch),
                );
            }
        }
    }
}

struct Player {
    x: u32,
    y: u32,
}

struct State {
    player: Player,
    map: Map,
}

impl State {
    pub fn new() -> State {
        State {
            player: Player { x: 1, y: 1 },
            map: Map::new(MAP_WIDTH, MAP_HEIGHT),
        }
    }
    pub fn move_player(&mut self, x: i8, y: i8) {
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
