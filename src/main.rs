use habit_tracker::{Habit, HabitFrequency};

fn main() {
    Habit::init();
    loop {
        println!("Hey there!");
        println!("Welcome to habit tracker.....");
        println!("1. Add a new habit");
        println!("2. Mark a habit as complete");
        println!("3. Show complete progress");
        println!("4. List all the habits");
        println!("exit => to exit the program");
        println!("\nInput: ");
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("invalid input");
        if input.trim() == "exit" {
            println!("bye bye!!");
            break;
        }
        let number: u8 = match input.trim().parse(){
            Ok(i) => i,
            Err(_) => {
                eprintln!("Invalid Input");
                continue;
            }
        };

        match number {
            1 => {
                println!("Enter a new HABIT!!!");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Enter a valid habit");

                println!("How frequently you plan to do this habit");
                println!("1. Daily");
                println!("2. Weekly");
                println!("3. Monthly");
                let mut frequency_input = String::new();
                std::io::stdin()
                    .read_line(&mut frequency_input)
                    .expect("Invalid Input");
                let frequency_input: u8 = match frequency_input.trim().parse(){
                    Ok(i) => i,
                    Err(_) => {
                        eprintln!("Invalid Input");
                        continue;
                    }
                };
                let frequency: HabitFrequency = match frequency_input {
                    1 => HabitFrequency::Daily,
                    2 => HabitFrequency::Weekly,
                    3 => HabitFrequency::Monthly,
                    _ => {
                        eprintln!("Enter a valid number");
                        continue;
                    }
                };
                Habit::new(input.trim().parse().unwrap(), frequency);
                println!("\nPress Enter to continue.");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Invalid Input");
                if input == "\n" {
                    continue;
                }
            }
            2 => {
                Habit::complete();
                println!("\nPress Enter to continue.");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Invalid Input");
                if input == "\n" {
                    continue;
                }
            }
            3 => {
                Habit::track_progress();
                println!("\nPress Enter to continue.");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Invalid Input");
                if input == "\n" {
                    continue;
                }
            }
            4 => {
                Habit::list_all();
                println!("\nPress Enter to continue.");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Invalid Input");
                if input == "\n" {
                    continue;
                }
            }
            _ => println!("default"),
        }
    }
}
