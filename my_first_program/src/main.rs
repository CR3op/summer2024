use std::io::{self, Read, Write};
use std::process::{Command};

fn executing_os_commands_linux(command_full: &str){

    let parts: Vec<&str> = command_full.split_whitespace().collect();
    let actual_command = parts[0];
    let arg1 = parts[1];

    let output = Command::new(actual_command)
        .arg(arg1)
        .output()
        .unwrap();
        //.expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));

}


fn accept_linux_command_from_user()-> String{
    let mut buffer= String::new();
    print!("Enter a linux command: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}


fn main() {
    // reading_from_console();
    // reading_from_file();

    let full_command = accept_linux_command_from_user();
    executing_os_commands_linux(&full_command);

    // executing_os_commands_linux("mkdir hello");
    // executing_os_commands_linux("echo hello");
    // executing_os_commands_linux("ls -la");
}
