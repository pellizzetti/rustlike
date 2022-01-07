use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

struct Player {
    x: i32,
    y: i32,
}

struct State {
    player: Player,
}

impl State {
    pub fn new() -> State {
        State {
            player: Player { x: 1, y: 1 },
        }
    }
    pub fn move_player(&mut self, x: i32, y: i32) {
        self.player.x += x;
        self.player.y += y;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
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

    let game_state: State = State::new();

    main_loop(context, game_state)
}
