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

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }
}

struct State {
    mode: GameMode,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::GameOver => self.game_over(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }
    
    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::GameOver;
    }
    
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }
    
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Flappy");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Dead");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}