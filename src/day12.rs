use crate::loadinput::loadinput;
use std::collections::HashMap;

fn traverse_graph(graph_connections : &HashMap<&str, Vec<&str>>, 
                  node_visits : &mut HashMap<&str, usize>,
                  mut graph_paths : usize,
                  cur_node : &str,) -> usize{

    for connection in &graph_connections[cur_node] {
        if *connection == "end" {
            graph_paths += 1;
            continue;
        }
        *node_visits.get_mut(cur_node).unwrap() += 1;
        if connection.chars().nth(0).unwrap().is_uppercase()
            || node_visits[*connection] == 0 {
            graph_paths = traverse_graph(graph_connections, node_visits, graph_paths, connection);
        }
        *node_visits.get_mut(cur_node).unwrap() -= 1;
    }
    graph_paths
}

pub fn day12p1() {
    let input = loadinput("./input/day12.txt");
    let mut graph_connections : HashMap<&str, Vec<&str>> = HashMap::new();
    for line in &input {
        let mut iter = line.split('-');
        let first = iter.next().unwrap();
        let last = iter.last().unwrap();
        if !graph_connections.contains_key(&first) {
            graph_connections.insert(first.clone(), vec![last.clone()]);
        }else{
            graph_connections.get_mut(&first).unwrap().push(last.clone());
        }
        if !graph_connections.contains_key(&last) {
            graph_connections.insert(last.clone(), vec![first.clone()]);
        }else{
            graph_connections.get_mut(&last).unwrap().push(first.clone());
        }
    }
    let mut node_visits : HashMap<&str, usize> = graph_connections.iter().map(|(key, _val)| (*key, 0)).collect();
//    for (key,val) in &graph_connections {
//        println!("Node: {} - Connections: {:#?}", key, val);
//    }
//    let mut graph_paths : Vec<Vec<String>> = vec![Vec::new()];
    let graph_paths = traverse_graph(&graph_connections, &mut node_visits, 0, "start");
    println!("Cave paths: {}", graph_paths);
}

fn traverse_graph2(graph_connections : &HashMap<&str, Vec<&str>>, 
                  node_visits : &mut HashMap<&str, usize>,
                  //graph_paths : &mut Vec<Vec<String>>,
                  mut graph_paths : usize,
                  mut multiple_visits: bool,
                  cur_node : &str) -> usize{
    for connection in &graph_connections[cur_node] {
        if *connection == "end" {
            graph_paths += 1;
            continue;
        }
        multiple_visits |= node_visits[cur_node] >= 1 && cur_node.chars().nth(0).unwrap().is_lowercase();
        *node_visits.get_mut(cur_node).unwrap() += 1;
        if (connection.chars().nth(0).unwrap().is_uppercase() ||
            (node_visits[*connection] < 2 && !multiple_visits) ||
             node_visits[*connection] == 0) && 
            *connection != "start" {
            graph_paths = traverse_graph2(graph_connections, node_visits, graph_paths, multiple_visits, connection);
        }
        *node_visits.get_mut(cur_node).unwrap() -= 1;
    }
    graph_paths
}

pub fn day12p2() {
    let input = loadinput("./input/day12.txt");
    let mut graph_connections : HashMap<&str, Vec<&str>> = HashMap::new();
    for line in &input {
        let mut iter = line.split('-');
        let first = iter.next().unwrap();
        let last = iter.last().unwrap();
        if !graph_connections.contains_key(&first) {
            graph_connections.insert(first.clone(), vec![last.clone()]);
        }else{
            graph_connections.get_mut(&first).unwrap().push(last.clone());
        }
        if !graph_connections.contains_key(&last) {
            graph_connections.insert(last.clone(), vec![first.clone()]);
        }else{
            graph_connections.get_mut(&last).unwrap().push(first.clone());
        }
    }
    let mut node_visits : HashMap<&str, usize> = graph_connections.iter().map(|(key, _val)| (*key, 0)).collect();
    let graph_paths = traverse_graph2(&graph_connections, &mut node_visits, 0, false, "start");
    println!("Cave paths: {}", graph_paths);
}
