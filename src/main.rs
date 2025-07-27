pub mod helpers;
pub mod spinner;
pub mod utils;
pub mod run_as_admin;

use crate::{
    helpers::{delete_all_in_dir, setup_and_run_cleanmgr, tree_cleanup},
    utils::sleep,
    run_as_admin::run_as_admin,
};
use is_elevated::is_elevated;
use std::{env, path::Path};

fn main() {
    // Check for admin rights
    if !is_elevated() {
        return run_as_admin();
    }

    // Welcome Screen
    println!("=============================================");
    println!("=============================================");
    println!("=========== WINDOWS CLEANING LAB ============");
    println!("============================== mahikoishor ==");
    println!("=============================================");
    println!("\n");

    // 0. Tree
    tree_cleanup("C:/");

    // 1. Recent
    if let Some(userprofile) = env::var_os("USERPROFILE") {
        let recent = Path::new(&userprofile).join("AppData/Roaming/Microsoft/Windows/Recent");
        delete_all_in_dir(recent, "Recent Files");
    }

    // 2. Temp
    delete_all_in_dir("C:/Windows/Temp", "Windows Temp");

    // 3. %temp%
    if let Some(temp) = env::var_os("TEMP") {
        delete_all_in_dir(temp, "%TEMP% Files");
    }

    // 4. Prefetch
    delete_all_in_dir("C:/Windows/Prefetch", "Prefetch");

    // 5. Disk Cleanup (single sageset/sagerun)
    setup_and_run_cleanmgr();

    println!("\n");
    println!("All cleaning tasks completed!");

    sleep(2000);
}
