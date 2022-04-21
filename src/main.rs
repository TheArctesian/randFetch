use clap::Parser;

struct Cli {
    name: String,
    specs: String,
}
/*
 let name = std::env::args().nth(1).expect("no pattern given");
// let specs = std::env::args().nth(2).expect("no args");
let args = Cli {
       name: name, 
       specs: specs,
    }
    */
    
fn main() {
    let args = Cli::parse();
}
