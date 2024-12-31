use crate::options;

pub fn help(exit_code: i32) {
    let help = std::fs::read_to_string("src/help.txt").unwrap();
    println!("{}", help);
    std::process::exit(exit_code);
}

pub fn run(options: &options::XXDOptions, infile: String, outfile: String) {
    if infile.is_empty() {
        // TODO
        // read_stdin(options);
    }

    // Read from infile
    let data = std::fs::read(infile).expect("Unable to read file.");

    write_formatted(&data, options, outfile);
}

fn write_formatted(data: &[u8], options: &options::XXDOptions, outfile: String) {
    let mut output = String::new().to_owned();

    let byte_width = match options.bits {
        true => 8,
        false => 2,
    };
    let groupsize = match options.groupsize {
        Some(size) => size,
        None => {
            if options.bits {
                1
            } else {
                2
            }
        }
    };
    let cols = match options.cols {
        Some(size) => size,
        None => {
            if options.bits {
                6
            } else if options.include {
                12
            } else if options.postscript {
                30
            } else {
                16
            }
        }
    };

    let mut row_ascii = String::new();

    if options.postscript {
    } else if options.include {
    } else {
        let mut i = 0;
        while let Some(byte) = data.get(i) {
            // Check if exceeded -l len
            match options.len {
                Some(len) => {
                    if i == len {
                        break;
                    }
                }
                None => {}
            }
            if i == 0 || i % cols == 0 {
                // ASCII representation
                if i != 0 {
                    output.push_str(&format!("  {}", row_ascii));
                }
                row_ascii.clear();

                output.push('\n');

                // Offset
                output.push_str(&format!("{:0>8x}:", i));
            }

            if i % groupsize == 0 {
                output.push(' ');
            }
            if !options.bits {
                output.push_str(&format!("{:0>byte_width$x}", byte));
            } else {
                output.push_str(&format!("{:0>byte_width$b}", byte));
            }

            if *byte >= 32 && *byte <= 126 {
                row_ascii.push(*byte as char);
            } else {
                row_ascii.push('.');
            }

            i += 1;
        }

        // Leftover not outputted in ASCII
        if i % cols != 0 {
            output.push_str(&format!(
                "{:>w$}",
                row_ascii,
                w = row_ascii.len() // Len of ASCII
                    + 2 // Padding to the left
                    + (cols - row_ascii.len()) * byte_width // Width of remaining cols
                    + (cols - row_ascii.len()) / groupsize // Space seperation between cols
            ));
        }
    }

    if !outfile.is_empty() {
        std::fs::write(outfile, output).expect("Unable to write to file.");
    } else {
        println!("{}", output);
    }
}

// fn read_stdin(options: &options::XXDOptions) {
//     // TODO
//     std::process::exit(0);
// }
