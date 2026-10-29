//! parse_args
//!
//! We have very few options, so we're going to parse arguments manually.  This will also allow for
//! more flexible handling of arguments.  Besides, this simple command line interface will be
//! superseded by a fully fledged TUI soon.

/// Command line options.  All of them will have default values.
#[derive(Debug)]
pub struct Options {
    name: String,
    directory: String,
    duration: usize,
    transition: bool,
    transition_duration: usize,
    install: bool,
    copy: bool,
    overwrite: bool,
}

impl Options {
    /// Create a new Options object with default values.
    fn new() -> Self {
        Options {
            // Default wallpaper name.  This will work only one time as names must not conflict.
            name: String::from("dynawall"),
            // The default images directory is the current directory if none is provided.  It must
            // contain some images.
            directory: String::from("."),
            // Default image duration is 5 minutes (in seconds).
            duration: 5 * 60,
            // By default don't apply transitions between images.
            transition: false,
            // Default transition duration is 2 seconds.
            transition_duration: 2,
            // By default, generate the wallpaper files but don't install it.
            install: false,
            // By default link to the existing images and DON'T COPY.
            // WARNING: If you later on remove or rename the images you will break the wallpaper.
            copy: false,
            // By default don't overwrite the wallpaper if it already exists.
            overwrite: false,
        }
    }
}

/// Parse command line arguments (if any) and return the Options object or an error.
pub fn parse() -> Result<Options, String> {
    let args: Vec<String> = std::env::args().collect();

    return _parse(args);
}

enum NextAs {
    Flag,
    Name,
    Directory,
    Duration,
    TransitionDuration,
}

const NAME_VALUE_ERROR: &str = "--name expects a name";
const DIR_VALUE_ERROR: &str = "--dir expects a path";
const TRANSITION_DURATION_ERROR: &str = "--transition-duration expects a number";
const IMAGE_DURATION_ERROR: &str = "--image-duration expects a number";

/// Module level function.  This is what we're gonna test.
fn _parse(args: Vec<String>) -> Result<Options, String> {
    let mut options = Options::new();

    let mut next_is = NextAs::Flag;

    for arg in &args {
        match next_is {
            NextAs::Name => {
                // TODO: Sanitise.
                if arg.chars().nth(0).unwrap() == '-' {
                    return Err(NAME_VALUE_ERROR.to_string());
                }
                options.name = arg.clone();
                next_is = NextAs::Flag;
            }
            NextAs::Directory => {
                // TODO: Sanitise.
                if arg.chars().nth(0).unwrap() == '-' {
                    return Err(DIR_VALUE_ERROR.to_string());
                }
                options.directory = arg.clone();
                next_is = NextAs::Flag;
            }
            NextAs::Duration => {
                let duration = arg.parse::<usize>();
                match duration {
                    Ok(n) => options.duration = n,
                    Err(_) => return Err(IMAGE_DURATION_ERROR.to_string()),
                }
                next_is = NextAs::Flag;
            }
            NextAs::TransitionDuration => {
                let duration = arg.parse::<usize>();
                match duration {
                    Ok(n) => options.transition_duration = n,
                    Err(_) => return Err(TRANSITION_DURATION_ERROR.to_string()),
                }
                // If the user passes a transition duration, ensure transitions are enabled.
                // No need to pass both the '-t' and '-td NN' flags.
                options.transition = true;
                next_is = NextAs::Flag;
            }
            NextAs::Flag => match arg.as_str() {
                "-i" | "--install" => options.install = true,
                "-t" | "--transition" => options.transition = true,
                "-c" | "--copy" => options.copy = true,
                "-o" | "--overwrite" => options.overwrite = true,
                "-d" | "--dir" | "--directory" => next_is = NextAs::Directory,
                "-n" | "--name" => next_is = NextAs::Name,
                "-id" | "--duration" | "--image-duration" => next_is = NextAs::Duration,
                "-td" | "--transition-duration" => next_is = NextAs::TransitionDuration,
                _ => return Err(format!("Invalid argument: {arg}")),
            },
        }
    }

    match next_is {
        NextAs::Name => Err(NAME_VALUE_ERROR.to_string()),
        NextAs::Directory => Err(DIR_VALUE_ERROR.to_string()),
        NextAs::Duration => Err(IMAGE_DURATION_ERROR.to_string()),
        NextAs::TransitionDuration => Err(TRANSITION_DURATION_ERROR.to_string()),
        NextAs::Flag => Ok(options),
    }
}

#[cfg(test)]
mod tests {
    use super::_parse;
    use super::DIR_VALUE_ERROR;
    use super::IMAGE_DURATION_ERROR;
    use super::NAME_VALUE_ERROR;
    use super::TRANSITION_DURATION_ERROR;

    #[test]
    fn test_parse_no_flags() {
        let args: Vec<String> = Vec::new();

        let options = _parse(args).unwrap();

        assert!(options.name.as_str() == "dynawall");
        assert!(options.directory.as_str() == ".");
        assert!(options.duration == 300);
        assert!(!options.transition);
        assert!(options.transition_duration == 2);
        assert!(!options.install);
        assert!(!options.copy);
        assert!(!options.overwrite);
    }

