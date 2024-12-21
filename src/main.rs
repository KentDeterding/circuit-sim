use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::Result;

struct StdElement {
    name: u32,
    node_p: u32,
    node_n: u32,
    value: f64,
}

struct GroupedElement {
    name: u32,
    node_p: u32,
    node_n: u32,
    value: f64,
    is_group_2: Option<bool>,
}

struct Diode {
    name: u32,
    node_p: u32,
    node_n: u32,
    value: Option<f64>,
}

struct BJT {
    name: u32,
    node_c: u32,
    node_b: u32,
    node_e: u32,
    value: Option<f64>,
}

struct MOSFET {
    name: u32,
    node_d: u32,
    node_g: u32,
    node_s: u32,
    value: Option<f64>,
}

fn main() -> Result<()> {
    println!("Starting...");

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut voltage_sources: Vec<StdElement> = Vec::new();
    let mut current_sources: Vec<GroupedElement> = Vec::new();
    let mut resistors: Vec<GroupedElement> = Vec::new();
    let mut capacitors: Vec<GroupedElement> = Vec::new();
    let mut inductors: Vec<StdElement> = Vec::new();
    let mut diodes: Vec<Diode> = Vec::new();
    let mut bjts: Vec<BJT> = Vec::new();
    let mut fets: Vec<MOSFET> = Vec::new();

    // Parse line by line. Each line will add a single element to one of the vectors defined above.
    for line in reader.lines() {
        
    }

    println!("Done!");

    Ok(())
}
