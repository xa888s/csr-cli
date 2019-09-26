static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

pub fn encrypt(plaintext: &String, key: usize) -> String {
    let mut ciphertext = String::new();

    for letter in plaintext.trim().chars() {
        let pos = ASCII_LOWER
            .iter()
            .position(|&bet| bet == letter)
            .unwrap();
        let new_pos = (key + pos) % 26;

        ciphertext.push(ASCII_LOWER[new_pos]);
    }

    ciphertext
}

pub fn decrypt(ciphertext: &String, key: usize) -> String {
    let mut plaintext = String::new();

    for letter in ciphertext.trim().chars() {
        let pos = ASCII_LOWER
            .iter()
            .position(|&bet| bet == letter)
            .unwrap();
        let new_pos = ((pos as isize) - (key as isize)).rem_euclid(26); 
        
        plaintext.push(ASCII_LOWER[new_pos as usize]);
    }

    plaintext
}
