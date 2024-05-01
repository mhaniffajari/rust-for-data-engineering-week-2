use clap::Parser;
use cli_customize_fruit_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(name = "Fruit Salad", version = "1.0", author = "M H Fajari", about = "Mix fruits to create a fruit salad")]
struct Opts{
    #[clap(short,log)]
    fruits: Option<String>,
    csvfile:Option<String>,
}
fn csv_to_vec(csv:&str)-> Vec<String>{
    csv.split(',').map(|s| s.to_string()).collect()
}
fn display_fruit_salad(fruits:Vec<String>){
    println!("Your fruit salad contains:");
    for fruit in fruits{
        println!("{}",fruit);
    }   
}
fn main() {
    let opts = Opts::parse();
    let fruit_list = match opts.fruits{
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename).expect("Could not read file");
            csv_to_vec(&fruits)
        },
        None =>{{opts.fruits.unwrap_or_default().split(',').map(|s| s.to_string()).collect()}
    },
    };
    let fruid_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruid_salad)
}
