use std::path::Path;

use anyhow::{anyhow, bail, Result};

pub fn rust_scaffold_add_to_linked_projects(project_path: &Path) -> Result<()> {
    // edit the vscode settings.json to add the new project to the linked projects
    let mut settings_json: serde_json::Value =
        if let Ok(settings_json) = std::fs::File::open(".vscode/settings.json") {
            // if we can open the file, parse it
            match serde_json::from_reader(settings_json) {
                Ok(serdesettings_json) => serdesettings_json,
                Err(e) if e.is_eof() => serde_json::Value::Object(serde_json::Map::new()),
                Err(e) => {
                    bail!("\tCould not parse .vscode/settings.json: {}", e);
                }
            }
        } else {
            // otherwise create a new one
            std::fs::create_dir_all(".vscode")
                .map_err(|e| anyhow!("\tCould not create .vscode directory: {}", e.to_string()))?;
            serde_json::Value::Object(serde_json::Map::new())
        };

    // append to or create the linked projects array
    let new_linked_project = serde_json::Value::String(project_path.to_string_lossy().to_string());

    match settings_json.get_mut("rust-analyzer.linkedProjects") {
        Some(serde_json::Value::Array(linked_projects))
            if !linked_projects.contains(&new_linked_project) =>
        {
            linked_projects.push(new_linked_project);
        }
        Some(serde_json::Value::Array(_)) => {}
        Some(v) => {
            // if it's not an array, replace it with an array containing the new linked project
            *v = serde_json::Value::Array(vec![new_linked_project]);
        }
        None => {
            settings_json.as_object_mut().unwrap().insert(
                "rust-analyzer.linkedProjects".to_string(),
                serde_json::Value::Array(vec![new_linked_project]),
            );
        }
    }

    // write the settings.json back to disk, pretty printed
    match serde_json::to_string_pretty(&settings_json) {
        Ok(pretty_settings) => std::fs::write(".vscode/settings.json", pretty_settings)
            .map_err(|e| anyhow!("\tCould not write .vscode/settings.json: {}", e.to_string()))?,
        Err(e) => {
            bail!("Could not serialize .vscode/settings.json: {}", e);
        }
    };

    Ok(())
}
