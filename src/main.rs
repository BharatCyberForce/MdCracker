use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use md5;
use colored::*;

fn compute(password: &str) -> String {
    let hash = md5::compute(password);
    format!("{:x}", hash)
}

fn cracker(target: &str, passlist: &str) -> Option<String> {
    let file = File::open(passlist).expect("Error Opening Pass list File");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let password = line.unwrap().trim().to_string();
        let hashed = compute(&password);
        
        if hashed == target {
            println!("{}", password.green());
            return Some(password);
        }
    }

    println!("{}","[-] Bad luck! Hash not matched".red());
    None
}

fn icfbanner(){
    let banner=r#"
    ========================================
    |               MdCracker              |
    |    Developed by: Indian Cyber Force  |
    ========================================"#;

    println!("{}",banner.cyan());
}
fn main() {
    icfbanner();
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("{} <hash> <passwordlist.txt>", args[0]);
        std::process::exit(1);
    }

    let target = &args[1];
    let passlist = &args[2];

    cracker(target, passlist);
}
