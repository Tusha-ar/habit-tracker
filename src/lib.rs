use std::{
    fs::OpenOptions, io::{self, Read, Write}
};

use chrono::{DateTime, Datelike, Duration, Utc, Weekday};

const FILE_NAME: &str = "habit.txt";

#[derive(PartialEq)]
pub enum HabitFrequency {
    Daily,
    Weekly,
    Monthly,
}

pub struct Habit {
    habit: String,
    completed: bool,
    frequency: HabitFrequency,
    last_recorded_timestamp: Option<DateTime<Utc>>,
    streak: u16
}


impl Habit {
    pub fn new(habit: String, frequency: HabitFrequency) -> Habit {
        let time_now = Utc::now();
        let new_habit = Habit {
            habit,
            frequency,
            completed: false,
            streak: 0,
            last_recorded_timestamp: Some(time_now)
        };
        add_to_file(&new_habit, String::from(FILE_NAME), true);
        println!("hurray!!!!!");
        println!("Added a new habit!");
        new_habit
    }

    pub fn complete() {
        println!("Which habit you want to be marked as complete?");
        list_file(String::from(FILE_NAME));
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Invalid input");
        let habit_id: u8 = user_input.trim().parse().expect("Invalid Input");
        let mut habits = get_habits_from_file(String::from(FILE_NAME));
        for (index, habit) in habits.iter_mut().enumerate() {
            if usize::from(habit_id - 1) == index {
                if habit.completed {
                    let freq_string = match habit.frequency {
                        HabitFrequency::Daily => "this day",
                        HabitFrequency::Monthly => "this month",
                        HabitFrequency::Weekly => "this week"
                    };
                    println!("You already completed this one for {}", freq_string);
                } else {
                    habit.completed = true;
                    habit.streak += 1;
                    habit.last_recorded_timestamp = Some(Utc::now());
                }
            }
        }

        OpenOptions::new().write(true).truncate(true).open(FILE_NAME)
        .expect("Failed to clear the file.");
        for habit in habits.iter() {
            add_to_file(habit, String::from(FILE_NAME), true);
        }
    }

    pub fn list_all() {
        list_file(String::from(FILE_NAME));
    }

    pub fn init() {
        let mut habits = get_habits_from_file(String::from(FILE_NAME));
        let now_date = Utc::now();
        
        for habit in habits.iter_mut() {
            match habit.frequency {
                // Check for daily habit
                HabitFrequency::Daily => {
                    if habit.last_recorded_timestamp.unwrap().date_naive() == now_date.date_naive() {
                        continue;
                    }
                    habit.completed = false;
                    if !is_yesterday(&habit.last_recorded_timestamp.unwrap()) {
                        habit.streak = 0;
                    }
                },
                HabitFrequency::Weekly => {
                    if !(habit.last_recorded_timestamp.unwrap().iso_week() == now_date.iso_week() && 
                         habit.last_recorded_timestamp.unwrap().year() == now_date.year()) {
                        habit.completed = false;
                    }
                    
                    if is_older_than_last_full_week(&habit.last_recorded_timestamp.unwrap()) {
                        habit.streak = 0;
                    }
                },
                HabitFrequency::Monthly => {
                    if !(habit.last_recorded_timestamp.unwrap().year() == now_date.year() &&
                         habit.last_recorded_timestamp.unwrap().month() == now_date.month()) {
                        habit.completed = false;
                    }
    
                    if is_older_than_last_full_month(&habit.last_recorded_timestamp.unwrap()) {
                        habit.streak = 0;
                    }
                }
            }
        }

        OpenOptions::new().write(true).truncate(true).open(FILE_NAME)
        .expect("Failed to clear the file.");
        for habit in habits.iter() {
            add_to_file(habit, String::from(FILE_NAME), true);
        }
    }
}

