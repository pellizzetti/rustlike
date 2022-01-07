use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print(1, 1, "Hey!");
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

    let game_state: State = State {};

    main_loop(context, game_state)
}
