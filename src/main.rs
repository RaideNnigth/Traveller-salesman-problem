use std::env;
use std::i32::MAX;
use std::process;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn hamiltonian_cycle(graph: &Vec<Vec<i32>>) {
    let path: Vec<usize> = hamiltonian_cycle_util(graph);
    let smallest_distance: i32 = get_path_distance(graph, &path) as i32;
    if smallest_distance == MAX {
        println!("There is no Solution for a Hamiltonian Cycle, exiting...");
        return;
    }

    println!("Solution Exists: Following is smallest distance Hamiltonian Cycle");
    println!("Path: {:?}", path);
}

fn hamiltonian_cycle_util(graph: &Vec<Vec<i32>>) -> Vec<usize> {
    let n = graph.len();
    let mut path: Vec<usize> = (0..n).collect();

    let mut smallest_distance = MAX;
    let mut smallest_path: Vec<usize> = Vec::new();

    permute_path(graph, &mut path, 1, &mut smallest_distance, &mut smallest_path);

    return smallest_path
}

fn permute_path(
    graph: &Vec<Vec<i32>>,
    path: &mut Vec<usize>,
    pos: usize,
    smallest_distance: &mut i32,
    smallest_path: &mut Vec<usize>,
) {
    if pos == graph.len() {
        let distance = get_path_distance(graph, path);
        if distance < *smallest_distance {
            *smallest_distance = distance;
            *smallest_path = path.clone();
        }
        print!("Path: {:?}, Distance: {}\n", path, distance)
    }
    for v in pos..graph.len() {
        if is_valid(v, graph, path, pos) {
            path.swap(pos, v);
            permute_path(graph, path, pos + 1, smallest_distance, smallest_path);
            path.swap(pos, v);
       }
    }
    
}

fn is_valid(v: usize, graph: &Vec<Vec<i32>>, path: &Vec<usize>, pos: usize) -> bool {
    if graph[path[pos - 1]][v] == 0 {
        return false;
    }

    for j in 0..pos {
        if path[j] == v {
            return false;
        }
    }

    return true
}

fn get_path_distance(graph: &Vec<Vec<i32>>, path: &Vec<usize>) -> i32 {
    let mut distance = 0;
    for i in 0..path.len() - 1 {
        distance += graph[path[i]][path[i + 1]];
    }
    distance += graph[path[path.len() - 1]][path[0]];
    return distance
}

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
    
    // Create a vector of vectors to store the graph
    let mut graph: Vec<Vec<i32>> = Vec::new();
    // Create a regex to find all the numbers in the file
    let regex = Regex::new(r"\d+").unwrap();
    // Populate the graph
    for line in content.lines() {        
        let row: Vec<i32> = regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        graph.push(row);
    }
    println!("Graph: {:?}", graph);
    
    hamiltonian_cycle(&graph)
}