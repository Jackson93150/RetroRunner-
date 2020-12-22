use ggez::{Context,GameResult};
use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;

// on reprend les memes bases que pour la deathstar

pub struct Background{
    pub position : Vector2<f32>,
    movement_left : Vector2<f32>,
}


impl Background{
    pub fn new(_context: &mut Context)->GameResult<Self>{
        let position = Vector2::new(0.0,0.0); // position du background
        let movement_left = Vector2::new(-1.0,0.0); // vitesse a laquelle le background va se deplacer 
        Ok(Background{
            position,
            movement_left,
        })
    } // meme principe que pour la deathstar mais adapter pour le background
    pub fn location(&self) -> Point2<f32>{ 
        Point2::new(self.position.x,self.position.y)
    }
    pub fn movement(&mut self){ 
        self.position += self.movement_left;
    }
    pub fn respawn(&mut self){
        if self.position.x < -2560.0{
            self.position = Vector2::new(2560.0,0.0);
        }
    }
}
