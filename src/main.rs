use std::{thread, time};
use rand::Rng;
use std::io;

fn main() {
     println!("welcome to the reaction time tester!");
     println!("When you see the word {}, press enter", "GO!");
    let sleep_time = rand::thread_rng().gen_range(0..5) + 5;   
    thread::sleep(time::Duration::from_secs(sleep_time));
    let start_time = time::SystemTime::now();
    println!("GO!");

    let mut input = String::new();

    io::stdin().read_line(&mut input);
    let end_time = time::SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    

    let duration_ms = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;
    println!("you took {} milliseconds", duration_ms);
}