use std::env;
use std::i32::MAX;
use std::process;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
use regex::Regex;

fn solve_tsp(graph: &Vec<Vec<i32>>) {

    // Start the timer
    let start = SystemTime::now();
    let path: Vec<usize> = hamiltonian_cycle(graph);
    // Stop the timer
    let total_time = SystemTime::now().duration_since(start).unwrap();
    
    // If the smallest distance is MAX, there is no solution
    let smallest_distance: i32 = get_path_distance(graph, &path) as i32;
    if smallest_distance == MAX {
        println!("There is no Solution for a Hamiltonian Cycle, exiting...");
        return;
    }
    // Print the results
    println!("Graph: {:?}", graph);
    println!("Solution Exists:");
    println!("Path: {:?}", path);
    println!("Distance: {}", smallest_distance);
    print!("Time: {}", total_time.as_secs_f64());    
}

fn hamiltonian_cycle(graph: &Vec<Vec<i32>>) -> Vec<usize> {
    // Create a vector to store the path and populate it with the nodes (nodes are represented by their index)
    let n = graph.len();
    let mut path: Vec<usize> = (0..n).collect();

    // Create a variable to store the smallest distance and the path that corresponds to it
    let mut smallest_distance: i32 = MAX;
    let mut smallest_path: Vec<usize> = Vec::new();

    // Call the permute_path function
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
    // If the position is the last one, calculate the distance and compare it to the smallest distance
    if pos == graph.len() {
        let distance = get_path_distance(graph, path);
        if distance < *smallest_distance {
            *smallest_distance = distance;
            *smallest_path = path.clone();
        }
    }else {
        // Swap the current position with all the positions after it
        for v in pos..graph.len() {
            // Swap the current position with the position v (being v >= pos)
            path.swap(pos, v);
            // Call the function recursively with the next position
            permute_path(graph, path, pos + 1, smallest_distance, smallest_path);
            // Swap back the current position with the position v (being v >= pos)
            path.swap(pos, v);
        }
    }    
}

fn get_path_distance(graph: &Vec<Vec<i32>>, path: &Vec<usize>) -> i32 {
    // If the path is empty, return MAX
    let mut distance = 0;
    // Add the distance from the first node to before last node
    for i in 0..path.len() - 1 {
        distance += graph[path[i]][path[i + 1]];
    }
    // Add the distance from the last note to the first node to complete the cycle
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
    
    // Call the solve_tsp function
    solve_tsp(&graph);
    
}