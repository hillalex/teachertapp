use std::fs;
use std::path::{Path, PathBuf};
use schemars::schema_for;
use teachertapp::models::*;

type List = Vec<School>;

fn main() {
    let schema_root = get_schema_root();
    let schemas = vec![
        ("ErrorDetail", schema_for!(ErrorDetail)),
        ("School", schema_for!(School)),
        ("CreateSchool", schema_for!(CreateSchool)),
        ("List", schema_for!(List))];

    for s in schemas.iter() {
        let contents = serde_json::to_string_pretty(&s.1).expect("Wrote JSON string");
        write_schema_to_file(&schema_root, get_schema_path(s.0), contents)
    }
}

fn get_schema_path(model_name: &str) -> PathBuf {
    PathBuf::from(format!("{}.schema.json", model_name))
}

fn get_schema_root() -> PathBuf {
    let root = project_root::get_project_root().expect("Found project root");
    root.join("schema")
}

fn write_schema_to_file(schema_root: &Path, filename: PathBuf, contents: String) {
    let path = schema_root.join(filename);
    fs::File::create(&path).expect("Created schema file");
    fs::write(path, contents).expect("Wrote schema")
}
