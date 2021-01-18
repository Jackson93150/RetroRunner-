use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};
extern crate rand;
use super::player::Player;
use super::tir::Tir;
use rand::Rng;

// on reprend les memes bases que pour le player

pub struct Deathstar {
    pub position: Vector2<f32>,
    movement_left: Vector2<f32>,
    pub score: i32,
    pub hp: i32,
    pub state: bool,
}

impl Deathstar {
    pub fn new(_context: &mut Context) -> GameResult<Self> {
        let y = rand::thread_rng().gen_range(-100.0, 700.0); // on applique une position en y de facon aléatoire
        let position = Vector2::new(1400.0, y); // et on fixe la position initial de sorte que l'ennemie apparait en dehors de l'écran
        let movement_left = Vector2::new(-0.25, 0.0); // vitesse a laquelle l'ennemie va se deplacer
        let score = 0;
        let hp = 2;
        let state = false;
        Ok(Deathstar {
            position,
            movement_left,
            score,
            hp,
            state,
        })
    }

    pub fn location(&self) -> Point2<f32> {
        Point2::new(self.position.x, self.position.y)
    }

    pub fn movement(&mut self) {
        // fonction pour que l'ennemie se deplace continuellement a la gauche
        self.position += self.movement_left;
    }
    pub fn respawn(&mut self) {
        // fonction qui va permettre de faire réappaitre l'énnemie apres qu'il soit sortie de l'écran ou que ces hp sont a 0
        let y = rand::thread_rng().gen_range(-100.0, 700.0);
        if self.position.x < -200.0 || self.hp == 0 {
            self.score = self.score + 1;
            self.position = Vector2::new(1450.0, y);
            self.hp = 2;
        }
    }
    pub fn speed(&mut self) {
        // augmente la vitesse de l'énnemmie au fur et a mesure du temps
        loop {
            if self.movement_left.x > -15.0 {
                self.movement_left.x += -0.001;
            }
            break;
        }
    }

    // TODO: en cas de collision:
    // Arrete le jeu et nous affiche le score + bouton redémarrer
    pub fn collision(&mut self, player: &Player) -> bool {
        // fonction qui va déterminer si le joueur est rentrer en collision avec l'énnemie
        let ploc = player.location(); // (fonction a retravailler pour etres plus précis)

        self.position.x <= ploc.x
            && self.position.x >= ploc.x - 185.0
            && self.position.y <= ploc.y + 60.0
            && self.position.y >= ploc.y - 160.0
    }

    pub fn tir_hit(&mut self, tir: &Tir) {
        // fonction pour savoir si le tir à toucher les deathstar
        let tirs = tir.state;
        let tirloc = tir.location();
        if self.state == false {
            // on verifie si l'état des deathstar sont en false
            // si la position du tir est dans la hitbox des deathstar
            if tirs == true {
                if self.position.x <= tirloc.x
                    && self.position.x >= tirloc.x - 185.0
                    && self.position.y <= tirloc.y + 60.0
                    && self.position.y >= tirloc.y - 160.0
                {
                    self.hp = self.hp - 1; // on enleve 1 hp
                    self.state = true; // on passe a l'état true
                }
            }
        }
        if tirs == false {
            //si la position du tir et inferieur en x a la deathstar on repasse a l'état false
            self.state = false;
        }
    }
    pub fn reset(&mut self) {
        // on reset toute les value de la deathstar
        let y = rand::thread_rng().gen_range(-100.0, 700.0);
        self.position = Vector2::new(1400.0, y);
        self.movement_left = Vector2::new(-0.25, 0.0);
        self.score = 0;
    }
}
