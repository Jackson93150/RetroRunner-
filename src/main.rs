//extern crate ggez_retrorunner;
use ggez::event::run;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::ContextBuilder;
use ggez_retrorunner::MyGame;


fn main() {
    let window_mode = WindowMode::default().dimensions(1400.0, 800.0);
    let window_setup = WindowSetup::default().title("Retrorunner2077");
    let window_setup2 = WindowSetup::default().vsync(false); // depuis que j'ai mis les background les fps du jeu ont drop ce n'est plus fluide
                                                            // j'ai essayer de desactiver la vsync mais c'est la meme a corriger pour plus tard
    let (mut context, mut event_loop) = ContextBuilder::new("Retrorunner", "Jacku")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .window_setup(window_setup2)
        .build()
        .expect("blop");

    let mut my_game = MyGame::new(&mut context).unwrap();

    match run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}