    // Tests individual flags.
    #[test]
    fn test_parse_with_transition() {
        let args = vec!["-t".to_string()];

        let options = _parse(args).unwrap();

        assert!(options.name.as_str() == "dynawall");
        assert!(options.directory.as_str() == ".");
        assert!(options.duration == 300);
        assert!(options.transition);
        assert!(options.transition_duration == 2);
        assert!(!options.install);
        assert!(!options.copy);
        assert!(!options.overwrite);
    }

    #[test]
    fn test_parse_with_transition_duration() {
        let args = vec!["-td".to_string(), "10".to_string()];

        let options = _parse(args).unwrap();

        assert!(options.name.as_str() == "dynawall");
        assert!(options.directory.as_str() == ".");
        assert!(options.duration == 300);
        assert!(options.transition);
        assert!(options.transition_duration == 10);
        assert!(!options.install);
        assert!(!options.copy);
        assert!(!options.overwrite);
    }

    #[test]
    fn test_parse_with_name() {
        let name = String::from("awesome");
        let args = vec!["-n".to_string(), name.clone()];

        let options = _parse(args).unwrap();

        assert!(options.name == name);
        assert!(options.directory.as_str() == ".");
        assert!(options.duration == 300);
        assert!(!options.transition);
        assert!(options.transition_duration == 2);
        assert!(!options.install);
        assert!(!options.copy);
        assert!(!options.overwrite);
    }

    #[test]
    fn test_parse_with_directory() {
        let dir_name = String::from("paht/to/some/images");
        let args = vec!["-d".to_string(), dir_name.clone()];

        let options = _parse(args).unwrap();

        assert!(options.name.as_str() == "dynawall");
        assert!(options.directory == dir_name);
        assert!(options.duration == 300);
        assert!(!options.transition);
        assert!(options.transition_duration == 2);
        assert!(!options.install);
        assert!(!options.copy);
        assert!(!options.overwrite);
    }

    #[test]
    fn test_parse_with_copy() {
        let args = vec!["-c".to_string()];

        let options = _parse(args).unwrap();

        assert!(options.name.as_str() == "dynawall");
        assert!(options.directory.as_str() == ".");
        assert!(options.duration == 300);
        assert!(!options.transition);
        assert!(options.transition_duration == 2);
        assert!(!options.install);
        assert!(options.copy);
        assert!(!options.overwrite);
    }

    #[test]
    fn test_parse_with_overwrite() {
        let args = vec!["-o".to_string()];

        let options = _parse(args).unwrap();

        assert!(options.name.as_str() == "dynawall");
        assert!(options.directory.as_str() == ".");
        assert!(options.duration == 300);
        assert!(!options.transition);
        assert!(options.transition_duration == 2);
        assert!(!options.install);
        assert!(!options.copy);
        assert!(options.overwrite);
    }

    #[test]
    fn test_parse_with_install() {
        let args = vec!["-i".to_string()];

        let options = _parse(args).unwrap();

        assert!(options.name.as_str() == "dynawall");
        assert!(options.directory.as_str() == ".");
        assert!(options.duration == 300);
        assert!(!options.transition);
        assert!(options.transition_duration == 2);
        assert!(options.install);
        assert!(!options.copy);
        assert!(!options.overwrite);
    }

    #[test]
    fn test_parse_with_all() {
        let name = String::from("awesome");
        let dir_name = String::from("paht/to/some/images");
        let image_duration = 100;
        let transition_duration = 10;
        let args = vec![
            "-n".to_string(),
            name.clone(),
            "-d".to_string(),
            dir_name.clone(),
            "-id".to_string(),
            format!("{image_duration}"),
            "-t".to_string(),
            "-td".to_string(),
            format!("{transition_duration}"),
            "-c".to_string(),
            "-o".to_string(),
            "-i".to_string(),
        ];

        let options = _parse(args).unwrap();

        assert!(options.name == name);
        assert!(options.directory == dir_name);
        assert!(options.duration == image_duration);
        assert!(options.transition);
        assert!(options.transition_duration == transition_duration);
        assert!(options.install);
        assert!(options.copy);
        assert!(options.overwrite);
    }

    #[test]
    fn test_parse_without_name() {
        let args = vec!["-n".to_string(), "-t".to_string()];

        let error_message = _parse(args).unwrap_err();

        assert!(error_message == NAME_VALUE_ERROR.to_string())
    }

    #[test]
    fn test_parse_without_directory() {
        let args = vec!["-d".to_string(), "-n".to_string()];

        let error_message = _parse(args).unwrap_err();

        assert!(error_message == DIR_VALUE_ERROR.to_string())
    }

    #[test]
    fn test_parse_invalid_image_duration() {
        let args = vec!["-id".to_string(), "foo".to_string()];

        let error_message = _parse(args).unwrap_err();

        assert!(error_message == IMAGE_DURATION_ERROR.to_string())
    }

    #[test]
    fn test_parse_invalid_transition_duration() {
        let args = vec!["-td".to_string(), "foo".to_string()];

        let error_message = _parse(args).unwrap_err();

        assert!(error_message == TRANSITION_DURATION_ERROR.to_string())
    }
}
