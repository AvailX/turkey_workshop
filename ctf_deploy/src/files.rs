use dyn_fmt::AsStrFormatExt;
// use snarkvm::package::Package;
// use snarkvm::prelude::block::Transaction;
// use snarkvm::prelude::query::*;
// use snarkvm::prelude::store::helpers::memory::*;
// use snarkvm::prelude::store::*;
use snarkvm::prelude::*;

use snarkvm::prelude::Result;
use std::fs::{create_dir, File, OpenOptions};
use std::path::Path;

use crate::aleo_code::{
    AVAIL_CTF_GOOSE_ALEO, AVAIL_CTF_GOOSE_JSON, AVAIL_CTF_GOOSE_LEO, MAIN_ALEO, PROGRAM_JSON,
    PROG_COUNTRYMAN_ID, PROG_GOOSE, PROG_GOOSE_ID, PROG_GOOSE_LEO,
};
use crate::prog_addr;

pub fn create_file(base: &Path, filename: &str, content: &str) -> Result<()> {
    let file_path = base.join(filename);

    // Attempt to create the file and write the text to it
    match File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error writing '{}': {}", file_path.display(), e);
            } else {
                println!("'{}' created.", file_path.display());
            }
        }
        Err(e) => {
            eprintln!("Error creating '{}': {}", file_path.display(), e);
            return Err(e.into());
        }
    }

    Ok(())
}

pub fn create_project_dir(base: &Path, index: &str) -> Result<String> {
    let project_name = PROG_GOOSE.format(&[index]);
    let project_path = base.join(project_name);

    // Attempt to create the directory
    match create_dir(&project_path) {
        Ok(_) => println!("Directory '{}' created.", project_path.display()),
        Err(e) => {
            eprintln!("Error creating '{}': {}", project_path.display(), e);
            return Err(e.into());
        }
    }

    Ok(project_path.to_string_lossy().to_string())
}

pub fn create_full_project(base: &Path, index: &str) -> Result<(String, String)> {
    let project = create_project_dir(base, index)?;
    let project = Path::new(&project);
    println!();

    let cman_id = PROG_COUNTRYMAN_ID.format(&[index]);
    let cman_addr = prog_addr(&cman_id)?;
    println!();

    let content_aleo = AVAIL_CTF_GOOSE_ALEO.format(&[index, &cman_addr]);
    create_file(project, MAIN_ALEO, &content_aleo)?;

    let content_json = AVAIL_CTF_GOOSE_JSON.format(&[index]);
    create_file(project, PROGRAM_JSON, &content_json)?;

    let goose_leo = PROG_GOOSE_LEO.format(&[index]);
    let content_leo = AVAIL_CTF_GOOSE_LEO.format(&[index, &cman_addr, index, &cman_addr]);
    create_file(project, &goose_leo, &content_leo)?;
    println!();

    let ret = (
        PROG_GOOSE_ID.format(&[index]),
        project.to_string_lossy().to_string(),
    );
    Ok(ret)
}

pub fn write_file_line(base: &Path, filename: &str, line: &str) -> Result<()> {
    // write pk to file
    let file_path = base.join(filename);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open(file_path)?;

    // Append the line to the file
    writeln!(file, "{}", line)?;

    Ok(())
}
