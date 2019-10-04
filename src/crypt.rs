pub fn encrypt(plaintext: String, key: u8) -> String {
    let mut ciphervec: Vec<u8> = Vec::with_capacity(plaintext.len());

    for char in plaintext.bytes() {
        let mut new_char = char;

        if char < 91 && char > 64 {
            let pos = char % 65;
            new_char = 65 + ((pos + key) % 26);
        } else if char < 123 && char > 96 {
            let pos = char % 97;
            new_char = 97 + ((pos + key) % 26);
        }
        ciphervec.push(new_char);
    }
    unsafe { String::from_utf8_unchecked(ciphervec) }
}

pub fn decrypt(ciphertext: String, key: u8) -> String {
    let mut plainvec: Vec<u8> = Vec::with_capacity(ciphertext.len());

    for char in ciphertext.bytes() {
        if char < 91 && char > 64 {
            let pos = char % 65;
            let new_char = 65 + ((pos as i8 - key as i8).rem_euclid(26));
            plainvec.push(new_char as u8);
        } else if char < 123 && char > 96 {
            let pos = char % 97;
            let new_char = 97 + ((pos as i8 - key as i8).rem_euclid(26));
            plainvec.push(new_char as u8);
        } else {
            plainvec.push(char);
        }
    }
    unsafe { String::from_utf8_unchecked(plainvec) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        let input = String::from("Drsc sc k coxdoxmo");
        let output = String::from("This is a sentence");
        let key: u8 = 10;
        assert_eq!(decrypt(input, key), output);
    }

    #[test]
    fn test_encrypt_basic() {
        let input = String::from("Tests are important");
        let output = String::from("Nymnm uly cgjilnuhn");
        let key: u8 = 20;
        assert_eq!(encrypt(input, key), output);
    }
}
