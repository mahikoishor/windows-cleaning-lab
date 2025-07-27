use std::{env, process::Command};

pub fn run_as_admin() {
    println!("Restarting with administrator privileges...");

    let exe = env::current_exe().expect("Failed to get current exe path");
    let args: Vec<String> = env::args().skip(1).collect();
    let mut _cmd = Command::new("powershell");
    let mut arg_str = format!("-File \"{}\"", exe.display());
    for arg in &args {
        arg_str.push(' ');
        arg_str.push_str(&format!("\"{}\"", arg));
    }

    // Use Start-Process with -Verb runAs for elevation
    match Command::new("powershell")
        .arg("-Command")
        .arg(format!("Start-Process -FilePath '{}'", exe.display()))
        .arg("-Verb")
        .arg("runAs")
        .status()
    {
        Ok(_) => println!("Launched as admin."),
        Err(err) => eprintln!("Failed to relaunch as admin: {err}"),
    }
}
