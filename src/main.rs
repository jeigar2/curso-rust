use rand;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

const LIMITE_PELICULAS: u8 =  175;

fn numero_aleatorio_hasta_limite_u8(limite: u8) -> u8 {
    let tuple = rand::random::<u8>();
    let mut resultado = tuple%LIMITE_PELICULAS;
    if resultado == 0 {
        resultado = numero_aleatorio_hasta_limite_u8(limite);
    }
    return resultado;
}

fn cargar_pelicula_aleatoria () -> String {
    let mut buff_reader = BufReader::new(File::open("pelis.txt").unwrap());
    let mut pelicula: String  = String::new();
    let numero_aleatorio =  numero_aleatorio_hasta_limite_u8(LIMITE_PELICULAS);
    // println!("numero aleatorio: {}", numero_aleatorio );
    for num in 1..LIMITE_PELICULAS {
        let mut buff = String::new();
        buff_reader.read_line(&mut buff).unwrap();
        if num == numero_aleatorio {
            pelicula = buff.clone();
            // println!("{} Pelicula: {}", num, buff.clone() );
        }
    }
    return pelicula.trim().to_string();
}

fn main() {
    let mut descubre_pelicula: HashMap<usize,char>=HashMap::new();
    let pelicula_aleatoria: String = cargar_pelicula_aleatoria();
//    println!("Pelicula aleatoria: {}",pelicula_aleatoria);
    let mut index: usize = 0;
    let mut total_letras: usize = 0;
    let mut total_letras_descubiertas: usize = 0;
    let mut intentos:Vec<char> = vec![];
    let mut fallos: u8 = 0;
    for c in pelicula_aleatoria.chars() {
        if c == ' '{
            print!(" ");
            descubre_pelicula.insert(index, c);
        } else {
            print!("-");
            descubre_pelicula.insert(index, '-');
            total_letras += 1;
        }
        index += 1;
    }
    println!();

    // Game Loop
    loop{
        println!("- Fallos: {}", fallos);
        println!("- Total letras: {}", total_letras);
        println!("- Total letras descubiertas: {}", total_letras_descubiertas);
        for (i,ch) in intentos.iter().enumerate() {
            print!("{}", ch);
        }
        println!();
        println!("Introduzca una letra para adivinar la pelicula:");
        let mut selection = String::new();
        std::io::stdin().read_line(&mut selection).unwrap();
        for letra in  selection.trim().to_ascii_uppercase().to_string().chars(){
            if letra == '0' {
                println!("El usuario ha forzado la salida");
                return;
            } else {
                let mut index = 0;
                if !intentos.contains(&letra) {
                    intentos.insert(index, letra);
                    if !pelicula_aleatoria.contains(letra) {
                        println!("La letra {} no existe", letra);
                        fallos += 1;
                        // Pintar descubre pelicula como estaba antes de este turno
                        for c in pelicula_aleatoria.chars() {
                            print!("{}", descubre_pelicula.get(&index).unwrap()); 
                            index += 1;
                        }
                        println!();
                    } else {
                        //actulizar descubre pelicula con la letra averiguada
                        for c in pelicula_aleatoria.chars() {
                            if c == letra {
                                print!("{}",letra);
                                descubre_pelicula.insert(index, letra);
                                total_letras_descubiertas += 1;
                            } else {
                                print!("{}", descubre_pelicula.get(&index).unwrap()); 
                            }
                            index += 1;
                        }
                        println!();
                    }
                } else {
                    println!("La letra {} ya se ha puesto anteriormente", letra);
                }
            }
        }
        if total_letras_descubiertas == total_letras {
            println!("¡Enhorabuena ha descubierto la película!");
            return;
        }
        if fallos > 7 {
            println!("Se acabo el juego, ha tenido {} fallos", fallos);
            println!("La película era {}", pelicula_aleatoria);
            return;
        }
    }
}
