use ggez::{Context,GameResult,graphics};
use ggez::event::EventHandler;
mod player;
use player::Player;

pub struct MyGame {
    player: Player,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let player = Player::new(context)?;
        Ok(MyGame{
            player,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(53, 10, 49));
        self.player.draw(ctx);
        graphics::present(ctx)
    }
}