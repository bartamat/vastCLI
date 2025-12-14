use colored::Colorize;
use dialoguer::Input;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const API_KEY_FILE: &str = "vast_key";

pub struct Config {
    api_key_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_key_path: PathBuf::from(API_KEY_FILE),
        }
    }

    pub fn get_api_key(&self) -> io::Result<String> {
        match self.read_api_key_from_file() {
            Ok(key) if !key.trim().is_empty() => {
                println!(
                    "{} API key from {}",
                    "Using".green(),
                    API_KEY_FILE.cyan()
                );
                Ok(key.trim().to_string())
            }
            _ => {
                println!(
                    "{} API key file not found or empty",
                    "Warning:".yellow()
                );
                self.prompt_and_save_api_key()
            }
        }
    }

    fn read_api_key_from_file(&self) -> io::Result<String> {
        fs::read_to_string(&self.api_key_path)
    }

    fn prompt_and_save_api_key(&self) -> io::Result<String> {
        let api_key: String = Input::new()
            .with_prompt("VastAI API Key (Bearer token)")
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if api_key.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "API key cannot be empty",
            ));
        }

        self.save_api_key(&api_key)?;

        println!(
            "{} API key saved to {}",
            "Success:".green(),
            API_KEY_FILE.cyan()
        );
        println!(
            "{} Add '{}' to .gitignore to avoid committing your API key",
            "Tip:".yellow(),
            API_KEY_FILE
        );

        Ok(api_key.trim().to_string())
    }

    fn save_api_key(&self, api_key: &str) -> io::Result<()> {
        let mut file = fs::File::create(&self.api_key_path)?;
        file.write_all(api_key.trim().as_bytes())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_api_key(&self) -> io::Result<String> {
        println!("{}", "Updating API key...".cyan());
        self.prompt_and_save_api_key()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
