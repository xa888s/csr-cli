static ASCII_ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn encrypt(plaintext: &String, key: usize) -> String {
    let mut ciphertext = String::with_capacity(plaintext.capacity());

    for letter in plaintext.chars() {
        if letter.is_ascii_alphabetic() {
            let pos = ASCII_ALPHABET
                .iter()
                .position(|&bet| bet == letter)
                .unwrap();
            let new_pos = (key + pos) % 26;

            if letter.is_uppercase() {
                ciphertext.push(ASCII_ALPHABET[new_pos + 26]);
            } else {
                ciphertext.push(ASCII_ALPHABET[new_pos]);
            }
        } else {
            ciphertext.push(letter);
        }
    }

    ciphertext
}

pub fn decrypt(ciphertext: &String, key: usize) -> String {
    let mut plaintext = String::with_capacity(ciphertext.capacity());

    for letter in ciphertext.chars() {
        if letter.is_ascii_alphabetic() {
            let pos = ASCII_ALPHABET
                .iter()
                .position(|&bet| bet == letter)
                .unwrap();
            let new_pos = ((pos as isize) - (key as isize)).rem_euclid(26);

            if letter.is_uppercase() {
                plaintext.push(ASCII_ALPHABET[new_pos as usize + 26]);
            } else {
                plaintext.push(ASCII_ALPHABET[new_pos as usize]);
            }
        } else {
            plaintext.push(letter);
        }
    }

    plaintext
}
