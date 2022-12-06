use eyre::Result;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Shape {
    /// Self is your hand, while other is the other player's hand.
    fn beats(&self, other: &Shape) -> usize {
        
        let points = match (self, other) {
            (Shape::Rock, Shape::Scissors) |
            (Shape::Paper, Shape::Rock) |
            (Shape::Scissors, Shape::Paper) => 6,

            (Shape::Rock, Shape::Rock) |
            (Shape::Paper, Shape::Paper) |
            (Shape::Scissors, Shape::Scissors) => 3,

            _ => 0,
        };

        let p = match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        points + p
    }
}

impl Outcome {
    fn play_score(&self, other: &Shape) -> usize {

        let points = match (self, other) {
            (Outcome::Loss, Shape::Paper) |
            (Outcome::Draw, Shape::Rock) |
            (Outcome::Win, Shape::Scissors) => 1, // Rock

            (Outcome::Loss, Shape::Scissors) |
            (Outcome::Draw, Shape::Paper) |
            (Outcome::Win, Shape::Rock) => 2, // Rock

            _ => 3 // Scissor      
        
        };

        let p = match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win =>  6
        };

        points + p

    }
}

fn main() -> Result<()>{

    let input = std::io::read_to_string(std::io::stdin())?;
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));


    Ok(())
}

fn part_one(input: &String) -> usize{
    
    // Input parsing
    let input = input.lines().map(|x| {

        let p1 = x.chars().nth(0).unwrap();
        let p2 = x.chars().nth(2).unwrap();

        let p1 = match p1 {
            'A'  => Shape::Rock,
            'B'  => Shape::Paper,
            'C'  => Shape::Scissors,
            _ => panic!("unknown shape")
        };

        let p2 = match p2 {
            'X'  => Shape::Rock,
            'Y'  => Shape::Paper,
            'Z'  => Shape::Scissors,
            _ => panic!("unknown shape")
        };

        (p1, p2)
        
    }).collect::<Vec<(Shape, Shape)>>();

    input.iter().fold(0usize, |acc, vec| {
        acc + vec.1.beats(&vec.0)
    })
}
fn part_two(input: &String) -> usize {

    // Input parsing
    let input = input.lines().map(|x| {

        let p1 = x.chars().nth(0).unwrap();
        let p2 = x.chars().nth(2).unwrap();

        let p1 = match p1 {
            'A'  => Shape::Rock,
            'B'  => Shape::Paper,
            'C'  => Shape::Scissors,
            _ => panic!("unknown shape")
        };

        let p2 = match p2 {
            'X'  => Outcome::Loss,
            'Y'  => Outcome::Draw,
            'Z'  => Outcome::Win,
            _ => panic!("unknown shape")
        };

        (p1, p2)
        
    }).collect::<Vec<(Shape, Outcome)>>();

    input.iter().fold(0usize, |acc, (shape, outcome)| {
        acc + outcome.play_score(shape)
    })



}