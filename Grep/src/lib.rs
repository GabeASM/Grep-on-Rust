/*
std::env -> 
std::fs -> to handle files (fs = file system)
.expect -> to handle errors 
*/ 
use std::fs;
use std::error::Error;

pub fn run(config : Config) -> Result<() , Box <dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}" , contents);
    Ok(())
}

pub struct Config{
    pub query : String,
    pub filename : String,
}

impl Config{
    //guardando los argumentos en variables
   pub fn new(args : &[String])->Result<Config , &'static str>{
        if args.len() < 3 {
            return Err("not son argumentos suficientes")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query , filename})
    }

}




