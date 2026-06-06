use std::fs;

fn main() {
    let totes = parse_input("INPUT");

    let mut score = 0;

    let mut stack: Vec<&Tote> = vec![];

    totes.iter().for_each(|tote| {
        if stack.is_empty() {
            stack.push(tote);
        } else if tote.can_add(stack.last().unwrap()) {
            stack.push(tote);
            if stack.len() == 4 {
                score += score_and_clear_stack(&mut stack);
                stack.clear();
            }
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
        3 => 10,
        _ => 0,
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
    let binding = fs::read_to_string(file).unwrap();
    let contents = binding.split_whitespace();

    let mut totes = vec![];

    contents.into_iter().for_each(|chunk| {
        let temp = match chunk.chars().next().unwrap() {
            'A' => Temp::Ambient,
            'C' => Temp::Chilled,
            'F' => Temp::Frozen,
            _ => panic!("non valid temp"),
        };
        let weight = chunk[1..].parse::<usize>().unwrap();
        let tote = Tote { weight, temp };
        dbg!(&tote);
        totes.push(tote);
    });

    totes
}
