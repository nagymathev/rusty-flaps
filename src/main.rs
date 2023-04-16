use bracket_lib::prelude::*;

fn main() -> BError{
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy")
        .build()?;
    main_loop(context, State::new())
}

enum GameMode {
    Menu,
    Playing,
    GameOver,
}

struct State {
    mode: GameMode,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket World!");
    }
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }
}