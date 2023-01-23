trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut morse_code: Message = vec![];

        for chr in self.chars() {
            let code = match chr {
                'a' | 'A' => vec![Pulse::Long, Pulse::Short],
                'b' | 'B' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'c' | 'C' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'd' | 'D' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'e' | 'E' => vec![Pulse::Short],
                'f' | 'F' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'g' | 'G' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'h' | 'H' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'i' | 'I' => vec![Pulse::Short, Pulse::Short],
                'j' | 'J' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'k' | 'K' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'l' | 'L' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'm' | 'M' => vec![Pulse::Long, Pulse::Long],
                'n' | 'N' => vec![Pulse::Long, Pulse::Short],
                'o' | 'O' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'p' | 'P' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'q' | 'Q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'r' | 'R' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                's' | 'S' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                't' | 'T' => vec![Pulse::Long],
                'u' | 'U' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'v' | 'V' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'w' | 'W' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'x' | 'X' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'y' | 'Y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'z' | 'Z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
                '1' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long],
                '2' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                '3' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Long],
                '4' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                '5' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                '6' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                '7' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                '8' => vec![Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
                '9' => vec![Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Short],
                '0' => vec![Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long, Pulse::Long],
                _ => continue
            };

            morse_code.push(code);
        }

        morse_code
    }
}

type Message = Vec<Letter>;

type Letter = Vec<Pulse>;

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let message = String::from("Hello etch1000").to_morse_code();
    
    print_morse_code(&message);
}

#[test]
fn test_hello() {
    let message = String::from("hello").to_morse_code();
    assert_eq!(message, vec![
        vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
        vec![Pulse::Short],
        vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
        vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
        vec![Pulse::Long, Pulse::Long, Pulse::Long]
    ]);
}

