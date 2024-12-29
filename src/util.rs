pub fn help(exit_code: i32) {
    let help = std::fs::read_to_string("src/help.txt").unwrap();
    println!("{}", help);
    std::process::exit(exit_code);
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in bytes.iter() {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    return hex_string;
}

fn hex_to_ascii(hex: &str) -> String {
    // Convert hex string to bytes
    let mut bytes = Vec::new();
    for i in 0..hex.len() / 2 {
        let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
        bytes.push(byte);
    }

    // Convert bytes to ascii
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

fn read_stdin() {
    // TODO
    std::process::exit(0);
}

pub fn run(infile: String, outfile: String) {
    if infile.is_empty() {
        read_stdin();
    }

    let data = std::fs::read(infile).expect("Unable to read file.");
    let hex = bytes_to_hex(&data);
    if outfile.is_empty() {
        println!("{:?}", format_hex(&hex));
    } else {
        std::fs::write(outfile, format_hex(&hex)).expect("Unable to write file.");
    }
}
