use std::process;
use std::error::Error;
use std::collections::HashMap;
use std::env;
use std::{thread, time};
use chrono;
use reqwest;



fn main() {
    // loops and calls runner then sleeps
    let args: Vec<String> = env::args().collect();

    if args.len() != 4{
        println!("Program only takes 3 arguments.");
        process::exit(1);
    }

    let pi_id: String = String::from(&args[1]);
    let url: String = String::from(&args[2]);
    let auth: String = String::from(&args[3]);
    
    runner(pi_id, url, auth);
}

fn runner(pi_id: String, url: String, auth: String) {

    // let thirty_seconds = time::Duration::from_micros(30);
    // thread::sleep(thirty_seconds);
    // println!("Slept {} micro-seconds", 30);

    let cpu_result: String = get_program_output("../stats-collector/utils/cpu_runner.sh");
    
    let ram_result: String = get_program_output("../stats-collector/utils/ram_runner.sh");
    
    let time_result: String = get_current_time();
    // let temperature_result: String = get_program_output("../stats-collector/utils/temperature_runner.sh"); Uncomment before build to run on pi (command in temp bash script is not available for linux)

    send_information(url, auth, pi_id, cpu_result, ram_result, String::from("34.6"), time_result)
}

fn send_information(url: String, _auth: String, pi_id: String, cpu: String, ram: String, temp: String, time: String) {
    let mut map = HashMap::new();
        map.insert("PiId", pi_id);
        map.insert("Time", time);
        map.insert("cpu", cpu);
        map.insert("temp", temp);
        map.insert("ram", ram);
    
    let client = reqwest::blocking::Client::new();
    let _res = client.post(url)
        .json(&map)
        .send()
        .unwrap();
}


fn get_current_time() -> String {
    let result: chrono::DateTime<chrono::Local> = chrono::Local::now();     // Returns local time of type DateTime
    let current_time: String = result.to_string().clone();                  // Clones result into a string
    current_time
}

fn get_program_output(program: &str) -> String {
    let output = process::Command::new(program)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .expect("Program failed to execute.");
    let result = String::from_utf8_lossy(&output.stdout);   // Convert list of bytes to COW
    let str_result: String = result.to_string().clone();    // Clones result into a string
    str_result
}

