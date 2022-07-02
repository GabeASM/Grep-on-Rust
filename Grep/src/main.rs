/*
std::env -> 
std::fs -> to handle files (fs = file system)
.expect -> to handle errors 
*/ 
use std::env;
use std::fs;
use std::process;

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


    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}" , contents);

}

struct Config{
    query : String,
    filename : String,
}

impl Config{
    //guardando los argumentos en variables
    fn new(args : &[String])->Result<Config , &'static str>{
        if args.len() < 3 {
            return Err("not son argumentos suficientes")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query , filename})
    }

}




