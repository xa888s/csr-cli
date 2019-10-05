pub fn encrypt(plaintext: String, key: u8) -> String {
    let mut ciphervec: Vec<u8> = Vec::with_capacity(plaintext.len());

    for char in plaintext.bytes() {
        ciphervec.push(match char {
            65..=90 => {
                let pos = char % 65;
                65 + ((pos + key) % 26)
            }
            97..=122 => {
                let pos = char % 97;
                97 + ((pos + key) % 26)
            }
            _ => char,
        });
    }
    unsafe { String::from_utf8_unchecked(ciphervec) }
}

pub fn decrypt(ciphertext: String, key: u8) -> String {
    let mut plainvec: Vec<u8> = Vec::with_capacity(ciphertext.len());

    for char in ciphertext.bytes() {
        plainvec.push(match char {
            65..=90 => {
                let pos = char % 65;
                65 + ((pos as i8 - key as i8).rem_euclid(26)) as u8
            }
            97..=122 => {
                let pos = char % 97;
                97 + ((pos as i8 - key as i8).rem_euclid(26)) as u8
            }
            _ => char,
        });
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

    #[test]
    fn test_emoji_passthrough() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");
        let key: u8 = 15;
        assert_eq!(&encrypt(input.clone(), key.clone()), &input);
        assert_eq!(&decrypt(input.clone(), key), &input);
    }
}
