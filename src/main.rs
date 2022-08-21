use regex::Regex;

fn operar(mat_regex: Regex, mut expression: String, _sign: &str) -> String {

    loop {
        let caps = mat_regex.captures(&expression);

        // si ya no queda que leer salir del loop
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let _sign = caps.get(2).unwrap().as_str();

        if _sign == "*" {
            let result = left_value * right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        } else if _sign == "/" {
            let result = left_value / right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        } else if _sign == "+" {
            let result = left_value + right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        } else if _sign == "-" {
            let result = left_value - right_value;
            expression = expression.replace(cap_expression, &result.to_string());
        }
    }
    return expression;
}

fn main() {
    // mensaje a mostar en pantalla
    println!("Calculadora");

    let mat_regex_mult = Regex::new(r"(\d+)\s?(\*)\s?(\d+)").unwrap();
    let mat_regex_div = Regex::new(r"(\d+)\s?(/)\s?(\d+)").unwrap();
    let mat_regex_add = Regex::new(r"(\d+)\s?(\+)\s?(\d+)").unwrap();
    let mat_regex_sust = Regex::new(r"(\d+)\s?(-)\s?(\d+)").unwrap();    
    // debe ser mutable para recibir la expression matemática
    let mut expression: String = String::new();
    // unwrap se utiliza para tratar los errores
    std::io::stdin().read_line(&mut expression).unwrap();

    expression = operar(mat_regex_mult, expression, "*");
    expression = operar(mat_regex_div, expression, "/");
    expression = operar(mat_regex_add, expression, "+");
    expression = operar(mat_regex_sust, expression, "-");


    // Mostrar resultado
    println!("Resultado: {}", expression);
}
