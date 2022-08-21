use regex::Regex;

fn main() {
    // mensaje a mostar en pantalla
    println!("Calculadora"); 

    let mat_regex_mult_div = Regex::new(r"(\d+)\s?([/\*])\s?(\d+)").unwrap();
    let mat_regex_add_sust = Regex::new(r"(\d+)\s?([\+\-])\s?(\d+)").unwrap();
    // debe ser mutable para recibir la expression matemática
    let mut expression: String = String::new(); 
    // unwrap se utiliza para tratar los errores
    std::io::stdin().read_line(&mut expression).unwrap(); 

    // primero validar las multiplicaciones y divisiones
    loop {
        let caps = mat_regex_mult_div.captures(&expression);

        // si ya no queda que leer salir del loop
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let sign = caps.get(2).unwrap().as_str();

        if sign == "*" {
            let result = left_value * right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        } else if sign == "/" {
            let result = left_value / right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        }
    }

    // segundo validar las sumas y restas
    loop {
        let caps = mat_regex_add_sust.captures(&expression);

        // si ya no queda que leer salir del loop
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let sign = caps.get(2).unwrap().as_str();

        if sign == "+" {
            let result = left_value + right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        } else if sign == "-" {
            let result = left_value - right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        }
    }
  
    // Mostrar resultado
    println!("Resultado: {}", expression);
}
