use std::env;
use std::path::PathBuf;
use tempfile::TempDir;

const _COMMAND_HISTORY_PATH: &str = ".nebula_history";

pub fn run() {
    let args: Vec<String> = env::args().collect();

    match _get_command(&args) {
        Ok(Some(command)) => match _get_command_history_linux() {
            Ok(command_history_path) => {
                print!("{}", command);
                println!("{:?}", command_history_path);
            }
            Err(e) => {
                eprintln!("Error getting command history path: {}", e);
            }
        },
        Ok(None) => {
            eprintln!("Error: Command not found.");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn _get_command(args: &[String]) -> Result<Option<String>, String> {
    if args.len() > 1 && args[1] == "cmd" {
        Ok(Some(args[2..].join(" ")))
    } else {
        Ok(None)
    }
}

fn _get_command_history_linux() -> Result<PathBuf, String> {
    if let Some(home_dir) = env::var_os("HOME") {
        let mut path = PathBuf::from(home_dir);
        path.push(_COMMAND_HISTORY_PATH);
        Ok(path)
    } else {
        Err("HOME environment variable not set".to_string())
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
            "World".to_string(),
        ];

        assert_eq!(
            _get_command(&args).unwrap(),
            Some("echo Hello World".to_string())
        );
    }

    #[test]
    fn test_get_command_it_should_return_none_if_format_command_is_incorrect() {
        let args = vec!["program".to_string(), "other_cmd".to_string()];

        assert_eq!(_get_command(&args).unwrap(), None);
    }

    #[test]
    fn test_get_command_it_should_return_none_when_no_arguments_provided() {
        let args = vec!["program".to_string()];

        assert_eq!(_get_command(&args).unwrap(), None);
    }

    #[test]
    fn test_get_command_history_path_it_should_return_correct_path() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().to_str().unwrap();

        unsafe {
            env::set_var("HOME", home_dir);
        }

        let expected_path = temp_dir.path().join(_COMMAND_HISTORY_PATH);

        assert_eq!(_get_command_history_linux().unwrap(), expected_path);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_get_command_history_path_it_should_return_error_when_home_not_set() {
        unsafe {
            env::remove_var("HOME");
        }

        assert_eq!(
            _get_command_history_linux().unwrap_err(),
            "HOME environment variable not set".to_string()
        );
    }
}
