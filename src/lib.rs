use ggez::{Context,GameResult,graphics,timer};
use ggez::graphics::{Image,Font,Scale,Text};
use ggez::event::{EventHandler,KeyCode};
use ggez::nalgebra::{Vector2,Point2};
use ggez::input::keyboard;
extern crate rand;
use rand::Rng;
mod player;
use player::Player;
mod deathstar;
use deathstar::Deathstar;
mod background;
use background::Background;
mod health_bar;
use health_bar::Health;

pub enum GameState { // Structure pour déterminer l'état du jeu
    Start,
    Playing,
    GameOver,
}

pub struct MyGame {
    background : Background,
    background_img : Image,
    background2 : Background,
    background2_img : Image,
    runner: Player,
    player_img : Image,
    deathstar : Deathstar,
    deathstar_img : Image,
    deathstar2 : Deathstar,
    deathstar2_img : Image,
    deathstar3 : Deathstar,
    deathstar3_img : Image,
    gravity : Vector2<f32>,
    health_bar : Health,
    health_bar_img : Image,
    game : GameState,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let background = Background::new(ctx)?;
        let background_img = Image::new(ctx,"/background.png")?; //j'ai du compresser mon background pour gagner en fluidité
        let background2 = Background::new(ctx)?;
        let background2_img = Image::new(ctx,"/background.png")?;
        let runner = Player::new(ctx)?;
        let player_img = Image::new(ctx,"/perso.png")?;
        let deathstar = Deathstar::new(ctx)?;
        let deathstar_img = Image::new(ctx,"/ennemie.png")?;
        let deathstar2 = Deathstar::new(ctx)?;
        let deathstar2_img = Image::new(ctx,"/ennemie.png")?;
        let deathstar3 = Deathstar::new(ctx)?;
        let deathstar3_img = Image::new(ctx,"/ennemie.png")?;
        let gravity = Vector2::new(0.0,0.007); // on défini la force de notre gravité
        let health_bar = Health::new(ctx)?;
        let health_bar_img = Image::new(ctx,"/barre1.png")?;
        let game = GameState::Start; // on initialise l'état du jeu au start
        Ok(MyGame{
            background,
            background_img,
            background2,
            background2_img,
            runner,
            player_img,
            deathstar,
            deathstar_img,
            deathstar2,
            deathstar2_img,
            deathstar3,
            deathstar3_img,
            gravity,
            health_bar,
            health_bar_img,
            game,
        })
    }
    // fonction qui va nous afficher les fps dans un coin de notre écran
    fn show_fps(&self,ctx: &mut Context) -> GameResult <()> {
        let fps_font = Font::new(ctx, "/rainyhearts.ttf")?;
        let fps = (timer::fps(ctx)).round();
        let mut fps_render = Text::new(format!("FPS: {}",fps));
        fps_render.set_font(fps_font,Scale::uniform(24.0));
        graphics::draw(ctx, &fps_render,graphics::DrawParam::default().dest(Point2::new(5.0,5.0)))
    }
    // fonction qui va nous afficher le score 
    fn show_score(&self,ctx: &mut Context)-> GameResult <()> {
        let score_font = Font::new(ctx, "/rainyhearts.ttf")?; // le score sera calculer par rapport au nombre d'obstacle qu'on franchis
        let mut score_render = Text::new(format!("{}",self.deathstar.score + self.deathstar2.score + self.deathstar3.score));
        score_render.set_font(score_font,Scale::uniform(50.0));

        graphics::draw(ctx, &score_render,graphics::DrawParam::default().dest(Point2::new(700.0,5.0)))
    }

    fn show_start(&self,ctx: &mut Context) -> GameResult <()> { // on initialise l'image du start
        let start_img = Image::new(ctx,"/start.png")?; 
        graphics::draw(ctx, &start_img,graphics::DrawParam::default().dest(Point2::new(0.0,0.0)))
    }

    fn show_restart(&self,ctx: &mut Context) -> GameResult <()> { // on initialise l'image du restart
        let restart_img = Image::new(ctx,"/restart.png")?;
        let score_font = Font::new(ctx, "/rainyhearts.ttf")?;
        let mut score_render = Text::new(format!("{}",self.deathstar.score + self.deathstar2.score + self.deathstar3.score));
        score_render.set_font(score_font,Scale::uniform(100.0));
        graphics::draw(ctx, &restart_img,graphics::DrawParam::default().dest(Point2::new(350.0,200.0)))?;
        graphics::draw(ctx, &score_render,graphics::DrawParam::default().dest(Point2::new(680.0,300.0)))
    }

}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.game{
            GameState::Start => { // si une des fleche directionelle est appuyer on passe en mode playing
                self.runner.position = Vector2::new(250.0,250.0);
                if keyboard::is_key_pressed(ctx,KeyCode::Up) || keyboard::is_key_pressed(ctx,KeyCode::Down) || keyboard::is_key_pressed(ctx,KeyCode::Left) || keyboard::is_key_pressed(ctx,KeyCode::Right) {
                    self.game = GameState::Playing
                } 
            },
            GameState::Playing =>{
                let (perimetre_x,perimetre_y) = graphics::drawable_size(ctx); // on recupere la taille de l'écran pour les fixer comme limite
                self.background.movement(); // on ajoute nos fonction au player/ennemie/background
                self.background.respawn();
                self.background2.movement2();
                self.background2.respawn2();
                self.runner.create_gravity(&self.gravity); 
                self.runner.setup_gravity();
                self.runner.limite(perimetre_y);
                self.runner.limite2(-100.0);
                self.runner.limite3(-20.0); 
                self.runner.limite4(perimetre_x);
                self.deathstar.movement();
                self.deathstar.speed();
                self.deathstar.collision(&self.runner);
                self.deathstar.respawn();
                self.deathstar2.movement();
                self.deathstar2.speed();
                self.deathstar2.collision(&self.runner);
                self.deathstar2.respawn();
                self.deathstar3.movement();
                self.deathstar3.speed();
                self.deathstar3.collision(&self.runner);
                self.deathstar3.respawn();

                // regarde si il y'a une collision avec un des 3 deathstar
                if (self.deathstar.collision(&self.runner) || self.deathstar2.collision(&self.runner) || self.deathstar3.collision(&self.runner)) == true{
                    if self.runner.life == true { // si la vie du runner est dans l'etat true 
                        self.health_bar.health = self.health_bar.health - 1; // on enleve 1 point de vie
                        self.runner.life = false; // et on met la vie du runner dans l'état false pour le rendre invulnérable
                    }
                    if self.health_bar.health == 0 { // si la vie du runner et de 0 on passe à l'état gameover
                        self.game = GameState::GameOver;
                    }
                }
                // tant qu'il n'y a pas de collision on met set la vie du joueur dans l'état true
                if (self.deathstar.collision(&self.runner) || self.deathstar2.collision(&self.runner) || self.deathstar3.collision(&self.runner)) == false{
                    self.runner.life = true;
                }
                // on change les image de la barre de vie pour chaque pv qu'il perd
                if self.health_bar.health == 2 {
                    self.health_bar_img = Image::new(ctx,"/barre2.png")?;
                }
                if self.health_bar.health == 1 {
                    self.health_bar_img = Image::new(ctx,"/barre3.png")?;
                }

                // on essaye au maximum pour pas que les deathstar apparaisent tous sur une ligne droite
                if self.deathstar.position.x > 1400.0{
                    if self.deathstar.position.y <= self.deathstar2.position.y + 100.0 || self.deathstar.position.y >= self.deathstar2.position.y - 100.0{
                        self.deathstar.position.y = rand::thread_rng().gen_range(-100.0, 700.0);
                    }
                }

                if self.deathstar3.position.x > 1400.0{
                    if self.deathstar3.position.y <= self.deathstar2.position.y + 100.0 || self.deathstar3.position.y >= self.deathstar2.position.y - 100.0{
                        self.deathstar3.position.y = rand::thread_rng().gen_range(-100.0, 700.0);
                    }
                }
                // on positione la deuxieme deathstar pour qu'elle apparaissent apres un certain temps
                self.deathstar2.position.x = 5500.0; 
                self.deathstar3.position.x = 8500.0;// pareille
                // on assigie nos fonction de déplacement au touche du clavier
                if keyboard::is_key_pressed(ctx,KeyCode::Up){ 
                    self.runner.fly();
                }

                if keyboard::is_key_pressed(ctx,KeyCode::Down){
                    self.runner.stop_fly();
                }

                if keyboard::is_key_pressed(ctx,KeyCode::Left){
                    self.runner.fly_left();
                }

                if keyboard::is_key_pressed(ctx,KeyCode::Right){
                    self.runner.fly_right();
                }
            },
            GameState::GameOver => { 
                if keyboard::is_key_pressed(ctx,KeyCode::Space){ // si on presse la barre d'espace on reinitialise tout 
                    self.deathstar.reset();                     // et on repasse en mode playing
                    self.deathstar2.reset();
                    self.deathstar3.reset();
                    self.deathstar2.position.x = 5500.0; 
                    self.deathstar3.position.x = 8500.0;
                    self.deathstar.score = 0;
                    self.deathstar2.score = 0;
                    self.deathstar3.score = 0;
                    self.runner.position = Vector2::new(250.0,250.0);
                    self.runner.speed = Vector2::new(0.0,0.0);
                    self.health_bar.health = 3;
                    self.health_bar_img = Image::new(ctx,"/barre1.png")?;
                    self.game = GameState::Playing;
                }
            },
        }
        Ok(())
    }
    


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(7, 8, 17)); // couleur du background 
        match self.game{
            GameState::Start =>{ // affichage start
                graphics::draw(
                    ctx,
                    &self.background_img,
                    graphics::DrawParam::default().dest(self.background.location()),
                )?;
                graphics::draw(
                    ctx,
                    &self.player_img,
                    graphics::DrawParam::default().dest(self.runner.location()), 
                )?;
                self.show_start(ctx)?;
            }
            GameState::Playing =>{
                graphics::draw(
                    ctx,
                    &self.background_img,
                    graphics::DrawParam::default().dest(self.background.location()), // on va dessiner notre background a la position défini
                )?;
                graphics::draw(
                    ctx,
                    &self.background2_img,
                    graphics::DrawParam::default().dest(self.background2.location2()), // meme pour background2
                )?;
                graphics::draw(
                    ctx,
                    &self.player_img,
                    graphics::DrawParam::default().dest(self.runner.location()), // meme pour le perso etc ..
                )?;
                graphics::draw( 
                    ctx,
                    &self.deathstar_img,
                    graphics::DrawParam::default().dest(self.deathstar.location()),
                )?;
                graphics::draw( 
                    ctx,
                    &self.deathstar2_img,
                    graphics::DrawParam::default().dest(self.deathstar2.location()),
                )?;
                graphics::draw( 
                    ctx,
                    &self.deathstar3_img,
                    graphics::DrawParam::default().dest(self.deathstar3.location()),
                )?;
                graphics::draw( 
                    ctx,
                    &self.health_bar_img,
                    graphics::DrawParam::default().dest(self.health_bar.location()).scale(Vector2::new(0.8, 0.5)),
                )?;
                self.show_score(ctx)?; // affichage score 
            }
            GameState::GameOver => {
                self.show_restart(ctx)?; // affichage restart
            },
        }
        self.show_fps(ctx)?; // on affiche les fps
        graphics::present(ctx)
    }

}


