use std::collections::HashMap;
use std::env;
use std::io::stdin;

use regex::Regex;

#[derive(Debug)]
struct Molecule<'a> {
    formula: &'a str,
    amount: u32,
    elements: HashMap<&'a str, u32>,
}

enum DevState {
    Testing,
    Deploy,
}

fn main() {
    let test_string = "2N_2H_4 + N_2O_4 -> 3N_2 + 4H_2O_15";
    let molecule_pattern = r"(\d)*(([A-Z][a-z]?_\d*)|([A-Z][a-z]?))*";
    let equation_regex = format!("{} \\+ {} -> {} \\+ {}", molecule_pattern, molecule_pattern,
                                 molecule_pattern, molecule_pattern);
    let equation_regex = Regex::new(&equation_regex).unwrap();
    let mut input_string = String::new();

    let mut dev_state: DevState = DevState::Deploy;

    let testing = env::var("TESTING").expect("Enter the TESTING ENV");
    if testing.parse::<u32>().unwrap() == 1 {
        dev_state = DevState::Testing;
    }

    loop {
        println!("Enter a chemical equation to balance!");

        match dev_state {
            DevState::Testing => {
                input_string = test_string.parse().unwrap();
                break;
            },
            DevState::Deploy => {
                match stdin().read_line(&mut input_string) {
                    Ok(_) => {
                        println!();
                        input_string.pop();
                        if input_string.is_empty() {
                            println!("Input cannot be empty!")
                        } else if equation_regex.is_match(&input_string) {
                            break;
                        } else {
                            println!("Does not match the format")
                        }
                        println!();
                        input_string = String::new();
                    }
                    Err(_) => {
                        println!();
                        println!("Enter a valid equation!");
                        println!();
                        input_string = String::new();
                    }
                };
            },
        }
    }

    println!();
    println!("Input String: {}", input_string);
    println!("Test String: {}", test_string);
    println!();

    let reactants_and_products: Vec<&str> = input_string.split(" -> ").collect();
    let reactants: Vec<&str> = reactants_and_products[0].split(" + ").collect();
    let products: Vec<&str> = reactants_and_products[1].split(" + ").collect();

    println!("Reactants and Products: {:?}", reactants_and_products);
    println!("Reactants: {:?}", reactants);
    println!("Products: {:?}", products);
    println!();

    let mut reactant_molecules: Vec<Molecule> = vec![];
    let mut product_molecules: Vec<Molecule> = vec![];

    for reactant in reactants {
        reactant_molecules = create_molecule(reactant_molecules, reactant);
    }

    for product in products {
        product_molecules = create_molecule(product_molecules, product);
    }

    println!("Reactant Molecules: {:?}", reactant_molecules);
    println!("Product Molecules: {:?}", product_molecules);
}

fn create_molecule<'a>(mut molecules: std::vec::Vec<Molecule<'a>>, input_molecule: &'a str) -> Vec<Molecule<'a>> {
    let molecule_regex = Regex::new( r"(\d)*(([A-Z][a-z]?_\d*)|([A-Z][a-z]?))*").unwrap();
    let element_regex = Regex::new(r"([A-Z][a-z]?_\d*|[A-Z][a-z]?)").unwrap();

    let caps = molecule_regex.captures(input_molecule).unwrap();
    let amount: u32 = caps.get(1).map_or(1, |m| match m.as_str().parse::<u32>() {
        Ok(amount) => amount,
        Err(_) => panic!("Amount invalid"),
    });

    let mut elements = HashMap::new();

    for element_match in element_regex.find_iter(input_molecule) {
        let element_parts: Vec<&str> = element_match.as_str().split('_').collect();

        let element = element_parts[0];
        let count = element_parts.get(1).map_or(1, |m| m.parse::<u32>().unwrap());

        elements.insert(element, count);
    }

    let molecule = Molecule {
        formula: caps.get(0).unwrap().as_str(),
        amount,
        elements,
    };

    println!("Molecule: {}", molecule.formula);
    println!("Amount: {}", molecule.amount);
    println!("Elements: {:?}", molecule.elements);
    println!();

    molecules.push(molecule);

    molecules
}
