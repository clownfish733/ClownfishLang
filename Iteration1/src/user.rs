fn usage() {
    println!("Expected -c|--config and -p|--program flags");
}

/*
#[derive(Default, Debug)]
pub struct LangConfig;
*/
type LangConfig = String;
//TODO implement deserializer for LangConfig
type FilePath = String;

#[derive(Default, Debug)]
pub struct UserIO {
    config_path: FilePath,
    config_file: LangConfig,
    program_path: FilePath,
    program_file: String,
    output_code: Option<String>,
    output_path: Option<String>,
}

impl UserIO {
    //returns None if incorrect Usage or Invalid paths with error message already displayed
    pub fn parse_args() -> Option<Self> {
        let mut args = std::env::args().skip(1);
        let mut config_path = None;
        let mut program_path = None;

        while let Some(a) = args.next() {
            match a.as_str() {
                "-c" | "--config" => {
                    config_path = args.next();
                }
                "-p" | "--program" => {
                    program_path = args.next();
                }
                _ => {
                    usage();
                    return None;
                }
            }
        }
        let config_path = match config_path {
            Some(p) => p,
            None => {
                usage();
                return None;
            }
        };
        let program_path = match program_path {
            Some(p) => p,
            None => {
                usage();
                return None;
            }
        };
        let config_file = UserIO::read_file(&config_path)?;
        let program_file = UserIO::read_file(&program_path)?;

        Some(UserIO {
            config_path,
            config_file,
            program_path,
            program_file,
            output_code: None,
            //TODO: take either from program_path or except arguement
            output_path: None,
        })
    }

    //attemts to read file if unreadable displays error message and returns none
    fn read_file(path: &String) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("Unable to read {}: {}", path, e);
                None
            }
        }
    }
    pub fn get_src(&self) -> String {
        self.program_file.clone()
    }

    pub fn add_code(&mut self, code: String) {
        self.output_code = Some(code);
    }

    pub fn output(&mut self) {
        todo!("Write to file Optionally : compile file | run file (depending on flags)");
    }
}
