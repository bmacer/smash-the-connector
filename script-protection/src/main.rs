use std::process::Command;
use std::fs;
use rand::Rng;
use std::process;
use std::time::Duration;
use std::{thread, time};

fn create_file() -> String {
    // Creates a random Powershell script `test-powershell-script-xxx.ps1`
    // inside hard-coded test directory "temporary-powershell-scripts"
    // where xxx is a random number between 0 and 99999
    // The script itself changes as well, a basic echo of the filename
    // Write-Host "Running xxx" where xxx is the filename

    let temp_dir = ".\\temporary-powershell-scripts";
    fs::create_dir_all(temp_dir).expect("some failure");
    let num = rand::thread_rng().gen_range(1..100000);
    let filename = format!("{}\\test-powershell-script-{}.ps1", temp_dir, num);
    let contents = format!("Write-Host \"Running {}\"", filename);
    
    fs::write(filename.clone(), contents).expect("Unable to write file");
    filename
}

fn main() {
    // Runs 10,000 times, 4 times per second to keep Powershell from yelling at us.
    // Each loop creates a unique powershell file, which we then run a new process
    // to execute.
    for _ in 0..10000 {
        thread::sleep(Duration::from_millis(250));
        let filename = create_file();
        Command::new("powershell")
        .args(&[format!(".\\{}", filename)])
        .spawn()
        .expect("echo command failed to start");
    }
}
