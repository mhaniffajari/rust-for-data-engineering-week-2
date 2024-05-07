pub fn encrypt(text:&str,shift:u8)->String{
    let mut result = String::new();
    for c in text.chars(){
        if c.is_ascii_alphabetic(){
            let base = if c.is_ascii_lowercase(){b'a'}else{b'A'};
            let offset = (c as u8 - base + shift) % 26;
            result.push((base+offset) as char);
        }else{
            result.push(c)
        }
    }
    result
}