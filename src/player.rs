use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};

pub struct Player {
    pub position: Vector2<f32>,
    acceleration: Vector2<f32>,
    pub speed: Vector2<f32>,
    turbo_power: Vector2<f32>,
    turbo_stop: Vector2<f32>,
    movement_left: Vector2<f32>,
    movement_right: Vector2<f32>,
    pub life: bool,
}

impl Player {
    pub fn new(_context: &mut Context) -> GameResult<Self> {
        let position = Vector2::new(250.0, 250.0); // initialisation des paramétres du joueur
        let acceleration = Vector2::new(0.0, 0.0);
        let speed = Vector2::new(0.0, 0.0);
        let turbo_power = Vector2::new(0.0, -0.025);
        let turbo_stop = Vector2::new(0.0, 0.015);
        let movement_left = Vector2::new(-0.007, 0.0);
        let movement_right = Vector2::new(0.007, 0.0);
        let life = true;
        Ok(Player {
            position,
            acceleration,
            speed,
            turbo_power,
            turbo_stop,
            movement_left,
            movement_right,
            life,
        })
    }

    pub fn location(&self) -> Point2<f32> {
        // determine la position du joueur
        Point2::new(self.position.x, self.position.y)
    }

    // implementation d'un algorithme simple pour la gravité
    pub fn create_gravity(&mut self, pression: &Vector2<f32>) {
        self.acceleration += pression;
    }
    pub fn setup_gravity(&mut self) {
        self.speed += self.acceleration;
        self.position += self.speed;
        self.acceleration *= 0.0;
    }
    // on fixe des limite pour pas que notre perso sort de l'écran
    pub fn limite(&mut self, perimetre: f32) {
        // limite pour le bas de l'écran
        while self.position.y + 75.0 > perimetre {
            self.position.y = perimetre - 80.0;
            self.speed.y = 0.0;
            break;
        }
    }
    pub fn limite2(&mut self, perimetre: f32) {
        // limite pour le haut de l'écran
        while self.position.y - 80.0 < perimetre {
            self.position.y = perimetre + 80.0;
            self.speed.y = 0.0;
            break;
        }
    }
    pub fn limite3(&mut self, perimetre: f32) {
        // limite pour le coté gauche de l'écran
        while self.position.x - 10.0 < perimetre {
            self.position.x = perimetre + 10.0;
            self.speed.x = 0.0;
            break;
        }
    }
    pub fn limite4(&mut self, perimetre: f32) {
        // limite pour le coté droit de l'écran
        while self.position.x + 50.0 > perimetre {
            self.position.x = perimetre - 50.0;
            self.speed.x = 0.0;
            break;
        }
    }
    // on créer des fonction qui va changer l'impact de la gravité
    // ce qui va nous permettre de nous deplacer
    pub fn fly(&mut self) {
        let turbo_power = self.turbo_power;
        self.create_gravity(&turbo_power);
    }
    pub fn stop_fly(&mut self) {
        let turbo_stop = self.turbo_stop;
        self.create_gravity(&turbo_stop);
    }
    pub fn fly_left(&mut self) {
        let movement_left = self.movement_left;
        self.create_gravity(&movement_left);
    }
    pub fn fly_right(&mut self) {
        let movement_right = self.movement_right;
        self.create_gravity(&movement_right);
    }
    pub fn reset(&mut self) {
        // reset les value du player
        self.position = Vector2::new(250.0, 250.0);
        self.speed = Vector2::new(0.0, 0.0);
    }
}
