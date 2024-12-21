use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::Result;
use nom;

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
        let line = line?;

        let (remaining, element_type) = parse_type(line)?;
        match &element_type.to_uppercase()[0..] {
            "V" => {
                let (remaining, name) = get_next(remaining)?;
                let name = name.parse()?;
                let (remaining, node_p) = get_next(remaining)?;
                let node_p = node_p.parse()?;
                let (remaining, node_n) = get_next(remaining)?;
                let node_n = node_n.parse()?;
                let (_remaining, value) = get_next(remaining)?;
                let value = value.parse()?;

                voltage_sources.push(StdElement {name, node_p, node_n, value})
            }
            "I" => {}
            "R" => {}
            "C" => {}
            "L" => {}
            "D" => {}
            "QN" => {}
            "QP" => {}
            "MN" => {}
            "MP" => {}
            _ => panic!("Unexpected element type")
        }
    }

    println!("Done!");

    Ok(())
}

fn parse_type(input: String) -> Result<(String, String)> {
    Ok(("001 3 4 ".to_string(), "d".to_string()))
}

fn get_next(input: String) -> Result<(String, String)> {

    Ok(("string".to_string(), "another string".to_string()))
}
