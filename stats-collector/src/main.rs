use std::process::{Command, Stdio};
use chrono::{Local, DateTime};

fn main() {
    println!("{}",get_command_output("ls"));
    println!("{}",get_current_time());
}

fn get_current_time() -> String{
    let result: DateTime<Local> = Local::now();             // Returns local time of type DateTime
    let current_time: String = result.to_string().clone();  // Clones result into a string
    current_time
}

fn get_command_output(command: &str) -> String{
    let output = Command::new(command)   //Execute command, pipes stdio to output
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute");
    let result = String::from_utf8_lossy(&output.stdout);   // Convert list of bytes to COW
    let str_result: String = result.to_string().clone();    // Clones result into a string
    str_result
}


// fn parse_cpu_stats(){
    
// }

// fn parse_gpu_stats(){

// }

// fn parse_ram_stats(){

// }