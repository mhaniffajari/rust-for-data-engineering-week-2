use caesar_cipher::decrypt;
use caesar_cipher::encrypt;

fn main() {
    let plaintext = "Hi My name is Hanif";
    let shift = 1;
    let ciphertext = encrypt(plaintext, shift);
    let decrypted_text = decrypt(&plaintext, shift);
    println!("Plain Text :{}",plaintext);
    println!("Cipher Text:{}",ciphertext);
    println!("Decrypted Text: {}",decrypted_text);

}
