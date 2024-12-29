use std::process::exit;

fn help(exit_code: i32) {
    let help = std::fs::read_to_string("src/help.txt").unwrap();
    println!("{}", help);
    exit(exit_code);
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in bytes.iter() {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    return hex_string;
}

fn bytes_to_ascii(bytes: Vec<u8>) -> String {
    let mut ascii = String::new();
    for byte in bytes {
        if byte >= 32 && byte <= 126 {
            ascii.push(byte as char);
        } else {
            ascii.push('.');
        }
    }
    return ascii;
}

fn hex_to_ascii(hex: &str) -> String {
    // Convert hex string to bytes
    let mut bytes = Vec::new();
    for i in 0..hex.len() / 2 {
        let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
        bytes.push(byte);
    }

    return bytes_to_ascii(bytes);
}

fn format_hex(hex: &str) -> String {
    let hex_bytes = hex.as_bytes();
    let mut result = String::new().to_owned();
    let mut hex_row = "".to_owned();

    result.push_str(&format!("{:08x}: ", 0));
    let mut i = 0;
    while i < hex.len() {
        if i != 0 && i % 32 == 0 {
            result.push_str(&format!("  {}", &hex_to_ascii(&hex_row)));
            hex_row = "".to_owned();
            result.push('\n');
            result.push_str(&format!("{:08x}: ", (i + 1) / 2));
        } else if i != 0 && i % 4 == 0 {
            result.push(' ');
        }
        result.push(hex_bytes[i] as char);
        hex_row.push(hex_bytes[i] as char);

        i += 1;
    }
    // Output leftover
    if i % 32 != 0 {
        result.push_str(&format!(
            "{:>width$}",
            &hex_to_ascii(&hex_row),
            width = 43 - hex_row.len()
        ));
    }
    return result;
}

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
                    help(0);
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
                    help(1);
                }
            };
        } else {
            if infile.is_empty() {
                infile = arg.to_string();
            } else if outfile.is_empty() {
                outfile = arg.to_string();
            } else {
                help(1);
            }
        }
    }
    run(infile, outfile);
}

fn read_file(file_path: &str) -> Vec<u8> {
    let data = std::fs::read(file_path).expect("Unable to read file.");
    return data;
}

fn read_stdin() {
    exit(0);
}

fn run(infile: String, outfile: String) {
    if infile.is_empty() {
        read_stdin();
    }

    let data = bytes_to_hex(&read_file(&infile));
    if outfile.is_empty() {
        println!("{:?}", format_hex(&data));
    } else {
        std::fs::write(outfile, format_hex(&data)).expect("Unable to write file.");
    }
}
