use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use ggez::nalgebra;

//use ggez::error::GameError;
//use std::path::PathBuf;

struct MainState {
    background: graphics::Image,
    image: graphics::Image,
    image2: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> { // il faudra mettre copier les 3 images et les mettres dans target ressources pour eviter les bugs
        let s = MainState {
            background: graphics::Image::new(ctx, "/background.png")?,
            image: graphics::Image::new(ctx, "/perso.png")?,
            image2: graphics::Image::new(ctx, "/ennemie.png")?,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self,_ctx : &mut Context) -> GameResult{
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
       // graphics::clear(ctx, graphics::Color::from_rgb(53, 10, 49));
        graphics::draw(
            ctx,
            &self.background,
            graphics::DrawParam::new().scale(nalgebra::Vector2::new(1.5, 1.5)),
        )?;
        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::new().scale(nalgebra::Vector2::new(3.0, 3.0)),
        )?;
        graphics::draw(
            ctx,
            &self.image2,
            graphics::DrawParam::new().scale(nalgebra::Vector2::new(0.5, 0.5)),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let scr = ggez::ContextBuilder::new("retrorunner","Jacku");
    let (ctx,e_loop) = &mut scr.build()?;
    graphics::set_window_title(ctx,"RetroRunner2077");
    let state = &mut MainState::new(ctx)?;
    event::run(ctx,e_loop,state);
    Ok(())
}
