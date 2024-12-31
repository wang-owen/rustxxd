mod options;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut infile: String = String::new();
    let mut outfile: String = String::new();

    let mut options = options::XXDOptions {
        ..Default::default()
    };
    let mut i = 1;
    while let Some(arg) = args.get(i) {
        if arg.starts_with("-") {
            // Handle flags
            match arg.as_str() {
                "-b" | "-bits" => {
                    options.bits = true;
                }
                "-c" | "-cols" => {
                    options.cols = options::match_option(&args, i + 1);
                    if options.cols.unwrap() > 256 {
                        println!("xxd: invalid number of columns (max. 256).");
                        std::process::exit(1);
                    }
                    i += 1;
                }
                "-g" | "-groupsize" => {
                    options.groupsize = options::match_option(&args, i + 1);
                    i += 1;
                }
                "-h" | "-help" => {
                    util::help(0);
                }
                "-i" | "-include" => {
                    options.include = true;
                }
                "-l" | "-len" => {
                    options.len = options::match_option(&args, i + 1);
                    i += 1;
                }
                "-p" | "-ps" | "-postscript" | "-plain" => {
                    options.postscript = true;
                }
                "-r" | "-revert" => {
                    options.revert = true;
                }
                "-u" => {
                    options.uppercase = true;
                }
                "-v" | "-version" => {
                    println!("xxd 2024-12-27 by Owen Wang.");
                    std::process::exit(0)
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
        i += 1;
    }

    if options.bits && options.postscript {
        println!("xxd: only one of -b, -e, -u, -p, -i can be used");
        std::process::exit(1);
    }

    util::run(&options, infile, outfile);
}
