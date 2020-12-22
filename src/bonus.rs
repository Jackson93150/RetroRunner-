use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};
extern crate rand;
use rand::Rng;

// on reprend les memes bases que pour la deathstar

pub struct Bonus {
    pub position: Vector2<f32>,
    movement_left: Vector2<f32>,
    pub state: bool,
}

impl Bonus {
    pub fn new(_context: &mut Context) -> GameResult<Self> {
        let y = rand::thread_rng().gen_range(-70.0, 670.0);
        let position = Vector2::new(1600.0, y); // position du background
        let movement_left = Vector2::new(-2.0, 0.0); // vitesse a laquelle le background va se deplacer
        let state = false;
        Ok(Bonus {
            position,
            movement_left,
            state,
        })
    } // meme principe que pour la deathstar mais adapter pour le background
    pub fn location(&self) -> Point2<f32> {
        Point2::new(self.position.x, self.position.y)
    }
    pub fn movement(&mut self) {
        self.position += self.movement_left;
    }
    pub fn respawn(&mut self) {
        if self.position.x < -300.0 || self.state == true {
            // on regarde si le bonus et sortie de l'écran ou a ete pris par le joueur
            let y = rand::thread_rng().gen_range(-70.0, 670.0); // et on fait respawn le bonus et on passe a l'état false
            self.position = Vector2::new(1430.0, y);
            self.state = false;
        }
    }
}
