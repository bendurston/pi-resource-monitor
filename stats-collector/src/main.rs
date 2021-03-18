use std::process::{Command, Stdio};
use std::{thread, time};
use chrono::{Local, DateTime};
use reqwest::{blocking};
use std::collections::HashMap;

fn main() {
    // loops and calls runner then sleeps
    let thirty_seconds = time::Duration::from_micros(30);
    thread::sleep(thirty_seconds);
    println!("Slept {} micro-seconds", 30);
    runner();
}

fn runner() {
    let cpu_result: String = get_program_output("../stats-collector/utils/cpu_runner.sh");
    
    let ram_result: String = get_program_output("../stats-collector/utils/ram_runner.sh");
    
    let time_result: String = get_current_time();
    // let temperature_result: String = get_program_output("../stats-collector/utils/temperature_runner.sh"); Uncomment before build to run on pi (command in temp bash script is not available for linux)

    send_information(cpu_result, ram_result, String::from("34.6"), time_result)
}

fn send_information(cpu: String, ram: String, temp: String, time: String) {
    let mut map = HashMap::new();
    map.insert("PiId", String::from("1"));
    map.insert("Time", time);
    map.insert("cpu", cpu);
    map.insert("temp", temp);
    map.insert("ram", ram);
    
    let client = blocking::Client::new();
    let res = client.post("http://localhost:7071/api/PostPiStats")
        .json(&map)
        .send()
        .unwrap();
    println!("{:?}", res.status());
}

fn get_current_time() -> String {
    let result: DateTime<Local> = Local::now();             // Returns local time of type DateTime
    let current_time: String = result.to_string().clone();  // Clones result into a string
    current_time
}

fn get_program_output(program: &str) -> String {
    let output = Command::new(program)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Program failed to execute.");
    let result = String::from_utf8_lossy(&output.stdout);   // Convert list of bytes to COW
    let str_result: String = result.to_string().clone();    // Clones result into a string
    str_result
}

