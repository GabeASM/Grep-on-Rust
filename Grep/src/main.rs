/*
std::env -> 
std::fs -> to handle files (fs = file system)
.expect -> to handle errors 
*/ 
use std::env;
use std::fs;
use std::process;
use std::error::Error;
use Grep::Config;

fn main(){
    
    //leemos los 2 argumentos 
    let args : Vec<String> = env::args().collect();
    println!("{:?}" , args); 

   
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}" , err);
        process::exit(1);
    });

    println!("Searching for {}" , config.query);
    println!("in file {}" , config.filename);

    if let Err(e) = Grep::run(config){
        println!("Error en la aplicacion {}" , e);
        process::exit(1);
    }
}






