use eyre::Result;
use std::collections::HashMap;

fn main() -> Result<()>{

    let input = std::io::read_to_string(std::io::stdin())?;
    let input = input.chars().collect::<Vec<char>>();

    println!("part one: {}", find_start_message(&input, 4));
    println!("part two: {}", find_start_message(&input, 14));

    Ok(())
}

fn find_start_message(input: &Vec<char>, length: usize) -> usize {
    let mut buffer: HashMap<char, usize> = HashMap::new();
    let mut i = 0usize;

    for c in input.iter(){

        buffer.entry(*c)
            .and_modify(|v| *v+= 1)
            .or_insert(1);

        i += 1;
        if i < length {continue;}
        if buffer.len() >= length {break;}

        match buffer.get(&input[i-length]).unwrap() {
            1 => {buffer.remove(&input[i-length]);},
            v => {buffer.insert(input[i-length], v-1);}
        }
    }
    return i;

}