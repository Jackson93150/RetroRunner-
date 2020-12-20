use ggez::{Context,GameResult};
use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;

pub struct Health{
    position : Vector2<f32>,
    pub health : i32, //point  de vie
}


impl Health{
    pub fn new(_context: &mut Context)->GameResult<Self>{
        let position = Vector2::new(1150.0,0.0); // position de la barre de vie
        let health = 3; // point de vie
        Ok(Health{
            position,
            health,
        })
    }
    pub fn location(&self) -> Point2<f32>{ 
        Point2::new(self.position.x,self.position.y)
    }
}
