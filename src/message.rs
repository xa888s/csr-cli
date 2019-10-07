pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct Message {
    pub text: String,
}

impl Message {
    pub fn new(text: String) -> Message {
        Message { text: text }
    }

    pub fn encrypt(self, key: u8) -> String {
        let mut ciphervec: Vec<u8> = Vec::with_capacity(self.text.len());

        for char in self.text.bytes() {
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
        // this is safe because non-utf8 bytes will never be pushed to ciphervec
        unsafe { String::from_utf8_unchecked(ciphervec) }
    }

    pub fn decrypt(self, key: u8) -> String {
        let mut plainvec: Vec<u8> = Vec::with_capacity(self.text.len());

        for char in self.text.bytes() {
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
        // this is safe because non-utf8 bytes will never be pushed to plainvec
        unsafe { String::from_utf8_unchecked(plainvec) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_basic() {
        let input = String::from("Drsc sc k coxdoxmo");
        let output = String::from("This is a sentence");

        let message = Message::new(input);
        let key: u8 = 10;

        assert_eq!(message.decrypt(key), output);
    }

    #[test]
    fn test_encrypt_basic() {
        let input = String::from("Tests are important");
        let output = String::from("Nymnm uly cgjilnuhn");

        let message = Message::new(input);
        let key: u8 = 20;

        assert_eq!(message.encrypt(key), output);
    }

    #[test]
    fn test_emoji_passthrough_decrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Message::new(input);
        let key: u8 = 15;

        assert_eq!(message.decrypt(key), output);
    }

    #[test]
    fn test_emoji_passthrough_encrypt() {
        let input = String::from("😀 😁 😂 🤣 😃 😄 😅 😆 😉 😊 😋 😎 😍");

        let output = input.clone();
        let message = Message::new(input);
        let key: u8 = 15;

        assert_eq!(message.encrypt(key), output);
    }
}
