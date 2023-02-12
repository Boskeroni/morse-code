const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

const MORSE_CODE_VALUES: &[u8] = &[
    0b0110_0000, // A
    0b1000_1000, // B
    0b1010_1000, // C
    0b1001_0000, // D
    0b0100_0000, // E
    0b0010_1000, // F
    0b1101_0000, // G
    0b0000_1000, // H
    0b0010_0000, // I
    0b0111_1000, // J
    0b1011_0000, // K
    0b0100_1000, // L
    0b1110_0000, // M
    0b1010_0000, // N
    0b1111_0000, // O
    0b0110_1000, // P
    0b1101_1000, // Q
    0b0101_0000, // R
    0b0001_0000, // S
    0b1100_0000, // T
    0b0011_0000, // U
    0b0001_1000, // V
    0b0111_0000, // W
    0b1001_1000, // X
    0b1011_1000, // Y
    0b1100_1000, // Z
    0b1111_1100, // 0
    0b0111_1100, // 1
    0b0011_1100, // 2
    0b0001_1100, // 3
    0b0000_1100, // 4
    0b0000_0100, // 5
    0b1000_0100, // 6
    0b1100_0100, // 7
    0b1110_0100, // 8
    0b1111_1100, // 9
];

fn text_to_morse(input: String) -> Option<String> {
    println!("text -> morse code");
    let mut output = String::new();
    for c in input.chars() {
        if c == ' ' {
            output.push_str("/ ");
            continue;
        }
        let mut morse_code = " ".to_string();
        let index = match LETTERS.find(c) {
            Some(s) => s,
            None => return None,
        };
        let letter_u8 = MORSE_CODE_VALUES[index];
        let mut padding = true;
        for i in 1..=7 {
            let bit = (letter_u8>>i)&1 == 1;
            if padding {
                padding = bit == false;
                continue;
            }
            morse_code.push(if bit {'-'} else {'.'})
        }
        output.push_str(&morse_code.chars().rev().collect::<String>());
    }
    Some(output)
}

fn morse_to_text(input: String) -> Option<String> {
    let mut output = String::new();

    let morse_chars: Vec<&str> = input.split(" ").collect();
    for character in morse_chars {
        if character == "/" {
            output.push(' ');
            continue;
        }

        let mut morse_code = 0b0000_0000;
        for i in (0..=7).rev() {
            if 7-i+1 > character.len() {
                morse_code |= 1 << i;
                break;
            }
            if character.chars().nth(7-i).unwrap() == '-' {
                morse_code |= 1 << i;
            }
        }
        println!("{:08b}", morse_code);
        output.push(LETTERS.chars().nth(MORSE_CODE_VALUES.iter().position(|&r| r == morse_code).unwrap()).unwrap());
    }

    Some(output)
}

fn main() {
    println!("enter the conversion type (morse/plain): ");
    let mut dest = String::new();
    std::io::stdin().read_line(&mut dest).unwrap();
    dest = dest.trim().to_lowercase();

    let mut plain_text = String::new();
    std::io::stdin().read_line(&mut plain_text).unwrap();
    plain_text = plain_text.trim().to_lowercase();

    let output = if dest == "morse" {
        text_to_morse(plain_text).expect("invalid character in string")
    } else if dest == "plain" {
        morse_to_text(plain_text).expect("invalid morse code text")
    } else {
        panic!("invalid conversion type");
    };

    println!("{}", output);
}