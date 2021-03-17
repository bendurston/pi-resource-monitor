use std::process::{Command, Stdio};
use std::{thread, time};
use chrono::{Local, DateTime};

fn main() {
    // loops and calls runner then sleeps
    let thirty_seconds = time::Duration::from_micros(30);
    thread::sleep(thirty_seconds);
    println!("Slept {} micro-seconds", 30);
    runner();
}

fn runner(){
    let cpu_result: String = get_program_output("../stats-collector/utils/cpu_runner.sh");
    let cpu_final = parse_cpu_stats(&cpu_result);
    
    let ram_result: String = get_program_output("../stats-collector/utils/ram_runner.sh");
    let ram_final = parse_ram_stats(&ram_result);
    
    // let temperature_final: String = get_program_output("../stats-collector/utils/temperature_runner.sh"); Uncomment before build to run on pi (command in temp bash script is not available for linux)

    println!("{}", cpu_final);
    println!("{}", ram_final);
    // println!("{}", temperature_final); Uncomment before build to run on pi (command in temp bash script is not available for linux)
    println!("{}",get_current_time());
}

fn get_current_time() -> String{
    let result: DateTime<Local> = Local::now();             // Returns local time of type DateTime
    let current_time: String = result.to_string().clone();  // Clones result into a string
    current_time
}

fn get_program_output(program: &str) -> String{
    let output = Command::new(program)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Program failed to execute.");
    let result = String::from_utf8_lossy(&output.stdout);   // Convert list of bytes to COW
    let str_result: String = result.to_string().clone();    // Clones result into a string
    str_result
}

fn parse_cpu_stats(result: &str) -> &str{
    let cpu = &result[..];
    cpu
}

fn parse_ram_stats(result: &str) -> &str{
    let ram = &result[..];
    ram
}

