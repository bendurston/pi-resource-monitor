use std::process;
use std::collections::HashMap;
use std::env;
use std::{thread, time};
use reqwest;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        println!("Program takes exactly 6 arguments.");
        process::exit(1);
    }

    let pi_id: &str = &args[1];
    let url: &str = &args[2];
    let auth: &str = &args[3];
    let cpu_runner: &str = &args[4];
    let ram_runner: &str = &args[5];
    let temp_runner: &str = &args[6];
    runner(&pi_id, &url, &auth, &cpu_runner, &ram_runner, &temp_runner);
}

fn runner(pi_id: &str, url: &str, auth: &str, cpu_runner: &str, ram_runner: &str, temp_runner: &str) {
    loop {
        {
            let cpu: &str = &get_program_output(cpu_runner);
            let ram: &str = &get_program_output(ram_runner);
            let temperature: &str = &get_program_output(temp_runner);

            let mut map = HashMap::new();
                map.insert("piId", pi_id);
                map.insert("cpu", cpu);
                map.insert("ram", ram);
                map.insert("temp", temperature);

            let result = send_information(url, auth, &map);
            
            if let Err(_e) = result {
                let twenty_minutes: time::Duration = time::Duration::from_secs(1200);
                sleep_on_failed_post(twenty_minutes)
            }
        }   // memory occupied by variables and function execution freed.
        let ten_minutes = time::Duration::from_secs(600);
        thread::sleep(ten_minutes);
    }
}

fn send_information(url: &str, _auth: &str, map: &HashMap<&str, &str>) -> Result<(), reqwest::Error>{
    let client = reqwest::blocking::Client::new();
    let _res = client.post(url)
        .json(&map)
        .send()?;
    Ok(())
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

fn sleep_on_failed_post(time: time::Duration) {
    thread::sleep(time);
}

