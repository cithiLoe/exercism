struct Token {
    token: String,
}

pub struct WordProblem {
    command: String,
}

impl WordProblem {
    pub fn new(command: &str) -> Self {
        WordProblem { command: command.to_string() }
    }

    pub fn answer(&self) -> Result<i32, &'static str> {
        let mut tokens = self.tokenize();

        if tokens.len() % 2 == 1 || tokens.len() < 3 {
            Err("Wrong expression")
        } else {
            let mut answer = tokens.remove(0).token.parse::<i32>().unwrap();
            while tokens.len() > 1 {
                let operator = tokens.remove(0).token;
                let operand = tokens.remove(0).token.parse::<i32>().unwrap();
                if operator == "minus".to_string() {
                    answer = answer - operand;
                } else if operator == "plus".to_string() {
                    answer = answer + operand;
                } else if operator == "multiplied".to_string() {
                    answer = answer * operand;
                } else if operator == "divided".to_string() {
                    answer = answer / operand;
                }
            }
            Ok(answer)
        }
    }

    fn tokenize(&self) -> Vec<Token> {
        self.command
            .split(|c: char| c.is_whitespace() || c == '?')
            .filter(|&w| is_valid_token(w))
            .map(|w| Token { token: w.to_string() })
            .collect()
    }
}

fn is_valid_token(token: &str) -> bool {
    is_operand(token) || is_operator(token)
}

fn is_operand(token: &str) -> bool {
    token.chars().all(|c| c.is_numeric() || c == '-')
}

fn is_operator(token: &str) -> bool {
    token == "plus".to_string() || token == "minus".to_string() ||
    token == "multiplied".to_string() || token == "divided".to_string()
}
