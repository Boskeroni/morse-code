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
    0b0111_1100, // 1
    0b0011_1100, // 2
    0b0001_1100, // 3
    0b0000_1100, // 4
    0b0000_0100, // 5
    0b1000_0100, // 6
    0b1100_0100, // 7
    0b1110_0100, // 8
    0b1111_1100, // 9
    0b1111_1100, // 0
];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_lowercase();

    let mut output = String::new();

    for c in input.chars() {
        let mut morse_code = "".to_string();
        if !c.is_alphanumeric() {
            output.push(c);
            continue;
        }

        let index = if c.is_alphabetic() {
            c as u8 - 97u8
        } else {
            c as u8 - 23u8
        };
        let letter_u8 = MORSE_CODE_VALUES[index as usize];
        let mut extra = true;
        
        for i in 1..=7 {
            let bit = (letter_u8>>i)&1 == 1;
            if extra {
                if bit {
                    extra = false;
                }
                continue;
            }
            morse_code.push(if bit {'-'} else {'.'})
        }
        output.push_str(&morse_code.chars().rev().collect::<String>());
    }
    println!("{}", output);
}