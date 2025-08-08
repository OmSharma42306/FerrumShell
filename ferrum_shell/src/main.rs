use std::io::stdin;

use std::io::{self,Write};
use std::process::Command;
// use std::vec;

fn main() {
    

    loop {
        print!("fsh> ");
        io::stdout().flush().unwrap();
    let mut cmd : String = String::new();

    stdin().read_line(&mut cmd).expect("Unable to Read Your Command");
    println!("You Entered Command is : {}",cmd);
    
    let cmd = cmd.trim().to_string();  /*When you type something like d and press Enter, cmd actually becomes "d\n".
                                                        That’s why comparisons like cmd == "d" sometimes won’t work. */
    
    if cmd.is_empty(){
        continue;
    }
    // split input into commands and args
    let parts : Vec<&str> = cmd.split_whitespace().collect();
    let commands = parts[0];
    let args = &parts[1..];

      /*
    ********************* Handling Builtin Commands Stuff *******************************************
    When you type ls in a shell, the shell spawns a new process that runs /bin/ls.
    But when you type cd /some/path, you can’t spawn a new process to change the directory — because processes have their own working directory and the change wouldn’t affect the parent shell.

    So, built-in commands are those handled by the shell itself without calling an external process.
     */
    if commands == "cd"{
        if let Some(path) = args.get(0){
            if let Err(e) = std::env::set_current_dir(path){
                eprintln!("cd : {}: {}",path,e);
            }
        }else{
            eprintln!("cd : missing arguement!");
        }
        continue;
    }else if commands == "exit"{
        std::process::exit(0);
    }

    println!("Command : {} and args are : {:?}",commands,args);

    let status  = Command::new(commands).args(args).status();  // here the status that actullay spawn the process and after completing the process it gets back the result of command.

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
