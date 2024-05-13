
use rayon::prelude::*;
use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia;


struct ProcessedPage{
    title: String,
    data : String
}

const PAGES: [&str; 5] = ["Rust", "Python", "Java","C++","C"];

fn process_page(page: &Page<Client>) -> ProcessedPage {
    let title = page.get_title().unwrap();
    let concent = page.get_content().unwrap();
    ProcessedPage {
        title: title.to_string(),
        data: concent.to_string()
    }   
}

fn main() {
    let start = std::time::Instant::now();
    let wikipedia = Wikipedia::<Client>::default();
    let pages: Vec<_> = PAGES.par_iter().map(|&p|wikipedia.page_from_title(p.to_string())).collect();
    let processed_pages : Vec<ProcessedPage> = pages.par_iter().map(process_page).collect();
    for page in processed_pages{
        let start_page = std::time::Instant::now();
        let first_sentence = page.data.split('.').next().unwrap();
        println!("Title: {}\nFirst sentence: {}\n", page.title, first_sentence);
        let word_count = page.data.split_whitespace().count();
        println!("Word count: {}\n", word_count);
        println!("Page time:{:?}", start_page.elapsed());
    }
    println!("Total time:{:?}", start.elapsed());
    println!("Average time per page: {:?}",start.elapsed()/PAGES.len() as u32);
    println!("Number of threads{}",rayon::current_num_threads());
}
