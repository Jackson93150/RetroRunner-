use ggez::{event,Context,GameResult,graphics,timer};
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
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
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
                self.runner.life = false // et on met la vie du runner dans l'état false pour le rendre invulnérable
            }
            if self.health_bar.health == 0 { // si la vie du runner et de 0 on quit le jeu
                event::quit(ctx);
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
        if self.deathstar2.position.x == self.deathstar.position.x{ 
            self.deathstar2.position.x = 5500.0;
        }

        if self.deathstar3.position.x == self.deathstar.position.x{ // pareille
            self.deathstar3.position.x = 8500.0;
        }
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

        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(7, 8, 17)); // couleur du background 
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
        self.show_fps(ctx)?; // on affiche les fps
        graphics::present(ctx)
    }

}


