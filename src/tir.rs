use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};

pub struct Tir {
    pub position: Vector2<f32>,
    pub state: bool, //point  de vie
    pub movement_right: f32,
}

impl Tir {
    pub fn new(_context: &mut Context) -> GameResult<Self> {
        let position = Vector2::new(250.0, 250.0); // position de la barre de vie
        let state = false; // point de vie
        let movement_right = 22.0;
        Ok(Tir {
            position,
            state,
            movement_right,
        })
    }
    pub fn location(&self) -> Point2<f32> {
        Point2::new(self.position.x, self.position.y)
    }
    pub fn movement(&mut self) {
        // si le tir est en etat true il va ller en ligne droite a la vitesse défini
        if self.state == true {
            if self.position.x < 1450.0 {
                self.position.x += self.movement_right;
            }
        }
    }
    pub fn respawn(&mut self) {
        // si le tir sort de l'écran son état passe a false
        if self.position.x > 1450.0 {
            self.state = false;
        }
    }
}
