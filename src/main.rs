use std::io::stdin;

fn main() {
    println!("Enter a chemical equation to balance!");
    
    let test_string = "2N_2H_4 + N_2O_4 -> 3N_2 + 4H_2O";
    
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("That's not a valid string");
    input_string.pop();
    
    println!("Input String: {}", input_string);
    println!("Test String: {}", test_string);
    
    // Split string by ' + ' and ' -> '
    let reactants_and_products: Vec<&str> = input_string.split(" -> ").collect();
    let reactants: Vec<&str> = reactants_and_products[0].split(" + ").collect();
    let products: Vec<&str> = reactants_and_products[1].split(" + ").collect();
    
    println!("Reactants and Products: {:?}", reactants_and_products);
    println!("Reactants: {:?}", reactants);
    println!("Products: {:?}", products);
}
