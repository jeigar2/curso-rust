fn main() {
    // mensaje a mostar en pantalla
    println!("Ingrese su nombre:"); 
    // debe ser mutable para recibir el nombre
    let mut nombre: String = String::new(); 
    // unwrap se utiliza para tratar los errores
    std::io::stdin().read_line(&mut nombre).unwrap(); 
    // formatea el texto para eliminar saltos de línea y espacios en blanco.
    nombre = nombre.trim().to_string(); 

    println!("Que Dios te bendiga {}", nombre);

    // mensaje a mostar en pantalla
    println!("Ingrese su edad:"); 
    // debe ser mutable para recibir el texto con la edad
    let mut edad: String = String::new(); 
    std::io::stdin().read_line(&mut edad).unwrap();
    // formatea el texto para eliminar saltos de línea y espacios en blanco 
    // y transforma a entero
    let int_edad: u8 = edad.trim().parse().unwrap(); 

    // {} es un placeholder 
    // cada uno de ello se reemplaza por el valor de las varibles recibidas 
    // en el resto de argumentos de la función (nombre, int_edad)
    println!("{} tienes {} años", nombre, int_edad); 
}
