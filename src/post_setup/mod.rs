use crate::database::{
    manage::{self, setup_database},
    model::Project,
};

pub mod editor;

pub fn run_post_setup(project: &Project) -> color_eyre::Result<()> {
    editor::run_editor_setup(std::path::Path::new(&project.path))?;

    // Add project to database
    manage::add_project(&setup_database().expect("Failed to get database"), project)?;
    Ok(())
}
