use rcron::{JobScheduler, Job };
use std::{process::{Command,exit}, time::Duration};

fn main() {
    let mut sched = JobScheduler::new();
    let period = std::env::args().nth(1).expect("no schedule given");
    let _ = std::env::args().nth(2).expect("no command given");

    sched.add(Job::new(period.parse().unwrap(), || {
        let args: Vec<String> = std::env::args().collect();
        let command = &args[2..].join(" ");
        let output = Command::new("sh").arg("-c").arg(command).output().expect("Command failed");

        if !output.status.success() {
            eprintln!("Command executed with failing error code");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
