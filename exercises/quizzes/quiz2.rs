#[derive(PartialEq)]
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: &Vec<(String, Command)>) -> Vec<String> {
        let mut results = Vec::new();

        for (test_string, command) in input {
            let result = match command {
                Command::Uppercase => test_string.to_uppercase(),
                Command::Trim => test_string.trim().to_string(),
                Command::Append(n) => {
                    let mut temp = test_string.clone();
                    for _ in 0..*n {
                        temp.push_str("bar");
                    }
                    temp
                }
            };

            results.push(result);
        }

        results
    }
}

fn main() {
    // Experiment if needed
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(&input);

        assert_eq!(
            output,
            vec![
                "HELLO".to_string(),
                "all roads lead to rome!".to_string(),
                "foobar".to_string(),
                "barbarbarbarbarbar".to_string(),
            ]
        );
    }
}
