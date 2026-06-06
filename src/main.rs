use std::fs;

fn main() {
    let totes = parse_input("INPUT");

    let mut score = 0;

    let mut stack: Vec<&Tote> = vec![];

    totes.iter().for_each(|tote| {
        if stack.is_empty() || tote.can_add(stack.last().unwrap()) {
            stack.push(tote);
        } else {
            score += score_and_clear_stack(&mut stack);
            stack.clear();
            stack.push(tote);
        }
    });

    score += score_stack(&stack);

    dbg!(score);
}

fn score_and_clear_stack(stack: &mut Vec<&Tote>) -> usize {
    let score = score_stack(stack);
    stack.clear();
    score
}

fn score_stack(stack: &[&Tote]) -> usize {
    match stack.len() {
        1 => 50,
        2 => 25,
        5 => 35,
        x if x % 4 == 0 => 0,
        x if x % 4 == 1 => 30,
        x if x % 4 == 2 => 20,
        _ => 10,
    }
}

#[derive(Debug)]
struct Tote {
    temp: Temp,
    weight: usize,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Temp {
    Frozen,
    Chilled,
    Ambient,
}

impl Tote {
    fn can_add(&self, below_tote: &Tote) -> bool {
        self.weight <= below_tote.weight && self.temp >= below_tote.temp
    }
}

fn parse_input(file: &str) -> Vec<Tote> {
    let contents = fs::read_to_string(file).unwrap();
    let chunks = contents.split_whitespace();

    let mut totes = vec![];

    chunks.into_iter().for_each(|chunk| {
        let temp = match chunk.chars().next().unwrap() {
            'A' => Temp::Ambient,
            'C' => Temp::Chilled,
            'F' => Temp::Frozen,
            _ => panic!("non valid temp"),
        };
        let weight = chunk[1..].parse::<usize>().unwrap();
        let tote = Tote { weight, temp };
        totes.push(tote);
    });

    totes
}
