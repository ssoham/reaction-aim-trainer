use quicksilver::{
    geom::{Circle, Vector},
    graphics::{Color, VectorFont},
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window,
};

use rand::Rng;
use std::sync::mpsc;
use std::{thread, time};


fn main() {
    run (
        Settings {
            title: "Reaction & Aim Trainer",
            size: Vector{x: 800.0, y: 600.0},
            resizable: false,
            ..Settings::default()
        },
        home,
    );
}

async fn home(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mut running = true;
    while running {
        gfx.clear(Color::WHITE);
        let ttf = VectorFont::load("../static/Exo2.ttf").await.unwrap();
        let mut font = ttf.to_renderer(&gfx, 32.0)?;
    
        font.draw_wrapping(
            &mut gfx,
            "Welcome to the reaction & aim tester! \n Press [R] to test reaction time \n Press [A] to test aim time, and [ESC] to quit.",
            Some(500.0),
            Color::BLACK,
            Vector::new(100.0, 300.0),
        )?;

        gfx.present(&window)?;
        thread::sleep(time::Duration::from_millis(2000));

        while let Some(event) = input.next_event().await {
            match event {
                Event::KeyboardInput(key) => {
                    if key.key() == Key::Escape {
                        running = false;
                    }
                    if key.key() == Key::R {
                        println!("You have chosen to test your reaction time!");
                        reaction_time(&window, &mut gfx, &mut input).await?;
                        break;
                    }
                    if key.key() == Key::A {
                        println!("You have chosen to test your aim!");
                        aim_trainer(&window, &mut gfx, &mut input).await?;
                        break;
                    }
                }
                _ => {}
            }
        } 
    }
    Ok(())
}

async fn reaction_time(window: &Window, gfx: &mut Graphics, input: &mut Input) -> Result<()> {
    gfx.clear(Color::WHITE);

    let ttf = VectorFont::load("Exo2.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 40.0)?;
    font.draw_wrapping(
        gfx,
        "Press [SPACE] when the screen turns green!",
        Some(500.0),
        Color::BLACK,
        Vector::new(100.0, 300.0),
    )?;
    gfx.present(&window)?;
    
    let (send, _recv) = mpsc::channel();
    let _timer = thread::spawn(move || {
        let sleep_time = rand::thread_rng().gen_range(0..5) *2;
        thread::sleep(time::Duration::from_secs(sleep_time));
        send.send(true).unwrap();
    });

    gfx.clear(Color::GREEN);
    gfx.present(&window)?;
    let start_time = time::SystemTime::now();
    
    let mut running = true;
    while running {
        while let Some(event) = input.next_event().await {
            match event {
                Event::KeyboardInput(key) => {
                    if key.key() == Key::Space {
                        running = false;
                        let end_time = time::SystemTime::now();
                        let duration = end_time.duration_since(start_time).unwrap();
                        let duration_ms = duration.as_millis();
                        
                        gfx.clear(Color::WHITE);
                        println!("Your reaction time was {}ms", duration_ms);
                        font.draw_wrapping(
                            gfx,
                            &format!("Your reaction time was {}ms", duration_ms),
                            Some(500.0),
                            Color::BLACK,
                            Vector::new(170.0, 300.0),
                        )?;
                        gfx.present(&window)?;
                        thread::sleep(time::Duration::from_secs(2));
                    }
                }
                _ => {}
            }
        }
    }
        
    Ok(())
}

async fn aim_trainer(window: &Window, gfx: &mut Graphics, input: &mut Input) -> Result<()> {
    let mut rand_pos = Vector::new(0.0, 0.0);
    let mut target_exists = false;
    let mut count = 0;
    let mut average_time = 0.0;
    let mut start_time = time::SystemTime::now();
    let mut last_time = 0.0;
    let ttf = VectorFont::load("Exo2.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 40.0)?;
    
    loop {
        while let Some(_) = input.next_event().await {}
        gfx.clear(Color::WHITE);
        if !target_exists {
            rand_pos = Vector::new(
                rand::thread_rng().gen_range(50..750) as f32,
                rand::thread_rng().gen_range(50..550) as f32,
            );
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
            let end_time = time::SystemTime::now();
            println!(
                "{}",
                end_time.duration_since(start_time).unwrap().as_millis()
            );
            last_time = end_time.duration_since(start_time).unwrap().as_millis() as f32;
            average_time += end_time.duration_since(start_time).unwrap().as_millis() as f32;
            font.draw(
                gfx,
                &format!("target hit in {}ms\naverage time is {}ms", last_time, average_time / (count as f32)),
                Color::BLACK,
                Vector::new(100.0, 100.0),
            )?;
            println!("{}", count);
        }
        gfx.fill_circle(&Circle::new(mouse, 20.0), Color::RED);
        font.draw(
            gfx,
            &format!("target hit in {}ms\naverage time is {}ms", last_time, average_time / (count as f32)),
            Color::BLACK,
            Vector::new(100.0, 100.0),
        )?;
        if count == 10 {
            
            average_time = average_time / 10.0;
            gfx.clear(Color::WHITE);

            font.draw(
                gfx,
                &format!("Your average aim time was {}ms", average_time),
                Color::BLACK,
                Vector::new(100.0, 100.0),
            )?;
            gfx.present(&window)?;

            thread::sleep(time::Duration::from_secs(3));
            break;
        }
        gfx.present(&window)?;
    }

    Ok(())
}
