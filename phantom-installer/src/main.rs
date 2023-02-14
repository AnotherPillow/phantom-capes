use is_superuser::is_superuser;
use std::fs;
use regex::Regex;

fn main() {
    let optifine_regex = Regex::new(r"^(\d+\.)+(\d+) (ph|s).(optifine|lunarclientcdn)\.(.+)$").unwrap();
    if !is_superuser() {
        println!("Please run as admin");
        //wait for input before closing 
        get_input();
        return;
    }
    println!("Running as admin");
    let hosts_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    let hosts = fs::read_to_string(hosts_path).unwrap();
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
    println!("Writing new hosts file");
    fs::write(hosts_path, new_hosts).unwrap();
    println!("Done");

    get_input()


}

fn get_input() {
    let mut input = String::new();
    println!("Press any key to continue...");
    std::io::stdin().read_line(&mut input).unwrap();
}