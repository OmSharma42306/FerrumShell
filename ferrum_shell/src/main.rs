use std::io::stdin;

use std::io::{self,Write};
use std::process::Command;
// use std::vec;

fn main() {
    let mut a : i32 = 1;

    while a != 0 {
        print!("fsh> ");
        io::stdout().flush().unwrap();
    let mut cmd : String = String::new();

    stdin().read_line(&mut cmd).expect("Unable to Read Your Command");
    println!("You Entered Command is : {}",cmd);
    
    let cmd = cmd.trim().to_string();  /*When you type something like d and press Enter, cmd actually becomes "d\n".
                                                        That’s why comparisons like cmd == "d" sometimes won’t work. */
    if cmd == "d" {
        print!("i am here");
        a = 0;
    }

    // split input into commands and args
    let parts : Vec<&str> = cmd.split_whitespace().collect();
    let commands = parts[0];
    let args = &parts[1..];

    println!("Command : {} and args are : {:?}",commands,args);

    let status  = Command::new(commands).args(args).status();

    match status{
        Ok(s) =>{
            if !s.success(){
                 eprintln!("Command exited with status: {:?}", s.code());
            }   
        }
        Err(e) =>{
             eprintln!("Error running command: {}", e);
        }
    }



    }
    

}
