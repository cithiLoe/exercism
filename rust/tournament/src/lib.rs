use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
struct TeamResult {
    wins: u8,
    losses: u8,
    draws: u8,
}

impl TeamResult {
    fn new() -> Self {
        TeamResult {
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    fn update(&mut self, result: GameResult) {
        match result {
            GameResult::Win => self.wins += 1,
            GameResult::Loss => self.losses += 1,
            GameResult::Draw => self.draws += 1,
        }
    }
}

impl fmt::Display for TeamResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:2} | {:2} | {:2}", self.wins, self.draws, self.losses)
    }
}

pub fn tally(input: &str) -> String {
    let mut results: HashMap<String, TeamResult> = HashMap::new();
    
    for line in input.lines() {
        let record = line.split(';').collect::<Vec<_>>();
        let left_team = record[0];
        let right_team = record[1];

        let result = match record[2] {
            "win" => GameResult::Win,
            "loss" => GameResult::Loss,
            "draw" => GameResult::Draw,
            _ => panic!("Unexpected game result"),
        };

        match result {
            GameResult::Win => {
                update_results(&mut results, left_team.to_string(), GameResult::Win);
                update_results(&mut results, right_team.to_string(), GameResult::Loss);
            }
            GameResult::Loss => {
                update_results(&mut results, left_team.to_string(), GameResult::Loss);
                update_results(&mut results, right_team.to_string(), GameResult::Win);
            }
            GameResult::Draw => {
                update_results(&mut results, left_team.to_string(), GameResult::Draw);
                update_results(&mut results, right_team.to_string(), GameResult::Draw);
            }
        }
    }
    form_table(&results)
}

fn form_table(results: &HashMap<String, TeamResult>) -> String {
    let mut table = results
        .iter()
        .map(|(team, result)| {
                 let matches = result.wins + result.draws + result.losses;
                 let points = 3 * result.wins + result.draws;
                 (team, matches, result, points)
             })
        .collect::<Vec<_>>();

    table.sort_by(|a, b| b.3.cmp(&a.3).then(a.0.cmp(&b.0)));

    let mut lines = vec![format!("{:30} | MP |  W |  D |  L |  P", "Team")];
    lines.extend(table
                     .iter()
                     .map(|&(team, matches, result, points)| {
                              format!("{:30} | {:2} | {} | {:2}", team, matches, result, points)
                          }));
    lines.join("\n")
}

fn update_results(results: &mut HashMap<String, TeamResult>, team: String, result: GameResult) {
    results.entry(team).or_insert(TeamResult::new()).update(result);
}
