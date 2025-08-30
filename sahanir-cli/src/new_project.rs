use anyhow::{Context, Result};
use handlebars::Handlebars;
use include_dir::{include_dir, Dir, DirEntry};
use serde_json::json;
use std::fs;
use std::path::Path;

// Embed the template directory into the binary.
// The path is relative to the `sahanir-cli` crate's `Cargo.toml`.
static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

pub async fn create_new_project(name: &str) -> Result<()> {
    let project_path = Path::new(name);

    // Check if the directory already exists to avoid overwriting.
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists.", name);
    }

    println!("Creating a new SahaniR universe in '{}'...", project_path.display());

    // Setup Handlebars for templating.
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true); // Fail on missing variables.

    // Prepare data for templates.
    let data = json!({
        "project_name": name,
    });

    // Recursively walk the embedded template directory and create the project structure.
    walk_and_create(TEMPLATE_DIR.entries(), project_path, &handlebars, &data)?;

    println!("\nUniverse '{}' created successfully!", name);
    println!("To get started, run:");
    println!("  cd {}", name);
    println!("  # Follow your local testing instructions to run the project.");

    Ok(())
}

fn walk_and_create(
    entries: &'static [DirEntry],
    base_path: &Path,
    handlebars: &Handlebars,
    data: &serde_json::Value,
) -> Result<()> {
    for entry in entries {
        let path = base_path.join(entry.path());
        match entry {
            DirEntry::Dir(d) => {
                fs::create_dir_all(&path).context(format!("Failed to create directory {:?}", path))?;
                // Recurse into the subdirectory
                walk_and_create(d.entries(), base_path, handlebars, data)?;
            }
            DirEntry::File(file) => {
                // Determine the final path (stripping .hbs if present).
                let final_path = if path.extension().and_then(|s| s.to_str()) == Some("hbs") {
                    path.with_extension("")
                } else {
                    path
                };

                // Create parent directory if it doesn't exist.
                if let Some(parent) = final_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // Read the raw template content.
                let raw_content =
                    file.contents_utf8().context("Failed to read embedded file content")?;

                // Render the content with Handlebars.
                let rendered_content = handlebars
                    .render_template(raw_content, data)
                    .context("Failed to render template")?;

                // Write the final content to the file.
                fs::write(&final_path, rendered_content)
                    .context(format!("Failed to write file {:?}", final_path))?;
            }
        }
    }
    Ok(())
}
