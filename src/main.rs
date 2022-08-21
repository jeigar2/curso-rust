fn main() {
    // mensaje a mostar en pantalla
    println!("¡Hola! Neo:\nDebes elegir si tomar:\n - la pildora \"roja\" y seguir en Matrix\n - la pildora \"azul\" y conocer el mundo real:"); 
    // debe ser mutable para recibir el color
    let mut color: String = String::new(); 
    // unwrap se utiliza para tratar los errores
    std::io::stdin().read_line(&mut color).unwrap(); 
    // formatea el texto para eliminar saltos de línea y espacios en blanco y transforma a minúsculas
    color = color.trim().to_string(); 

    println!("Ha escogido la pildora {}", color);

    if color.to_lowercase() == "azul" {
        println!("Estamos en un mundo nuevo y descubrirás que has estado controlado por Matrix y las máquinas, ahora que conoces la verdad, eres parte de la resistencia.");
    } else if color.to_lowercase() == "roja" {
        println!("Todo esto que ha pasado, lo olvidarás y vivirás tu misma vida dentro de Matrix");
    } else {
        println!("solo puedes escoger el color de la pastilla \"roja\" o \"azul\" este mesanje se autodestruirá en 5 segundos");
    }
}
