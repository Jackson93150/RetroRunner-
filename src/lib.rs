use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Font, Image, Scale, Text};
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, timer, Context, GameResult};
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
mod tir;
use tir::Tir;
mod bonus;
use bonus::Bonus;

pub enum GameState {
    // Structure pour déterminer l'état du jeu
    Start,
    Playing,
    GameOver,
}

pub struct MyGame {
    background: Background,
    background_img: Image,
    background2: Background,
    background2_img: Image,
    runner: Player,
    player_img: Image,
    deathstar: Deathstar,
    deathstar_img: Image,
    deathstar2: Deathstar,
    deathstar2_img: Image,
    deathstar3: Deathstar,
    deathstar3_img: Image,
    gravity: Vector2<f32>,
    health_bar: Health,
    health_bar_img: Image,
    game: GameState,
    sound: audio::Source,
    sound2: audio::Source,
    sound3: audio::Source,
    sound4: audio::Source,
    sound5: audio::Source,
    sound6: audio::Source,
    sound7: audio::Source,
    music: audio::Source,
    tir: Tir,
    tir_img: Image,
    sound_img: Image,
    bonus: Bonus,
    bonus_img: Image,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let background = Background::new(ctx)?;
        let background_img = Image::new(ctx, "/background.png")?; //j'ai du compresser mon background pour gagner en fluidité
        let background2 = Background::new(ctx)?;
        let background2_img = Image::new(ctx, "/background.png")?;
        let runner = Player::new(ctx)?;
        let player_img = Image::new(ctx, "/perso.png")?;
        let deathstar = Deathstar::new(ctx)?;
        let deathstar_img = Image::new(ctx, "/ennemie.png")?;
        let deathstar2 = Deathstar::new(ctx)?;
        let deathstar2_img = Image::new(ctx, "/ennemie.png")?;
        let deathstar3 = Deathstar::new(ctx)?;
        let deathstar3_img = Image::new(ctx, "/ennemie.png")?;
        let gravity = Vector2::new(0.0, 0.007); // on défini la force de notre gravité
        let health_bar = Health::new(ctx)?;
        let health_bar_img = Image::new(ctx, "/barre1.png")?;
        let game = GameState::Start; // on initialise l'état du jeu au start
        let sound = audio::Source::new(ctx, "/BOOM.wav")?;
        let sound2 = audio::Source::new(ctx, "/SKRAAA.wav")?;
        let sound3 = audio::Source::new(ctx, "/SKYAA.wav")?;
        let sound4 = audio::Source::new(ctx, "/poom.ogg")?;
        let sound5 = audio::Source::new(ctx, "/yeboi.ogg")?;
        let sound6 = audio::Source::new(ctx, "/wait.ogg")?;
        let sound7 = audio::Source::new(ctx, "/GOTEM.ogg")?;
        let music = audio::Source::new(ctx, "/fond.ogg")?;
        let tir = Tir::new(ctx)?;
        let tir_img = Image::new(ctx, "/tir.png")?;
        let sound_img = Image::new(ctx, "/son.png")?;
        let bonus = Bonus::new(ctx)?;
        let bonus_img = Image::new(ctx, "/bonus.png")?;
        Ok(MyGame {
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
            sound,
            sound2,
            sound3,
            sound4,
            sound5,
            sound6,
            sound7,
            music,
            tir,
            tir_img,
            sound_img,
            bonus,
            bonus_img,
        })
    }
    // fonction qui va nous afficher les fps dans un coin de notre écran
    fn show_fps(&self, ctx: &mut Context) -> GameResult<()> {
        let fps_font = Font::new(ctx, "/rainyhearts.ttf")?;
        let fps = (timer::fps(ctx)).round();
        let mut fps_render = Text::new(format!("FPS: {}", fps));
        fps_render.set_font(fps_font, Scale::uniform(24.0));
        graphics::draw(
            ctx,
            &fps_render,
            graphics::DrawParam::default().dest(Point2::new(5.0, 5.0)),
        )
    }
    // fonction qui va nous afficher le score
    fn show_score(&self, ctx: &mut Context) -> GameResult<()> {
        let score_font = Font::new(ctx, "/rainyhearts.ttf")?; // le score sera calculer par rapport au nombre d'obstacle qu'on franchis
        let mut score_render = Text::new(format!(
            "{}",
            self.deathstar.score + self.deathstar2.score + self.deathstar3.score
        ));
        score_render.set_font(score_font, Scale::uniform(50.0));

        graphics::draw(
            ctx,
            &score_render,
            graphics::DrawParam::default().dest(Point2::new(700.0, 5.0)),
        )
    }

    fn show_start(&self, ctx: &mut Context) -> GameResult<()> {
        // on initialise l'image du start
        let start_img = Image::new(ctx, "/start.png")?;
        graphics::draw(
            ctx,
            &start_img,
            graphics::DrawParam::default().dest(Point2::new(0.0, 0.0)),
        )
    }

    fn show_restart(&self, ctx: &mut Context) -> GameResult<()> {
        // on initialise l'image du restart
        let restart_img = Image::new(ctx, "/restart.png")?;
        let score_font = Font::new(ctx, "/rainyhearts.ttf")?;
        let mut score_render = Text::new(format!(
            "{}",
            self.deathstar.score + self.deathstar2.score + self.deathstar3.score
        ));
        score_render.set_font(score_font, Scale::uniform(100.0));
        graphics::draw(
            ctx,
            &restart_img,
            graphics::DrawParam::default().dest(Point2::new(350.0, 200.0)),
        )?;
        graphics::draw(
            ctx,
            &score_render,
            graphics::DrawParam::default().dest(Point2::new(650.0, 300.0)),
        )
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, KeyCode::F2) {
            // on baisse le volume de tous nos sons quand on appuie sur une touche
            let rst = self.music.volume();
            if rst > 0.00 {
                self.music.set_volume(rst - 0.01);
                self.sound.set_volume(rst - 0.01);
                self.sound2.set_volume(rst - 0.01);
                self.sound3.set_volume(rst - 0.01);
                self.sound4.set_volume(rst - 0.01);
                self.sound5.set_volume(rst - 0.01);
                self.sound6.set_volume(rst - 0.01);
                self.sound7.set_volume(rst - 0.01);
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::F3) {
            // on augmente le volume de tous nos sons quand on appuie sur une touche
            let rst = self.music.volume();
            if rst < 1.00 {
                self.music.set_volume(rst + 0.01);
                self.sound.set_volume(rst + 0.01);
                self.sound2.set_volume(rst + 0.01);
                self.sound3.set_volume(rst + 0.01);
                self.sound4.set_volume(rst + 0.01);
                self.sound5.set_volume(rst + 0.01);
                self.sound6.set_volume(rst + 0.01);
                self.sound7.set_volume(rst + 0.01);
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::F1) {
            // on éteint le volume de tous nos sons quand on appuie sur une touche
            self.music.set_volume(0.0);
            self.sound.set_volume(0.0);
            self.sound2.set_volume(0.0);
            self.sound3.set_volume(0.0);
            self.sound4.set_volume(0.0);
            self.sound5.set_volume(0.0);
            self.sound6.set_volume(0.0);
            self.sound7.set_volume(0.0);
        }
        match self.game {
            GameState::Start => {
                // si une des fleche directionelle est appuyer on passe en mode playing
                self.runner.position = Vector2::new(250.0, 250.0);
                if keyboard::is_key_pressed(ctx, KeyCode::Up)
                    || keyboard::is_key_pressed(ctx, KeyCode::Down)
                    || keyboard::is_key_pressed(ctx, KeyCode::Left)
                    || keyboard::is_key_pressed(ctx, KeyCode::Right)
                {
                    self.game = GameState::Playing
                }
            }
            GameState::Playing => {
                ggez::timer::check_update_time(ctx, 120);
                self.background.movement(); // on ajoute nos fonction au player/ennemie/background
                self.background.respawn();
                self.background2.movement();
                self.background2.respawn();
                self.runner.create_gravity(&self.gravity);
                self.runner.setup_gravity();
                self.runner.limite(800.0);
                self.runner.limite2(-100.0);
                self.runner.limite3(-20.0);
                self.runner.limite4(1400.0);
                self.deathstar.movement();
                self.deathstar.speed();
                self.deathstar.collision(&self.runner);
                self.deathstar.respawn();
                self.deathstar.tir_hit(&self.tir);
                self.deathstar2.movement();
                self.deathstar2.speed();
                self.deathstar2.collision(&self.runner);
                self.deathstar2.respawn();
                self.deathstar2.tir_hit(&self.tir);
                self.deathstar3.movement();
                self.deathstar3.speed();
                self.deathstar3.collision(&self.runner);
                self.deathstar3.respawn();
                self.deathstar3.tir_hit(&self.tir);
                self.tir.movement();
                self.tir.respawn();
                self.bonus.movement();
                self.bonus.respawn();

                // on regarde si le joueur et rentrer dans la hitbox du bonus
                if self.bonus.position.x <= self.runner.position.x
                    && self.bonus.position.x + 30.0 >= self.runner.position.x
                    && self.bonus.position.y <= self.runner.position.y + 50.0
                    && self.bonus.position.y + 30.0 >= self.runner.position.y
                {
                    let x = rand::thread_rng().gen_range(1, 4); // on defini une variable qui va choisir une valeur entre 1 et 7
                    if x == 3 {
                        // pour chacune des valeur on va lancer un son différent et va donner un score bonus different
                        self.sound5.play().unwrap();
                        self.deathstar.score += 15;
                    }
                    if x == 2 {
                        self.sound7.play().unwrap();
                        self.deathstar.score += 10;
                    }
                    if x == 1 {
                        self.sound6.play().unwrap();
                        self.deathstar.score += 5;
                    }
                    self.bonus.state = true; // on repasse a l'état true du bonus
                }

                if self.tir.state == false {
                    // si le tir est dans l'état false on le repositione au niveau de runner
                    self.tir.position.x = self.runner.position.x + 10.0;
                    self.tir.position.y = self.runner.position.y + 35.0;
                }

                if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                    // qaund on appuie sur espace le tir est lancer
                    if self.tir.state == false {
                        // on verifie que le tir est dans l'état false
                        if self.sound4.playing() == false {
                            // mais aussi que le son du tir est bien fini
                            self.sound4.play().unwrap(); // on lance le son
                            self.tir.state = true; // et on passe le tir en état true
                        }
                    }
                }
                // si l'état de un des deathstar est true le tir passe en état false
                if (self.deathstar.state || self.deathstar2.state || self.deathstar3.state) == true
                {
                    self.tir.state = false;
                }

                // regarde si il y'a une collision avec un des 3 deathstar
                if (self.deathstar.collision(&self.runner)
                    || self.deathstar2.collision(&self.runner)
                    || self.deathstar3.collision(&self.runner))
                    == true
                {
                    if self.runner.life == true {
                        // si la vie du runner est dans l'etat true
                        if self.deathstar.collision(&self.runner) == true {
                            self.sound.play().unwrap();
                        }
                        if self.deathstar2.collision(&self.runner) == true {
                            self.sound2.play().unwrap();
                        }
                        if self.deathstar3.collision(&self.runner) == true {
                            self.sound3.play().unwrap();
                        }
                        self.health_bar.health = self.health_bar.health - 1; // on enleve 1 point de vie
                        self.runner.life = false; // et on met la vie du runner dans l'état false pour le rendre invulnérable
                    }
                    if self.health_bar.health == 0 {
                        // si la vie du runner et de 0 on passe à l'état gameover
                        self.game = GameState::GameOver;
                    }
                }
                // tant qu'il n'y a pas de collision on met set la vie du joueur dans l'état true
                if (self.deathstar.collision(&self.runner)
                    || self.deathstar2.collision(&self.runner)
                    || self.deathstar3.collision(&self.runner))
                    == false
                {
                    self.runner.life = true;
                }
                // on change les image de la barre de vie pour chaque pv qu'il perd
                if self.health_bar.health == 2 {
                    self.health_bar_img = Image::new(ctx, "/barre2.png")?;
                }
                if self.health_bar.health == 1 {
                    self.health_bar_img = Image::new(ctx, "/barre3.png")?;
                }

                // on essaye au maximum pour pas que les deathstar apparaisent tous sur une ligne droite
                if self.deathstar.position.x > 1400.0 {
                    if self.deathstar.position.y <= self.deathstar2.position.y + 100.0
                        || self.deathstar.position.y >= self.deathstar2.position.y - 100.0
                    {
                        self.deathstar.position.y = rand::thread_rng().gen_range(-100.0, 700.0);
                    }
                }

                if self.deathstar3.position.x > 1400.0 {
                    if self.deathstar3.position.y <= self.deathstar2.position.y + 100.0
                        || self.deathstar3.position.y >= self.deathstar2.position.y - 100.0
                    {
                        self.deathstar3.position.y = rand::thread_rng().gen_range(-100.0, 700.0);
                    }
                }
                if self.background2.position.x == self.background.position.x {
                    self.background2.position.x = 2560.0;
                }

                // on positione la deuxieme deathstar pour qu'elle apparaissent apres un certain temps
                if self.deathstar2.position.x == self.deathstar.position.x {
                    self.deathstar2.position.x = 5500.0;
                }

                if self.deathstar3.position.x == self.deathstar.position.x {
                    // pareille
                    self.deathstar3.position.x = 8500.0;
                }

                // on assigie nos fonction de déplacement au touche du clavier
                if keyboard::is_key_pressed(ctx, KeyCode::Up) {
                    self.runner.fly();
                }

                if keyboard::is_key_pressed(ctx, KeyCode::Down) {
                    self.runner.stop_fly();
                }

                if keyboard::is_key_pressed(ctx, KeyCode::Left) {
                    self.runner.fly_left();
                }

                if keyboard::is_key_pressed(ctx, KeyCode::Right) {
                    self.runner.fly_right();
                }
            }
            GameState::GameOver => {
                if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                    // si on presse la barre d'espace on reinitialise tout
                    self.deathstar.reset(); // et on repasse en mode playing
                    self.deathstar2.reset();
                    self.deathstar3.reset();
                    self.deathstar2.position.x = 5500.0;
                    self.deathstar3.position.x = 8500.0;
                    self.deathstar.score = 0;
                    self.deathstar2.score = 0;
                    self.deathstar3.score = 0;
                    self.runner.reset();
                    self.health_bar.health = 3;
                    self.health_bar_img = Image::new(ctx, "/barre1.png")?;
                    self.game = GameState::Playing;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(7, 8, 17)); // couleur du background
        match self.game {
            GameState::Start => {
                // affichage start
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
            GameState::Playing => {
                graphics::draw(
                    ctx,
                    &self.background_img,
                    graphics::DrawParam::default().dest(self.background.location()), // on va dessiner notre background a la position défini
                )?;
                graphics::draw(
                    ctx,
                    &self.background2_img,
                    graphics::DrawParam::default().dest(self.background2.location()), // meme pour background2
                )?;
                graphics::draw(
                    ctx,
                    &self.tir_img,
                    graphics::DrawParam::default().dest(self.tir.location()),
                )?;
                graphics::draw(
                    ctx,
                    &self.player_img,
                    graphics::DrawParam::default().dest(self.runner.location()), // meme pour le perso etc ..
                )?;
                graphics::draw(
                    ctx,
                    &self.bonus_img,
                    graphics::DrawParam::default().dest(self.bonus.location()),
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
                    graphics::DrawParam::default()
                        .dest(self.health_bar.location())
                        .scale(Vector2::new(0.8, 0.5)),
                )?;
                self.show_score(ctx)?; // affichage score
            }
            GameState::GameOver => {
                self.show_restart(ctx)?; // affichage restart
            }
        }
        graphics::draw(
            ctx,
            &self.sound_img,
            graphics::DrawParam::default().dest(Point2::new(0.0, 700.0)),
        )?;
        if self.music.playing() == false {
            // si la music de fond et terminer la releance
            self.music.play().unwrap();
        }
        self.show_fps(ctx)?; // on affiche les fps
        graphics::present(ctx)
    }
}
