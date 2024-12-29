use std::process::exit;

mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut infile: String = String::new();
    let mut outfile: String = String::new();

    for arg in args.iter().skip(1) {
        if arg.starts_with("-") {
            // Handle flags
            match arg.as_str() {
                "-a" | "-autoskip" => {
                    println!("autoskip");
                }
                "-b" | "-bits" => {
                    println!("bits");
                }
                "-c" | "-cols" => {
                    println!("cols");
                }
                "-E" | "-EBCDIC" => {
                    println!("EBCDIC");
                }
                "-g" | "-groupsize" => {
                    println!("group");
                }
                "-h" | "-help" => {
                    util::help(0);
                }
                "-i" | "-include" => {
                    println!("include");
                }
                "-l" | "-len" => {
                    println!("len");
                }
                "-p" | "-ps" | "-postscript" | "-plain" => {
                    println!("ps");
                }
                "-r" | "-revert" => {
                    println!("revert");
                }
                "-seek" => {
                    println!("seek");
                }
                "-s" => {
                    println!("seek");
                }
                "-u" => {
                    println!("upper");
                }
                "-v" | "-version" => {
                    println!("xxd 2024-12-27 by Owen Wang.");
                    exit(0)
                }
                _ => {
                    util::help(1);
                }
            };
        } else {
            if infile.is_empty() {
                infile = arg.to_string();
            } else if outfile.is_empty() {
                outfile = arg.to_string();
            } else {
                util::help(1);
            }
        }
    }
    util::run(infile, outfile);
}
