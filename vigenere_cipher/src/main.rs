fn main() {
    let key = "thisIsakey";
    let plain_text = "This is Ariqt International";

    println!("Plain text: {plain_text}");
    println!("Vignere Ciphertext: {}", vic(plain_text, key));
}

fn vic(pt: &str, k: &str) -> String {
    let key: String = k.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
    let key = key.to_ascii_lowercase();
    let key_len = key.len();
    
    if key_len == 0 {
        return String::from(pt)
    }

    let mut index = 0;

    pt.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shift = key.as_bytes()[index % key_len] - b'a';
            index += 1;
            (first + (c as u8 + shift - first) % 26) as char
        } else {
            c
        }
    }).collect()
}
