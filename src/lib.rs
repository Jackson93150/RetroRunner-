use ggez::{Context,GameResult,graphics};
use ggez::graphics::Image;
use ggez::event::EventHandler;
use ggez::nalgebra::Vector2;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
mod player;
use player::Player;

pub struct MyGame {
    runner: Player,
    player_img : Image,
    gravity : Vector2<f32>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let runner = Player::new(ctx)?;
        let player_img = Image::new(ctx,"/perso.png")?;
        let gravity = Vector2::new(0.0,0.007); // on défini la force de notre gravité
        Ok(MyGame{
            runner,
            player_img,
            gravity,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (perimetre_x,perimetre_y) = graphics::drawable_size(ctx); // on recupere la taille de l'écran pour les fixer comme limite
        self.runner.create_gravity(&self.gravity); // on ajoute nos fonction au player
        self.runner.setup_gravity();
        self.runner.limite(perimetre_y);
        self.runner.limite2(-100.0);
        self.runner.limite3(-20.0); 
        self.runner.limite4(perimetre_x);
        if keyboard::is_key_pressed(ctx,KeyCode::Up){ // on assigie nos fonction de déplacement au touche du clavier
            self.runner.fly();
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
        graphics::clear(ctx, graphics::Color::from_rgb(53, 10, 49)); // couleur du background 
        graphics::draw(
            ctx,
            &self.player_img,
            graphics::DrawParam::default().dest(self.runner.location()), // on va dessiner notre perso a la position défini
        )?;
        graphics::present(ctx)
    }
}