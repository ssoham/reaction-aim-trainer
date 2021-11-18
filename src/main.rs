use quicksilver::{
    geom::{Circle, Vector},
    graphics::{Color, VectorFont},
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window,
};

use std:: {io, thread, time};
use rand::Rng;


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
            reaction_time,
        );
    }
    else if input == "a" {
        println!("You have chosen to test your aim!");
        run(
            Settings {
                size: Vector { x: 800.0, y: 600.0 },
                title: "Aim trainer",
                ..Settings::default()
            },
            aim_trainer,
        );
    }
    else {
        println!("You have entered an invalid input. Please try again.");
        main();
    }
}

async fn reaction_time(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {

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

async fn aim_trainer(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mut rand_pos = Vector::new(0.0, 0.0);
    let mut target_exists = false;
    let mut count = 0;
    let mut average_time = 0.0;
    let mut start_time = time::SystemTime::now();
    let mut end_time = time::SystemTime::now();
    loop {
        while let Some(_) = input.next_event().await {}
        gfx.clear(Color::WHITE);
        if !target_exists {
            rand_pos = Vector::new(rand::thread_rng().gen_range(50..750) as f32, rand::thread_rng().gen_range(50..550) as f32);
            gfx.fill_circle(&Circle::new(rand_pos, 20.0), Color::RED);
            target_exists = true;
            count = count + 1;
            start_time = time::SystemTime::now();
        } else {
            gfx.fill_circle(&Circle::new(rand_pos, 20.0), Color::RED);
        }
        let mouse = gfx.screen_to_camera(&window, input.mouse().location());
        if mouse.distance(rand_pos) < 10.0 {
            target_exists = false;
            gfx.fill_circle(&Circle::new(rand_pos, 20.0), Color::WHITE);
            end_time = time::SystemTime::now();
            println!("{}", end_time.duration_since(start_time).unwrap().as_millis());
            average_time += end_time.duration_since(start_time).unwrap().as_millis() as f32;
        }
        gfx.fill_circle(&Circle::new(mouse, 20.0), Color::RED);
        if count == 10 {
            average_time = average_time / 10.0;
            println!("Average reaction time: {} ms", average_time);
            break;
        }
        gfx.present(&window)?;
    }
    Ok(())
}