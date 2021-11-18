use quicksilver::{
    geom::Vector,
    graphics::{Color, VectorFont},
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window,
};
use rand::Rng;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use std::{thread, time};


fn main() {

    // TODO: if possible, convert this input segment into gui 
    println!("welcome to the reaction & aim tester!");
    println!("Enter [R]eaction if you would like to test your reaction or [A] if you would like to test your aim.");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input = input.trim();  

    // TODO: check first first letter of input string (convert to lowercase)
    if input == "r" {
        println!("You have chosen to test your reaction time!");
        run(
            Settings {
                size: Vector { x: 800.0, y: 600.0 },
                title: "Reaction Timer",
                ..Settings::default()
            },
            app,
        );
    }
    else if input == "a" {
        // TODO: load a basic white gui, randomly placing and removing target images 
    }
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {

    gfx.clear(Color::WHITE);
    gfx.present(&window)?;
    let ttf = VectorFont::load("../static/Exo2.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 40.0)?;
    let sleep_time = rand::thread_rng().gen_range(0..5) + 3;
    thread::sleep(time::Duration::from_secs(sleep_time));
    gfx.clear(Color::GREEN);
    gfx.present(&window)?;
    let start_time = time::SystemTime::now();


    // TODO: add functionality to break when user wants to break
    let mut running  = true;
    while running {
        while let Some(event) = input.next_event().await {
            match event {
                Event::KeyboardInput(key) if key.is_down() => {
                    if key.key() == Key::Return {
                        let end_time = time::SystemTime::now();
                        let duration = end_time.duration_since(start_time).unwrap();
                        let duration_ms = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;
                        gfx.clear(Color::WHITE);
                        font.draw(
                            &mut gfx,
                            &format!("You took {} milliseconds", duration_ms),
                            Color::BLACK,
                            Vector::new(50.0, 50.0),
                        )?;
                        gfx.present(&window)?;
                        thread::sleep(time::Duration::from_secs(2));
                        println!("you took {} milliseconds", duration_ms);
                        running = false;
                    }
                }
                _ => {}
            }
        }
    }
    

    Ok(())
}
