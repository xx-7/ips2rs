use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = if args.len() > 1 { &args[1] } else { "conf" };

    match ips2rs::run(file) {
        Ok(list) => {
            for ip in list {
                println!("{}", ip);
            }
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
