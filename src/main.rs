use plotters::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::hash::Hash;

#[allow(dead_code, non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq, Hash)]

// Edge struct where source is SOURECE_SUBREDDIT and
// target is TARGET_SUBREDDIT
struct Edge {
    source: String,
    target: String,
}

// read the csv file
fn read_csv(path: &str) -> Vec<Edge> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .flexible(true)
        .from_path(path);

    let mut edges: Vec<Edge> = vec![];

    for result in rdr.unwrap().records() {
        let record = result.unwrap();
        edges.push(Edge {
            source: record[0].to_string(),
            target: record[1].to_string(),
        });
    }
    return edges;
}

// create an adjacency list using HashMap, where each node has a list of the neighbors. 
// the key of the hashmap is a unique SOURCE_REDDIT and the value are TARGET_SUBREDDITs connected to it 
fn adj_list(edges: Vec<Edge>) -> HashMap<String, Vec<String>> {
    let mut graph_list: HashMap<String, Vec<String>> = HashMap::new();
    for v in edges.iter() {
        if graph_list.contains_key(&v.source) {
            let mut x = graph_list.get(&v.source).unwrap().clone();
            x.push(v.target.clone());
            graph_list.insert(v.source.clone(), x);
        } else {
            let new_vec: Vec<String> = vec![v.target.clone()];
            graph_list.insert(v.source.clone(), new_vec);
        }
    }
    return graph_list;
}

// calculate the length of each node’s adjacency list and store the length into a vector
fn calc_length(graph_list: HashMap<String, Vec<String>>) -> Vec<usize> {
    let mut length_vec: Vec<usize> = vec![];
    for (_k, v) in graph_list.into_iter() {
        let length = v.len();
        length_vec.push(length);
    }
    return length_vec;
}

// calculate the frequency of each length and store the frequency into a new vector
fn length_freq(length_vec: Vec<usize>) -> Vec<(usize, usize)> {
    let mut length_freq_vec: Vec<(usize, usize)> = vec![];
    let mut length_freq_hashmap: HashMap<usize, usize> = HashMap::new();
    for length in length_vec {
        if length_freq_hashmap.contains_key(&length) {
            let freq = length_freq_hashmap.get(&length).unwrap();
            length_freq_hashmap.insert(length, freq + 1);
        } else {
            length_freq_hashmap.insert(length, 1);
        }
    }

    // get vector of (length, freq) pairs from HashMap
    for entry in length_freq_hashmap {
        length_freq_vec.push(entry);
    }

    // sort length in descending order
    length_freq_vec.sort();
    return length_freq_vec;
}

// in main function, plot the frequencies using Rust Plotters
fn main() {
    let data = read_csv("./soc-redditHyperlinks-body.tsv");
    let graph_list_main = adj_list(data);
    let calc_length_main = calc_length(graph_list_main);
    let length_freq_main = length_freq(calc_length_main);
    println!("{:?}", length_freq_main);

    // plot the freqency
    let distn_graph = BitMapBackend::new("./distn.png", (600, 400)).into_drawing_area();
    distn_graph.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&distn_graph)
        .set_label_area_size(LabelAreaPosition::Left, 50.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 50.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("Distribution of Frequencies of Lengths", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0f64..90.0f64, 0f64..4500f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // draw bar plot
    ctx.draw_series((0..).zip(length_freq_main.iter()).map(|(x, y)| {
        let mut bar = Rectangle::new([(x as f64, 0f64), (x as f64 + 1f64, y.1 as f64)], BLUE.filled());
        bar.set_margin(0, 0, 5, 5);
        return bar;
    }))
    .unwrap();

}
