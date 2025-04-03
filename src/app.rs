use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if let Some(command) = _get_command(&args) {
        print!("{}", command)
    }
}

fn _get_command(args: &[String]) -> Option<String> {
    if args.len() > 1 && args[1] == "cmd" {
        Some(args[2..].join(" "))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_it_should_return_command_if_format_is_correct() {
        let args = vec![
            "program".to_string(),
            "cmd".to_string(),
            "echo".to_string(),
            "Hello".to_string(),
            "World".to_string()
        ];

        assert_eq!(_get_command(&args), Some("echo Hello World".to_string()));
    }

    #[test]
    fn test_get_command_it_should_return_none_if_format_command_is_incorrect() {
        let args = vec![
            "program".to_string(),
            "other_cmd".to_string()
        ];

        assert_eq!(_get_command(&args), None);
    }

    #[test]
    fn test_get_command_it_should_return_none_when_no_arguments_provided() {
        let args = vec!["program".to_string()];

        assert_eq!(_get_command(&args), None);
    }
}
