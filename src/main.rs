//extern crate ggez_retrorunner;
use ggez::event::run;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::ContextBuilder;
use ggez_retrorunner::MyGame;
use std::path;


fn main() {
    let window_mode = WindowMode::default().dimensions(1400.0, 800.0);
    let window_setup = WindowSetup::default().title("Retrorunner2077");
    let resources_path = path::PathBuf::from("./resources");
    let (mut context, mut event_loop) = ContextBuilder::new("Retrorunner", "Jacku")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path(resources_path)
        .build()
        .expect("blop");
    //probleme de fps regler avec --release

    let mut my_game = MyGame::new(&mut context).unwrap();
    match run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}