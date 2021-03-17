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
    let arr: [String; 5]= [String::from("-b"), String::from("-n"), String::from("1"), String::from(""), String::from("")];
    // top -b -n 1 | grep Cpu -> returns best result.
    let cpu_result: String = get_command_output("top", &arr);
    println!("{}", cpu_result);
    //let cpu_final = parse_cpu_stats(&cpu_result);

    //let ram_result: String = get_command_output("free", String::from(""));

    //let ram_final = parse_ram_stats(&ram_result);


    //println!("{}", ram_final);
    println!("----------------------------------------------------------------");
    //println!("{}", cpu_final);
    //println!("{}",get_current_time());
}

fn get_current_time() -> String{
    let result: DateTime<Local> = Local::now();             // Returns local time of type DateTime
    let current_time: String = result.to_string().clone();  // Clones result into a string
    current_time
}

fn get_command_output(command: &str, args: &[String; 5]) -> String{
    let output = Command::new(command)   //Execute command, pipes stdio to output
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute");
    println!("{:?}", String::from_utf8_lossy(&output.stderr));
    let result = String::from_utf8_lossy(&output.stdout);   // Convert list of bytes to COW
    // println!("{}", result);
    let str_result: String = result.to_string().clone();    // Clones result into a string
    str_result
}


fn parse_cpu_stats(result: &str) -> &str{
    let cpu = &result[..];
    cpu
}

// fn parse_gpu_stats(){

// }

fn parse_ram_stats(result: &str) -> &str{
    let ram = &result[..204];
    ram
}