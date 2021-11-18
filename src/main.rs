// use std::{thread, time};
// use rand::Rng;
// use std::io;
// use std::thread::sleep;
// use std::time::Duration;
use quicksilver::{
    geom::Vector,
    graphics::{Color, VectorFont},
    run, Graphics, Input, Result, Settings, Window,
};

const SIZE: Vector = Vector { x: 500.0, y: 500.0 };

fn main() {
    run(
        Settings {
            size: SIZE,
            title: "Square Example",
            ..Settings::default()
        },
        app,
    );

    //  println!("welcome to the reaction time tester!");
    //  println!("When you see the word {}, press enter", "GO!");
    // let sleep_time = rand::thread_rng().gen_range(0..5) + 5;
    // thread::sleep(time::Duration::from_secs(sleep_time));
    // let start_time = time::SystemTime::now();
    // println!("GO!");

    // let mut input = String::new();

    // io::stdin().read_line(&mut input);
    // let end_time = time::SystemTime::now();
    // let duration = end_time.duration_since(start_time).unwrap();

    // let duration_ms = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;
    // println!("you took {} milliseconds", duration_ms);
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);
    let ttf = VectorFont::load("../static/Exo2.ttf").await?;
    // Paint a blue square with a red outline in the center of our screen
    // It should have a top-left of (350, 100) and a size of (150, 100)
    // gfx.
    let mut font = ttf.to_renderer(&gfx, 40.0)?;
    gfx.clear(Color::WHITE);
    // Use the font rendering API to draw some text
    font.draw(
        &mut gfx,
        "Hello world!\nHello Quicksilver!",
        Color::BLACK,
        Vector::new(50.0, 50.0),
    )?;
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }

    // let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
    // gfx.fill_rect(&rect, Color::BLUE);
    // gfx.stroke_rect(&rect, Color::RED);
    // // Send the data to be drawn
    // gfx.present(&window)?;
    // loop {
    //     while let Some(_) = input.next_event().await {}
    // }
}
