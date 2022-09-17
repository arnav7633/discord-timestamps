
use chrono::{Local, TimeZone};
use std::{process, io};
fn main() {
    println!("Select format of timestamp \n1 - Only time \n2 - Date and time");
    let mut choice= String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: i32 = choice.trim().parse().expect("Please type a number!");

    println!("Enter the hour IN 24 HOUR format");
    let mut hours = String::new();
    io::stdin().read_line(&mut hours).expect("Failed to read hours");
    let hours: u32 = hours.trim().parse().expect("Hours is not a number");
    if hours > 24 {
        println!("Hours cannot be greater than 24");
        process::exit(0)
    }

    let mut mins = String::new();
    println!("Enter minutes");
    io::stdin().read_line(&mut mins).expect("Failed to read minutes");
    let mins: u32 = mins.trim().parse().expect("Minutes must be a number");
    if mins > 60 {
        println!("Minutes cannot be greater than 60");
        process::exit(0)
    }

    if choice == 1{
        let timestamp = Local.ymd(2017, 11, 2).and_hms(hours, mins, 00);
      println!("Your timestamp is \n <t:{}:t> and norml time is{}", timestamp.timestamp(), timestamp)
    };

}
