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
    polarity: Polarity,
}

enum Polarity {
    Ntype,
    Ptype,
}

struct MOSFET {
    name: u32,
    node_d: u32,
    node_g: u32,
    node_s: u32,
    value: Option<f64>,
    polarity: Polarity,
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

        if line.trim() == "" {
            continue;
        }

        let line = match line.split_once('%') {
            Some((line, _)) => line.to_string(),
            None => line,
        };

        let (remaining, element_type) = parse_type(line)?;
        match &element_type.to_uppercase()[0..] {
            "V" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_p = iter.next().unwrap().parse::<u32>()?;
                let node_n = iter.next().unwrap().parse::<u32>()?;
                let value = iter.next().unwrap().parse::<f64>()?;
                
                voltage_sources.push(StdElement{name, node_p, node_n, value})
            }
            "I" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_p = iter.next().unwrap().parse::<u32>()?;
                let node_n = iter.next().unwrap().parse::<u32>()?;
                let value = iter.next().unwrap().parse::<f64>()?;
                let is_group_2 = match iter.next() {
                    Some(_) => Some(true),
                    None => None
                };

                current_sources.push(GroupedElement{name, node_p, node_n, value, is_group_2});
            }
            "R" => {
                resistors.push(new_grouped_element(remaining)?);
            }
            "C" => {
                capacitors.push(new_grouped_element(remaining)?);
            }
            "L" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_p = iter.next().unwrap().parse::<u32>()?;
                let node_n = iter.next().unwrap().parse::<u32>()?;
                let value = iter.next().unwrap().parse::<f64>()?;
                
                inductors.push(StdElement{name, node_p, node_n, value});
            }
            "D" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_p = iter.next().unwrap().parse::<u32>()?;
                let node_n = iter.next().unwrap().parse::<u32>()?;
                let value = match iter.next() {
                    Some(x) => Some(x.parse::<f64>()?),
                    None => None
                };
                
                diodes.push(Diode{name, node_p, node_n, value});
            }
            "QN" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_c = iter.next().unwrap().parse::<u32>()?;
                let node_b = iter.next().unwrap().parse::<u32>()?;
                let node_e = iter.next().unwrap().parse::<u32>()?;
                let value = match iter.next() {
                    Some(x) => Some(x.parse::<f64>()?),
                    None => None
                };
                let polarity = Polarity::Ntype;
                
                bjts.push(BJT{name, node_c, node_b, node_e, value, polarity});
            }
            "QP" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_c = iter.next().unwrap().parse::<u32>()?;
                let node_b = iter.next().unwrap().parse::<u32>()?;
                let node_e = iter.next().unwrap().parse::<u32>()?;
                let value = match iter.next() {
                    Some(x) => Some(x.parse::<f64>()?),
                    None => None
                };
                let polarity = Polarity::Ptype;
                
                bjts.push(BJT{name, node_c, node_b, node_e, value, polarity});
            }
            "MN" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_d = iter.next().unwrap().parse::<u32>()?;
                let node_g = iter.next().unwrap().parse::<u32>()?;
                let node_s = iter.next().unwrap().parse::<u32>()?;
                let value = match iter.next() {
                    Some(x) => Some(x.parse::<f64>()?),
                    None => None
                };
                let polarity = Polarity::Ntype;
                
                fets.push(MOSFET{name, node_d, node_g, node_s, value, polarity});
            }
            "MP" => {
                let mut iter = remaining.split_whitespace();
                let name = iter.next().unwrap().parse::<u32>()?;
                let node_d = iter.next().unwrap().parse::<u32>()?;
                let node_g = iter.next().unwrap().parse::<u32>()?;
                let node_s = iter.next().unwrap().parse::<u32>()?;
                let value = match iter.next() {
                    Some(x) => Some(x.parse::<f64>()?),
                    None => None
                };
                let polarity = Polarity::Ptype;
                
                fets.push(MOSFET{name, node_d, node_g, node_s, value, polarity});
            }
            _ => panic!("Unexpected element type")
        }
    }
    
    let num_res = resistors.len();
    let num_cs = current_sources.len();
    let num_vs = voltage_sources.len();

    println!("Found {num_res} resistors");
    println!("Found {num_cs} CS");
    println!("Found {num_vs} VS");

    println!("Done!");

    Ok(())
}

fn new_grouped_element(remaining: String) -> Result<GroupedElement> {
    let mut iter = remaining.split_whitespace();
    let name = iter.next().unwrap().parse::<u32>()?;
    let node_p = iter.next().unwrap().parse::<u32>()?;
    let node_n = iter.next().unwrap().parse::<u32>()?;
    let value = iter.next().unwrap().parse::<f64>()?;
    let is_group_2 = match iter.next() {
        Some(_) => Some(true),
        None => None
    };
    
    Ok(GroupedElement {name, node_p, node_n, value, is_group_2})
}

fn parse_type(input: String) -> Result<(String, String)> {
    let mut chars = input.chars().peekable();

    let mut element_type = String::new();
    while chars.peek().unwrap().is_alphabetic() {
        element_type.push(chars.next().unwrap());
    }

    Ok((chars.collect(), element_type))
}
