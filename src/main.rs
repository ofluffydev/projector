use clap::{Parser, Subcommand};
use database::manage::setup_database;
use database::model::Project;
use state::languages::ProgrammingLanguage;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use ui::{grab_string, interactive_setup, not_implemented_warning, settings, yn};

mod database;
mod post_setup;
mod scaffold;
mod state;
mod ui;
pub mod util;

#[derive(Parser)]
#[command(name = "projector")]
#[command(about = "Projector CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        lang: Option<String>,
        project_name: Option<String>,
    },
    Settings,
    Gallery,
    Add {
        path: Option<String>,
    },
    Clear,
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { lang, project_name } => {
            if lang.is_none() {
                interactive_setup::show()?;
                return Ok(());
            }
            if lang.as_deref() != Some("rust") {
                not_implemented_warning::show(format!(
                    "Scaffolding for language '{}' is not implemented yet.",
                    lang.as_deref().expect("Language is required")
                ))?;
                return Ok(());
            }
            let name = match project_name {
                Some(n) => n.trim().to_string(),
                None => {
                    print!("Project name? (press Enter to use current directory): ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    input.trim().to_string()
                }
            };
            if name.is_empty() {
                let status = Command::new("cargo").arg("init").status()?;
                if !status.success() {
                    eprintln!("Failed to run cargo init in current directory");
                }
            } else {
                std::fs::create_dir_all(&name)?;
                let status = Command::new("cargo")
                    .args(["init", "--name", &name, &name])
                    .status()?;
                if !status.success() {
                    eprintln!("Failed to run cargo init");
                }
            }
        }
        Commands::Settings => {
            settings::show()?;
            let file_path = confy::get_configuration_file_path("projector", None)?;
            let file_path_str = file_path.to_string_lossy().replace("\\", "/");
            println!("Settings stored at: {:?}", file_path_str);
        }
        Commands::Gallery => {
            ui::gallery::show()?;
        }
        Commands::Add { path } => {
            let path_str = if let Some(arg) = path {
                arg
            } else {
                &grab_string::ask("Path to the project?")?
            };

            if path_str.is_empty() {
                println!("Path cannot be empty.");
                return Ok(());
            }

            let path = Path::new(&path_str);
            if !path.exists() {
                println!("Path does not exist.");
                return Ok(());
            }
            if !path.is_dir() {
                println!("Path is not a directory.");
                return Ok(());
            }

            // Convert path to absolute path
            let path = path.canonicalize()?;

            let project_type = grab_string::ask("Project type? (rust, python, etc.)")?;

            if project_type.is_empty() {
                println!("Project type cannot be empty.");
                return Ok(());
            }
            // Ask for project name
            let project_name = grab_string::ask("Project name?")?;
            if project_name.is_empty() {
                println!("Project name cannot be empty.");
                return Ok(());
            }

            let project_type_mapped = map_project_type(&project_type);

            let mut project = Project::new(project_type_mapped, project_name, &path);

            project.insert(&setup_database()?)?;

            // Print all the project info
            println!("Project name: {}", project.name);
            println!("Project type: {:?}", project.type_lang);
            println!("Project path: {}", project.path);

            println!("Project added!");
        }
        Commands::Clear => {
            if yn::ask("Are you sure you want to clear the database?")? {
                database::manage::clear()?;
            } else {
                println!("Aborted clearing the database.");
            }
        }
    }

    Ok(())
}

fn map_project_type(project_type: &str) -> ProgrammingLanguage {
    match project_type {
        "rust" => ProgrammingLanguage::Rust,
        "python" => ProgrammingLanguage::Python,
        "javascript" => ProgrammingLanguage::JavaScript,
        "java" => ProgrammingLanguage::Java,
        "c" => ProgrammingLanguage::C,
        "cpp" => ProgrammingLanguage::CPlusPlus,
        "go" => ProgrammingLanguage::Go,
        "swift" => ProgrammingLanguage::Swift,
        "kotlin" => ProgrammingLanguage::Kotlin,
        "csharp" => ProgrammingLanguage::CSharp,
        "FORCE_UNIMPLEMENTED" => ProgrammingLanguage::UnimplementedForced,

        // All others, default to Unknown
        _ => unimplemented!("Project type '{}' is not implemented yet.", project_type),
    }
}
