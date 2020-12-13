use ggez::{Context,GameResult,graphics::{self, Image}};
use ggez::nalgebra;

pub struct Player {
    player_img : Image,
}

impl Player{
    pub fn new(context: &mut Context)->GameResult<Self>{
        let player_img = Image::new(context,"/perso.png")?;
        Ok(Player{
            player_img,
        })
    }
    pub fn update(&mut self) -> GameResult{
        Ok(())
    }
    pub fn draw(&self,context: &mut Context) -> GameResult {
         graphics::draw(
            context,
            &self.player_img,
            graphics::DrawParam::new().scale(nalgebra::Vector2::new(1.0, 1.0)),
        )?;
        Ok(())
    }
}