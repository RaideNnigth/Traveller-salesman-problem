use std::env;
use std::process;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    // Getting the arguments from the command line
    // The first argument is the name of the program
    let args: Vec<_> = env::args().collect();
    
    // If no arguments are passed through the command line, exit
    if args.len() == 1 {
        println!("No argument send through command line");
        process::exit(64);
    }
    // Get the filepath from the arguments
    let filepath = &args[1];

    // Print the arguments
    println!("file path: {}", filepath);   
    
    // Open the file, if not possible, exit
    let mut file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Problem opening the file: {:?}", error);
            process::exit(1);
        }
    };

    // Read the file into a string        
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Should have been able to read the file");
    
    // Create a regex to find all the numbers in the file
    let regex = Regex::new(r"\d+").unwrap();

    // Find all the numbers in the file and put them in a vector
    let numbers: Vec<i32> = regex
        .find_iter(content.as_str())
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();

    // Print the numbers
    let num_of_columns = (numbers.len() as f64).sqrt() as u32;
    println!("Number of columns: {}", num_of_columns);
    println!("{:?}", numbers);
    
    
}