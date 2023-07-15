
use std::env;
use std::process;
use std::i32::MAX;
use std::time::SystemTime;
use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;
use std::cmp::Ordering;
use regex::Regex;
use std::cmp::Reverse;

#[derive(Clone, Debug, Copy)]
struct Edge {
    src: i32,
    dst: i32,
    weight: i32,
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

fn prim_mst(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Adding all the edges to a vector for futher sorting
    let mut edges: Vec<Reverse<Edge>> = Vec::new();
    for i in 0..graph.len() {
        for j in (i+1)..graph.len() {            
            if graph[i][j] != 0 {
                edges.push(Reverse(Edge {
                    src: i as i32,
                    dst: j as i32,
                    weight: graph[i][j],
                }));
            }
        }
    }
    // Sorting it by descending order cause will be used as stack later on
    edges.sort();
    
    // Creating a FIFO queue from the sorted edges
    let mut edges_fifo: VecDeque<Reverse<Edge>> = VecDeque::from(edges.clone());
    let mut visited: Vec<i32> = Vec::new();
    let mut mst: Vec<Edge> = Vec::new();
    
    // Adding the frist edge to the MST
    let first_edge = edges_fifo.pop_back().unwrap();
    mst.push(first_edge.0);
    visited.push(first_edge.0.src);
    visited.push(first_edge.0.dst);
    
    // While the MST is not complete
    while mst.len() < graph.len() - 1 {
        let edge = edges_fifo.pop_back().unwrap();
        if visited.contains(&edge.0.src) && !visited.contains(&edge.0.dst) {
            visited.push(edge.0.dst);
            mst.push(edge.0);
            let mut temp_vec: Vec<Reverse<Edge>> = edges_fifo.clone().into();
            temp_vec.sort();
            edges_fifo = VecDeque::from(temp_vec);
        } else if !visited.contains(&edge.0.src) && visited.contains(&edge.0.dst) {
            visited.push(edge.0.src);
            mst.push(edge.0);
            let mut temp_vec: Vec<Reverse<Edge>> = edges_fifo.clone().into();
            temp_vec.sort();
            edges_fifo = VecDeque::from(temp_vec);
        } else {
            edges_fifo.push_front(edge);
        }
    }
    
    // Initialize the minimum spanning tree graph with zeros
    let mut mst_graph: Vec<Vec<i32>> = vec![vec![0; graph.len()]; graph.len()];
        
    // Fill in the weights for the minimum spanning tree edges
    for edge in mst {
        mst_graph[edge.src as usize][edge.dst as usize] = edge.weight;
    }
    
    return mst_graph
}

fn mst_to_multigraph(mst: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut multigraph = mst.clone();
    for i in 0..mst.len() {
        for j in 0..mst.len() {
            if mst[i][j] != 0 {
                // Add duplicate edge
                multigraph[j][i] = mst[i][j];
            }
        }
    }
    return multigraph
}

fn solve_tsp(graph: &Vec<Vec<i32>>) {

    // Start the timer
    let start = SystemTime::now();
    let mut mst:Vec<Vec<i32>> = prim_mst(graph);
    let mut multigraph:Vec<Vec<i32>> = mst_to_multigraph(mst);
        
    
    let path: Vec<usize> = Vec::new(); //hamiltonian_cycle(graph);
    // Stop the timer
    let total_time = SystemTime::now().duration_since(start).unwrap();    
    
    /*
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
    */
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