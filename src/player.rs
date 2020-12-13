use ggez::{Context,GameResult};
use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;


pub struct Player {
    position : Vector2<f32>,
    acceleration : Vector2<f32>,
    speed : Vector2<f32>,
}

impl Player{
    pub fn new(_context: &mut Context)->GameResult<Self>{
        let position = Vector2::new(150.0,150.0); // initialisation de la position du joueur / acceleration / speed
        let acceleration = Vector2::new(0.0,0.0);
        let speed = Vector2::new(0.0,0.0);
        Ok(Player{
            position,
            acceleration,
            speed,
        })
    }

    pub fn location(&self) -> Point2<f32>{ // determine la position du joueur 
        Point2::new(self.position.x,self.position.y)
    }

    // implementation d'un algorithme simple pour la gravité
    pub fn create_gravity(&mut self, pression : &Vector2<f32>) {
        self.acceleration += pression;
    }
    pub fn setup_gravity(&mut self) {
        self.speed += self.acceleration;
        self.position += self.speed;
        self.acceleration *= 0.0;
    }
    // on fixe des limite pour pas que notre perso sort de l'écran
    pub fn limite(&mut self,perimetre : f32) {
        if self.position.y + 75.0 > perimetre {
            self.position.y = perimetre - 80.0 ; 
            self.speed.y = 0.0;
        }
    }
}