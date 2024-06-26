use std::path::Path;
use std::process::Command;
use std::io::{stdout, Write, stdin};
use std::env;


fn main(){
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace character
        // is interpreted as atgs to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args =parts;

        // let command = input.trim();
        match command{
            "cd" =>{
                //default to '/' as new directory if one was net provided
                let new_dir = args.peekable().peek().map_or("/",|x|*x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root){
                    eprintln!("{}",e);
                }
            },
            "exit" => return,
            command => {
                let  child = Command::new(command)
                    .args(args)
                    .spawn();

                match child{
                    Ok(mut child) => {child.wait().unwrap();},
                    Err(e)=> eprintln!("{}",e),
                }
            }
        }
    }
}