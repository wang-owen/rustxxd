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
    let cols = options.cols.unwrap();

    let mut row_ascii = String::new();

    if options.postscript {
    } else if options.include {
    } else {
        let mut i = 0;
        while let Some(byte) = data.get(i) {
            // Offset
            if i == 0 || i % cols == 0 {
                // ASCII representation
                if i != 0 {
                    output.push_str(&format!("  {}", row_ascii));
                }
                row_ascii.clear();

                output.push('\n');

                if !options.bits {
                    output.push_str(&format!("{:0>8x}:", i));
                } else {
                    output.push_str(&format!("{:0>8}:", i));
                }
            }

            if i % groupsize == 0 {
                output.push(' ');
            }
            if !options.bits {
                output.push_str(&format!("{:0>2x}", byte));
            } else {
                output.push_str(&format!("{:0>2}", byte));
            }

            if *byte >= 32 && *byte <= 126 {
                row_ascii.push(*byte as char);
            } else {
                row_ascii.push('.');
            }

            i += 1;
        }
        if i % cols != 0 {
            let width = cols * 2 + usize::div_ceil(cols, groupsize) + 2;

            output.push_str(&format!(
                "{:>w$}",
                row_ascii,
                w = width - (row_ascii.len() + usize::div_ceil(row_ascii.len(), groupsize))
            ));
        }
    }

    if !outfile.is_empty() {
        std::fs::write(outfile, output).expect("Unable to write to file.");
        return;
    }
    println!("{}", output);
}

// fn read_stdin(options: &options::XXDOptions) {
//     // TODO
//     std::process::exit(0);
// }
