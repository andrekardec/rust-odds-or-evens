use std::io;
use std::thread::sleep;
use std::time::Duration;
use spinners::{Spinner, Spinners};

fn read_input(player: u8) -> u32 {
    loop {
        let mut input = String::new();

        println!("🏁 Player #{}, give your best shot!", player);

        io::stdin()
            .read_line(&mut input)
            .expect("💀 Failed to read line");
        
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // there is no need to test input >= 0 because type is unsigned
        // if -1, e.g., it would be catch within line 14, Err(_) => continue,

        match input <= 10 {
            true => {
                return input;
            },
            false => {
                println!("👐 You only have 10 fingers. Try again!");
                continue;
            }
        }
    }
}

fn play_round() {
    println!("\n##### Odds or Evens ######");
    println!("Decide who plays odds who plays evens");
    println!("\n💥 One, two, three shoot!");

    let first: u32 = read_input(1);
    let second: u32 = read_input(2);

    let sum_up: u32 = first + second;    
    let result = match sum_up % 2 == 0 {
        true => "Evens player wins the round!",
        false => "Odds player wins the round!",
    };

    let mut spinner = Spinner::new(Spinners::Dots9, "Calculating result...".into());
    sleep(Duration::from_secs(2));
    println!("\n\n###############################");
    spinner.stop_and_persist("🥁", result.into());
    println!("###############################");
}

fn main() {
    loop {
        play_round();
    }
}