fn add_to_file(new_habit: &Habit, file_name: String, append: bool) {
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .read(true)
    .append(append)
    .open(file_name)
    .expect("There was a problem while adding the task to db");
    let frequency = match new_habit.frequency {
        HabitFrequency::Daily => "daily",
        HabitFrequency::Weekly => "weekly",
        HabitFrequency::Monthly => "monthly",
    };

    let last_recorded_timestamp = match new_habit.last_recorded_timestamp {
        Some(timestamp) => timestamp.to_rfc3339(),
        None => String::from("N/A"),
    };
    let content = format!("{}/{}/{}/{}/{}\n", 
    new_habit.habit, 
    new_habit.completed, 
    frequency, 
    last_recorded_timestamp, 
    new_habit.streak);
    file.write_all(content.to_string().as_bytes()).expect("Error while adding the habit in DB. Please try again");
}

fn list_file(file_name: String) {
    let mut file = OpenOptions::new().read(true).open(file_name).expect("Error while reading the db");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error while reading the DB");
    for (index, habit_content) in content.lines().into_iter().enumerate() {
        let h: Vec<&str> = habit_content.split("/").collect();
        if h.len() == 5 {
            let habit = h[0];
            let freq = h[2];
            let streak = h[4];
            println!("{}: {} - {}   {}🔥", index + 1, habit, freq,  streak);
        }
    } 
}

fn get_habits_from_file(file_name: String) -> Vec<Habit> {
    let mut file = OpenOptions::new().read(true).open(file_name).expect("Error while opening the db");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error while reading the db");
    let mut habits: Vec<Habit> = vec![];
    for (_, habit_content) in content.lines().into_iter().enumerate() {
        let h: Vec<&str> = habit_content.split("/").collect();
        if h.len() == 5 {
            let habit = h[0];
            let completed = h[1];
            let completed = match completed.trim() {
                "false" => false,
                "true" => true,
                _ => false
            };
            let frequency = h[2];
            let frequency  = match frequency {
                "daily" => HabitFrequency::Daily,
                "monthly" => HabitFrequency::Monthly,
                "weekly" => HabitFrequency::Weekly,
                _ => HabitFrequency::Daily
            };
            let last_recorded_timestamp = h[3];
            let last_recorded_timestamp = DateTime::parse_from_rfc3339(last_recorded_timestamp).unwrap().with_timezone(&Utc);
            let streak = h[4].parse().unwrap();
            habits.push(Habit {
                completed,
                frequency,
                habit: String::from(habit),
                last_recorded_timestamp: Some(last_recorded_timestamp),
                streak
            });
        }
    } 
    habits

}

fn is_yesterday(date: &chrono::DateTime<Utc>) -> bool {
    let today = Utc::now().date_naive(); // Get today's date
    let yesterday = today - Duration::days(1); // Subtract one day for yesterday

    date.date_naive() == yesterday // Compare the dates
}

fn is_older_than_last_full_week(date: &chrono::DateTime<Utc>) -> bool {
    let today = Utc::now();
    
    // Find the number of days since the last Monday
    let days_since_monday = today.weekday().num_days_from_monday();
    
    // Start of the current week (Monday)
    let current_week_start = today - Duration::days(days_since_monday as i64);
    
    // Start of the last full week (the Monday before the current week)
    let last_full_week_start = current_week_start - Duration::weeks(1);
    
    // Check if the date is before the start of the last full week
    date < &last_full_week_start // Directly compare DateTime
}


fn is_older_than_last_full_month(date: &chrono::DateTime<Utc>) -> bool {
    let today = Utc::now();
    
    // Get the first day of the current month
    let first_day_of_current_month = today.with_day(1).unwrap();
    
    // Get the first day of the last month
    let first_day_of_last_month = if today.month() == 1 {
        // If the current month is January, go back to December of last year
        first_day_of_current_month.with_year(today.year() - 1).unwrap()
            .with_month(12).unwrap()
    } else {
        first_day_of_current_month.with_month(today.month() - 1).unwrap()
    };
    
    // Check if the date is before the first day of the last month
    date < &first_day_of_last_month // Directly compare DateTime
}