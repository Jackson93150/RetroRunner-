use ggez::{Context,GameResult};
use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;
extern crate rand;
use rand::Rng;
use super::player::Player;

// on reprend les memes bases que pour le player

pub struct Deathstar{
    pub position : Vector2<f32>,
    movement_left : Vector2<f32>,
    pub score : i32,
}

impl Deathstar{
    pub fn new(_context: &mut Context)->GameResult<Self>{
        let y = rand::thread_rng().gen_range(-100.0, 700.0); // on applique une position en y de facon aléatoire
        let position = Vector2::new(1400.0,y); // et on fixe la position initial de sorte que l'ennemie apparait en dehors de l'écran
        let movement_left = Vector2::new(-0.25,0.0); // vitesse a laquelle l'ennemie va se deplacer 
        let score = 0;
        Ok(Deathstar{
            position,
            movement_left,
            score,
        })
    }

    pub fn location(&self) -> Point2<f32>{ 
        Point2::new(self.position.x,self.position.y)
    }

    pub fn movement(&mut self){ // fonction pour que l'ennemie se deplace continuellement a la gauche
        self.position += self.movement_left;
    }
    pub fn respawn(&mut self){ // fonction qui va permettre de faire réappaitre l'énnemie apres qu'il soit sortie de l'écran
        let y = rand::thread_rng().gen_range(-100.0, 700.0);
        if self.position.x < -200.0{
            self.score = self.score + 1;
            self.position = Vector2::new(1450.0,y);
        }
    }
    pub fn speed(&mut self){ // augmente la vitesse de l'énnemmie au fur et a mesure du temps
        loop{
            if self.movement_left.x > - 15.0 {
                self.movement_left.x += - 0.001;
            }
            break;
        }
    }
    
    pub fn collision(&mut self,player:&Player)-> bool{ // fonction qui va déterminer si le joueur est rentrer en collision avec l'énnemie
        let ploc = player.location(); // (fonction a retravailler pour etres plus précis)
        if self.position.x <= ploc.x && self.position.x >= ploc.x - 185.0 && self.position.y <= ploc.y + 60.0 && self.position.y >= ploc.y - 160.0{
            return true; // modifiera cela pour que le resultat arrete le jeu et nous affiche le score + bouton redémarrer
        }
        else {
            return false;
        }
    }
    pub fn reset(&mut self){
        let y = rand::thread_rng().gen_range(-100.0, 700.0); 
        self.position = Vector2::new(1400.0,y);
        self.movement_left = Vector2::new(-0.25,0.0);
        self.score = 0;
    }
}

