/*
std::env -> 
std::fs -> to handle files (fs = file system)
.expect -> to handle errors 
*/ 
use std::env;
use std::process;
use Grep::Config;

fn main(){
    
    //leemos los 2 argumentos 
    let args : Vec<String> = env::args().collect();   
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("{}" , err);
        process::exit(1);
    });

    println!("Searching for {}" , config.query);
    println!("in file {}" , config.filename);

    if let Err(e) = Grep::run(config){
        println!("Error en la aplicacion{}" , e);
        process::exit(1);
    }
}






