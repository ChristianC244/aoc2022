use eyre::Result;
use regex::Regex;

fn main() -> Result<()>{

    let input = std::io::read_to_string(std::io::stdin())?;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;

    let mut part_one_counter = 0usize;
    let mut part_two_counter = 0usize;
    
    input.lines().for_each(|row| {
        // This works directly with bits, since the range does not exceed the 99 id I can use u128
        let section1: u128;
        let section2: u128;
        let cap = re.captures(row).unwrap();

        let start = &cap[1].parse::<usize>().unwrap();
        let end = &cap[2].parse::<usize>().unwrap() + 1;
        section1 = (1 << end) - (1 << start);

        let start = &cap[3].parse::<usize>().unwrap();
        let end = &cap[4].parse::<usize>().unwrap() + 1;
        section2 = (1 << end) - (1 << start);

        match section1 & section2 {
            0 => (),
            x if x == section1 => {
                part_one_counter += 1;
                part_two_counter += 1
            },
            x if x == section2 => {
                part_one_counter += 1;
                part_two_counter += 1
            },
            _ => part_two_counter += 1            
        }

    });

    println!("Part one: {}", part_one_counter);
    println!("Part two: {}", part_two_counter);

    Ok(())
}
