static ASCII_ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn encrypt(plaintext: String, key: usize) -> String {
    let mut ciphertext = String::with_capacity(plaintext.capacity());

    for letter in plaintext.chars() {
        match ASCII_ALPHABET
            .iter()
            .position(|&character| character == letter)
        {
            Some(pos) => {
                let new_pos = (key + pos) % 26;
                if letter.is_uppercase() {
                    ciphertext.push(ASCII_ALPHABET[new_pos + 26]);
                } else {
                    ciphertext.push(ASCII_ALPHABET[new_pos]);
                }
            }
            None => ciphertext.push(letter),
        }
    }
    ciphertext
}

pub fn decrypt(ciphertext: String, key: usize) -> String {
    let mut plaintext = String::with_capacity(ciphertext.capacity());

    for letter in ciphertext.chars() {
        match ASCII_ALPHABET.iter().position(|&bet| bet == letter) {
            Some(pos) => {
                let new_pos = ((pos as isize) - (key as isize)).rem_euclid(26);
                if letter.is_uppercase() {
                    plaintext.push(ASCII_ALPHABET[new_pos as usize + 26]);
                } else {
                    plaintext.push(ASCII_ALPHABET[new_pos as usize]);
                }
            }
            None => plaintext.push(letter),
        }
    }
    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        let input = String::from("Drsc sc k coxdoxmo");
        let output = String::from("This is a sentence");
        let key: usize = 10;
        assert_eq!(decrypt(input, key), output);
    }

    #[test]
    fn test_encrypt_basic() {
        let input = String::from("Tests are important");
        let output = String::from("Nymnm uly cgjilnuhn");
        let key: usize = 20;
        assert_eq!(encrypt(input, key), output);
    }
}
