/**
 * 
 * Para probar el minigrep usamos "Cargo run palabraparabuscar textodondebuscar"
 * por ejemplo: Si queremos buscar las coincidencias de frog en el texto poem.txt debemos escrbir (en la consola)
 * Cargo run frog poem.txt 
 * 
 * Para que el resultado lo obtengamos en un otro txt aparte debemos agregar a la sentencia ">nombrearchivo.txt"
 * Cargo run frog poem.txt >output.txt
 * 
 * 
 * El proyecto fue hecho por Gabriel Aillapan y Daniel Ruiz
 *fecha de ultima modificacion:02/07/2022 
 */


/*
modulos:
std -> biblioteca estandar que contiene un set de metodos (metodos para tipos primitivos) y modulos
std::env -> modulos que contienen funciones para las variables de entorno, argumentos y otros.
std::fs -> contiene elementos basicos para manipular el contenido de los achivios del sistema (fs = filesystem)
Grep::Config -> importamos lo de lib.rs
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
    println!("in file : \n {}" , config.filename);

    if let Err(e) = Grep::run(config){
        println!("Error en la aplicacion{}" , e);
        process::exit(1);
    }
}






