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

    write_formatted(options, infile, outfile);
}

fn write_formatted(options: &options::XXDOptions, infile: String, outfile: String) {
    // Read from infile
    let data = std::fs::read(infile.clone()).expect("Unable to read file.");

    let mut output = String::new().to_owned();

    if !options.revert {
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
        let mut i = 0;

        if options.postscript {
            while let Some(byte) = data.get(i) {
                match options.len {
                    Some(len) => {
                        if i == len {
                            break;
                        }
                    }
                    None => {}
                }

                if options.uppercase {
                    output.push_str(&format!("{:0>byte_width$X}", byte));
                } else {
                    output.push_str(&format!("{:0>byte_width$x}", byte));
                }

                i += 1;
                if i % cols == 0 {
                    output.push('\n');
                }
            }
            output.push('\n');
        } else if options.include {
            // TODO
            let include_name = infile.replace('.', "_");
            output.push_str(&format!("unsigned char {}[] = {{", include_name));

            while let Some(byte) = data.get(i) {
                match options.len {
                    Some(len) => {
                        if i == len {
                            break;
                        }
                    }
                    None => {}
                }

                if i % cols == 0 {
                    output.push_str("\n  ");
                }

                if !options.bits {
                    if options.uppercase {
                        output.push_str(&format!("{:0>byte_width$X}", byte));
                    } else {
                        output.push_str(&format!("{:0>byte_width$x}", byte));
                    }
                } else {
                    output.push_str(&format!("{:0>byte_width$b}", byte));
                }

                i += 1;
                if i < data.len() {
                    output.push_str(", ");
                }
            }
            output.push_str(&format!(
                "\n}};\nunsigned int {}_len = {}\n",
                include_name,
                data.len()
            ));
        } else {
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
                        output.push('\n');
                    }
                    row_ascii.clear();

                    // Offset
                    if options.uppercase {
                        output.push_str(&format!("{:0>8X}:", i));
                    } else {
                        output.push_str(&format!("{:0>8x}:", i));
                    }
                }

                if i % groupsize == 0 || i % cols == 0 {
                    output.push(' ');
                }
                if !options.bits {
                    if options.uppercase {
                        output.push_str(&format!("{:0>byte_width$X}", byte));
                    } else {
                        output.push_str(&format!("{:0>byte_width$x}", byte));
                    }
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
    } else {
        // TODO: revert input
        write_revert(&data, &mut output, options.postscript);
    }

    if !outfile.is_empty() {
        std::fs::write(outfile, output).expect("Unable to write to file.");
    } else {
        print!("{}", output);
    }
}

fn write_revert(data: &[u8], output: &mut String, postscript: bool) {
    let mut bytes = Vec::new();
    let mut i = 0;
    if !postscript {
        let mut read_offset = false;
        let mut read_bytes = false;
        let mut last_char = '.';
        while let Some(byte) = data.get(i) {
            if !read_offset {
                if *byte as char == ':' {
                    i += 2;
                    read_offset = true;
                    continue;
                }
            }

            if read_offset && !read_bytes {
                if *byte as char != ' ' {
                    bytes.push(
                        u8::from_str_radix(std::str::from_utf8(&data[i..i + 2]).unwrap(), 16)
                            .unwrap(),
                    );
                    i += 1;
                } else if last_char == ' ' {
                    read_bytes = true;
                    last_char = '.';
                    i += 1;
                    continue;
                }
                last_char = *byte as char;
            }

            if read_bytes {
                if *byte as char == '\n' {
                    i += 1;
                    read_offset = false;
                    read_bytes = false;
                }
            }

            i += 1;
        }
    } else {
        while let Some(byte) = data.get(i) {
            if *byte as char == '\n' {
                i += 1;
                continue;
            }
            bytes.push(
                u8::from_str_radix(std::str::from_utf8(&data[i..i + 2]).unwrap(), 16).unwrap(),
            );
            i += 2;
        }
    }

    for byte in bytes {
        if byte >= 32 && byte <= 126 {
            output.push(byte as char);
        } else {
            output.push('.');
        }
    }
}

// fn read_stdin(options: &options::XXDOptions) {
//     // TODO
//     std::process::exit(0);
// }
