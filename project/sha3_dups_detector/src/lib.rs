use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use sha3::Digest;
use sha3::Sha3_256;
use std::collections::HashMap;


//List of Phrases
static PHRASES: [&str;7] = [
    "My name is Hanif",
    "I am a software engineer",
    "I am from Indonesia",
    "You can call me Hanif",
    "I like book and movie",
    "I usually exercise to fill my free time",
    "I am a big fan of Arsenal"
];
//generate random phrases
pub fn generate_random_phrases()-> Vec<&'static str>{
    let mut rng = thread_rng();
    let mut phrases = Vec::new();
    for &phrase in PHRASES.iter(){
        let copies = rng.gen_range(1..=3);
        for _ in 0..copies{
            phrases.push(phrase);
        }
    }
    phrases.shuffle(&mut rng);
    phrases
}
// detectingg duplicates
pub fn analyze_duplicates(phrases:&[&str]){
    let mut hashes: HashMap<_,(usize, &str)>=HashMap::new();
    println!("Total number of phrases {}",phrases.len());


    for phrase in phrases{
        let hash = Sha3_256::digest(phrase.as_bytes());
        let entry = hashes.entry(hash).or_insert((0,phrase));
        entry.0+=1;
    }
    let total_unique_phrases = hashes.len();
    let mut total_unique_duplicates = 0;
    let mut total_combined_duplicates = 0;
    for (hash,(count,phrase)) in &hashes{
        if *count>1{
            total_unique_duplicates+=1;
            total_combined_duplicates+=count-1;
            println!("{} - {} times:{}",hex::encode(hash),count,phrase);
        }
    }
    println!("Total unique phrases: {}",total_unique_phrases);
    println!{"Total Unique duplicates: {}",total_unique_duplicates};
    println!("Total combined duplicates: {}",total_combined_duplicates);
}


