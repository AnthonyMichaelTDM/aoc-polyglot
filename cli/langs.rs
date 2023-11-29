use std::{
    fmt::Display,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::Result;
use lazy_static::lazy_static;
use tera::Tera;

use crate::get_challenge_dir;

lazy_static! {
    static ref TERA: Tera = Tera::new("templates/**/*").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub enum SupportedLanguage {
    Rust,
    Python,
    Zig,
}

impl Display for SupportedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(f, "rust"),
            Self::Python => write!(f, "python"),
            Self::Zig => write!(f, "zig"),
        }
    }
}

impl FromStr for SupportedLanguage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "zig" => Ok(Self::Zig),
            _ => Err(anyhow::anyhow!("Invalid language")),
        }
    }
}

impl Solution for SupportedLanguage {
    fn get_working_dir(&self, day: u8, year: u16) -> Result<PathBuf, PathBuf> {
        let working_dir: PathBuf =
            get_challenge_dir(day, year).join(self.to_string().to_lowercase());

        // make sure the solution directory exists
        if working_dir.exists() {
            Ok(working_dir)
        } else {
            Err(working_dir)
        }
    }

    fn compile(&self, day: u8, year: u16) -> Result<()> {
        let mut command;
        match self {
            Self::Rust => {
                command = Command::new("cargo");
                command.args(["build", "--release"]);
            }
            Self::Python => return Ok(()), // python doesn't need to be compiled
            Self::Zig => {
                command = Command::new("zig");
                command.args(["build"]);
            }
        }

        let working_dir = self.get_working_dir(day, year).map_err(|_| {
            anyhow::anyhow!(
                "Could not find solution directory for day {} of year {}",
                day,
                year
            )
        })?;

        // run the command in the solution directory
        command
            .current_dir(working_dir)
            .stderr(Stdio::inherit())
            .stdout(Stdio::null())
            .output()?;

        Ok(())
    }

    fn run(&self, day: u8, year: u16) -> Result<String> {
        let mut command;
        match self {
            Self::Rust => {
                command = Command::new("cargo");
                command.args(["run", "--release"]);
            }
            Self::Python => {
                command = Command::new("python");
                command.args(["main.py"]);
            }
            Self::Zig => {
                command = Command::new("zig");
                command.args(["run"]);
            }
        }

        let working_dir = self.get_working_dir(day, year).map_err(|_| {
            anyhow::anyhow!(
                "Could not find solution directory for day {} of year {}",
                day,
                year
            )
        })?;

        // run the command in the solution directory
        let output = command
            .current_dir(working_dir)
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .output()?;

        // convert the output to a string
        let output = String::from_utf8(output.stdout)?;

        Ok(output)
    }

    fn scaffold(&self, day: u8, year: u16) -> Result<()> {
        // create the challenge directory
        let challenge_dir = self.get_working_dir(day, year).err().ok_or_else(|| {
            anyhow::anyhow!(
                "{lang} solution directory already exists for day {day} of year {year} at",
                lang = self.to_string(),
            )
        })?;
        std::fs::create_dir_all(&challenge_dir)?;

        // use the template for the given language
        let template_dir = match self {
            Self::Rust => format!("{}/templates/rust", env!("CARGO_MANIFEST_DIR")),
            Self::Python => format!("{}/templates/python", env!("CARGO_MANIFEST_DIR")),
            Self::Zig => format!("{}/templates/zig", env!("CARGO_MANIFEST_DIR")),
        };

        println!("Using template directory: {template_dir}");

        // copy folder
        let mut options: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;
        fs_extra::dir::copy(template_dir, get_challenge_dir(day, year), &options)?;

        // if needed, fill out .tera templates
        //  set up the context for the templates
        let mut context = tera::Context::new();
        context.insert("name", &format!("day_{day:02}_solution"));
        context.insert("year", &year);
        context.insert("day", &day);
        context.insert("language", &self.to_string());
        //  common templates
        let readme = TERA.render("README.md.tera", &context)?;
        let mut file = std::fs::File::create(challenge_dir.join("README.md"))?;
        file.write_all(readme.as_bytes())?;
        //  language specific templates
        match self {
            Self::Rust => {
                // delete the template Cargo.toml.tera we copied
                std::fs::remove_file(challenge_dir.join("Cargo.toml.tera"))?;
                // render cargo.toml
                let cargo_toml = TERA.render("rust/Cargo.toml.tera", &context)?;
                let mut file = std::fs::File::create(challenge_dir.join("Cargo.toml"))?;
                file.write_all(cargo_toml.as_bytes())?;
            }
            Self::Python | Self::Zig => {}
        }

        Ok(())
    }
}

pub trait Solution {
    /// get the working directory for the solution for a given day and year.
    /// returns and Err wrapping the solution dir if it does not exist.
    /// so, no matter what you can get the working directory for a solution, and the variant returned will tell you if it exists or not.
    ///
    /// # Errors
    ///
    /// returns an error if the solution directory does not exist.
    fn get_working_dir(&self, day: u8, year: u16) -> Result<PathBuf, PathBuf>;
    /// compile the solution for a given day and year.
    ///
    /// # Errors
    ///
    /// returns an error if the solution directory does not exist, or if there is an error compiling the solution.
    fn compile(&self, day: u8, year: u16) -> Result<()>;
    /// run the solution for a given day and year.
    ///
    /// TODO: add support for passing arguments to the solution, and for running individual parts of the solution.
    ///
    /// # Errors
    ///
    /// returns an error if the solution directory does not exist, or if there is an error running the solution.
    fn run(&self, day: u8, year: u16) -> Result<String>;
    /// scaffold the solution for a given day and year, basically creating a project template for the solution.
    ///
    /// # Errors
    ///
    /// returns an error if the solution directory already exists, the template directory does not exist, or if there is an error copying the template directory.
    ///
    fn scaffold(&self, day: u8, year: u16) -> Result<()>;
}
