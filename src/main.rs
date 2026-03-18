use std::process::exit;
fn main() {
    let mut args = std::env::args().skip(1);
    let mut hidden: Vec<String>;
    let color: String;
    if args.len() > 1 {
        loop {
            let mut curr = args.next().unwrap().as_str();
            match curr {
                "-help" => help(),
                "-hide" => {
                    while let Some(arg) = args.next() {
                        hidden.push(arg);
                    }
                    if hidden.is_empty() {
                        eprintln!("feofetch: expected settings to hide after -hide");
                        exit(1)
                    }
                }
                "-color" => {
                    match args.next() {
                        Some(arg) => color = arg,
                        None => {
                            eprintln!("feofetch: expected roygbiv color argument for -color");
                            exit(1)
                        }
                    }
                }
                _ => panic!("feofetch: unknown argument: {}", curr)
            }

        }

        while i < end {
            match args.nth(i).as_str() {
                "-help" => help(),
                "-hide" => {

                }
                _ => panic!("unknown argument: {}", args[i])
            }
        }
        for arg in args.skip(1) {

        }
    }
    let supported_platforms = ["macos", "windows", "linux"];
}

fn help() {
    println!("feofetch: showing your system information... but made in rust");
    println!("USAGE: feofetch [OPTIONS]");
    println!("When run with no arguments, prints all information and uses default coloring");
    println!("settings\topens a menu to easily edit your current feofetch config");
    println!("-hide [SETTINGS]\tHides certain system info from output temporarily");
    println!("-color [COLOR]\tchanges the color of ascii art; ROYGBIV is allowed");
    println!("-help or --help prints this menu.")
}
