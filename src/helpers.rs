use crate::spinner::Spinner;
use std::{fs, path::Path, process::Command};

pub fn delete_all_in_dir<P: AsRef<Path>>(path: P, name: &str) {
    let mut spinner = Spinner::new();
    spinner.start(&format!("Cleaning `{name}`..."));

    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    let _ = fs::remove_file(&entry_path);
                } else if entry_path.is_dir() {
                    let _ = fs::remove_dir_all(&entry_path);
                }
            }

            spinner.stop(Ok(&format!("Cleaned `{name}`!")));
        }

        Err(_err) => {
            spinner.stop(Err(&format!("Failed to clean `{name}`")));
        }
    }
}

pub fn setup_and_run_cleanmgr() {
    // sageset:1
    let mut spinner = Spinner::new();
    spinner.start("Running `cleanmgr`...");

    let _sageset = Command::new("cleanmgr").arg("/sageset:1").status();

    // sagerun:1
    match Command::new("cleanmgr").arg("/sagerun:1").status() {
        Ok(_status) => spinner.stop(Ok(&format!("Done `cleanmgr`"))),
        Err(_err) => spinner.stop(Err(&format!("Failed to run `cleanmgr`"))),
    }
}

pub fn tree_cleanup(drive: &str) {
    let mut spinner = Spinner::new();
    spinner.start(&format!("Tree {drive} ..."));

    // Run synchronously and wait for completion
    match Command::new("cmd").args(["/C", &format!("tree {drive} > NUL 2>&1")]).status() {
        Ok(_status) => spinner.stop(Ok(&format!("Done tree {drive} !"))),
        Err(_err) => spinner.stop(Err(&format!("Failed to tree {drive}"))),
    };
}
