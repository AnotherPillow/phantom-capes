use is_superuser::is_superuser;
use std::fs;
use regex::Regex;
use std::process::Command;
use std::path::Path;

fn main() {
    install_deps(false);
    let version = env!("CARGO_PKG_VERSION");
    let intro_text = "
    ╔══════════════════════════════════════════╗
    ║ ▄▄▄· ▄ .▄ ▄▄▄·  ▐ ▄ ▄▄▄▄▄      • ▌ ▄ ·.  ║
    ║▐█ ▄███▪▐█▐█ ▀█ •█▌▐█•██  ▪     ·██ ▐███▪ ║
    ║ ██▀·██▀▐█▄█▀▀█ ▐█▐▐▌ ▐█.▪ ▄█▀▄ ▐█ ▌▐▌▐█· ║
    ║▐█▪·•██▌▐▀▐█ ▪▐▌██▐█▌ ▐█▌·▐█▌.▐▌██ ██▌▐█▌ ║
    ║.▀   ▀▀▀ · ▀  ▀ ▀▀ █▪ ▀▀▀  ▀█▄▀▪▀▀  █▪▀▀▀ ║
    ╚══════════════════════════════════════════╝
    ";
    println!("{}", intro_text);
    println!("  ┋ Phantom v{}", version);
    if !is_superuser() {
        println!("  ┋ Please run as admin!");
        println!("  ┋ Exiting...");
        //wait for input before closing 
        get_input(true);
        return;
    }

    println!("
    
    ┋ Running as admin
    
    ┋ Please choose an option:

    [1] Install Phantom
    [2] Run Phantom
    [3] Uninstall Phantom
    [4] Exit

    ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    match input {
        "1" => install_phantom(),
        "2" => run_phantom(),
        "3" => uninstall_phantom(),
        "4" => return,
        _ => {
            println!("┋ Invalid option!");
            get_input(true);
        }
    }


}

fn get_input(display: bool) {
    let mut input = String::new();
    if display {
        println!("┋ Press enter to continue...");
    }
    std::io::stdin().read_line(&mut input).unwrap();
}

fn install_phantom() {
    install_deps(true);
    let hosts_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    let hosts = fs::read_to_string(hosts_path).unwrap();
    let lines_lines = hosts.lines();

    let mut lines: Vec<&str> = lines_lines.clone().collect();

    let lines_to_add = vec![
        "",
        "# Phantom Capes",
        "127.0.0.1 s.optifine.net",
        "127.0.0.1 s-optifine.lunarclientcdn.com",
        "107.182.233.85 ph.optifine.net",
        "107.182.233.85 ph.lunarclientcdn.com",
    ];
    for line in lines_to_add.into_iter() {
        lines.push(line);
    }
    let new_hosts = lines.join("\n");
    fs::write(hosts_path, new_hosts).unwrap();

    let _output = Command::new("ipconfig")
        .arg("/flushdns")
        .output()
        .expect("failed to execute process");

    println!("  ┋ Installed Phantom!");
    run_phantom();
}

fn uninstall_phantom() {
    let hosts_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    let hosts = fs::read_to_string(hosts_path).unwrap();

    let optifine_regex = Regex::new(r"^(\d+\.)+(\d+) (ph|s).(optifine|lunarclientcdn)\.(.+)$").unwrap();
    let lines_lines = hosts.lines();
    //turn lines into an array of lines
    let mut lines: Vec<&str> = lines_lines.clone().collect();

    for line in lines_lines.into_iter() {
        if optifine_regex.is_match(line) {
            //remove it from lines
            println!("Removing: {}", line);
            lines.retain(|&x| x != line);
        }
    }
    let new_hosts = lines.join("\n");
    fs::write(hosts_path, new_hosts).unwrap();
    println!("  ┋ Uninstalled Phantom!");
    println!("  ┋ Exiting...");
    get_input(true);
}

fn run_phantom() {
    let _output = Command::new("py")
        .arg("app.py")
        .current_dir(Path::new("server"))
        .output()
        .unwrap();
    println!("┋ Started Phantom!");
    println!("┋ Access dashboard at http://s.optifine.net");

    get_input(false);
    
}
fn install_deps(output: bool) {
    let _output = Command::new("py")
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("-r")
        .arg("requirements.txt")
        .current_dir(Path::new("server"))
        .output()
        .expect("failed to execute process");

    if output {
        println!("┋ Installed dependencies!");
    }
}