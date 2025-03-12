use crate::database::model::Project;
use directories::ProjectDirs;
use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let project = ProjectDirs::from("rs", "", "projector").expect("Failed to get project dirs");
    let data_dir = project.data_dir();

    let db_path = data_dir.join("projects.db");
    std::fs::create_dir_all(data_dir).expect("Failed to create project directory");

    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            type_lang TEXT NOT NULL,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            last_opened TIMESTAMP,
            created_on TIMESTAMP NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn get_all_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt =
        conn.prepare("SELECT id, type_lang, name, path, last_opened, created_on FROM projects")?;
    let project_iter = stmt.query_map([], |row| {
        Ok(Project {
            type_lang: row.get(1)?,
            name: row.get(2)?,
            path: row.get(3)?,
            last_opened: row.get(4)?,
            created_on: row.get(5)?,
        })
    })?;

    let mut projects = Vec::new();
    for project in project_iter {
        projects.push(project?);
    }
    Ok(projects)
}

pub fn add_project(conn: &Connection, project: &Project) -> Result<usize> {
    project.insert(conn)
}
