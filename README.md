# Curso Básico de Rust

## Creando una calculadora

En esta segunda versión vamos a permitir interpretar cadenas que tengan el operador de suma `+`, resta `-`, multiplicación `*` y división `/` a través de la consola y utilizando expresiones regulares.

- Suma: `(\d+)\s?\+\s?(\d+)`
- Resta: `(\d+)\s?\-\s?(\d+)`
- Multiplicación: `(\d+)\s?\*\s?(\d+)`
- División: `(\d+)\s?/\s?(\d+)`

Pero esto lo haremos con una función para eliminar el código repetido

### Utilización de dependencias en Rust

El respositorio de dependencias está en [https://crates.io/](https://crates.io/)
Se agregan las dependencias en el archivo `Cargo.toml` del proyecto
Para importarla se hace con `use` en el fichero de codigo que lo requiera

Ejemplo

```rs
use regex::Regex;

fn main() {
    // ...
}
```

## Utilizando Regex

Se debe crear un `Regex::new()` y dentro agregar la cadena entre comillas de la expresión regular `r"(\d+)\s?([\+\-])\s?(\d+)"` precedida del caracter `r`, posteriormente agregamos el tratamiento de errores con `unwrap()`

```rs
let re_add = Regex::new(r"(\d+)\s?([\+\-])\s?(\d+)").unwrap();
```

## Conversiones de datos en Rust

### String a entero

Hay varias formas de declarar una variable del tipo `String`:

```rs
let nombre = "123".to_string();
let nombre: String = "123".to_string();
let nombre = String::from("123");
let mut nombre: String = String::new();
nombre = "123".to_string();
```

Conviértelo a número entero con `parse()`, pero teniendo en cuenta de borrar los espacios en blanco con `trim()` para evitar inconvenientes y capturar los errores con `unwrap()`.

```rs
let number: i32 = nombre.trim().parse().unwrap();
println!("{}", number);
```

### String vs. &str

El tipo de dato `String` permite manipular una cadena de texto
El tipo de dato `&str` contiene la referencia a un `String`, pero solo contiene su valor.
Cuando le asignas a una variable un valor con dobles comillas `""`, las mismas crean un `&str`, por esto utilizamos `to_string()` para que retorne el tipo de dato en formato `String` y poder manipularlo. Y por este mismo motivo necesitas un `String` para hacer un `.trim().parse()` y convertir la variable al tipo entero.

Para convertir un `String` a `&str`, solo agrega el `&` (Ampersand) delante del nombre de la variable o con `as_str()`.

```rs
let nombre: String = "123".to_string();
let nombre_plano: &str = &nombre;

let nombre: String = "123".to_string();
let nombre_plano: &str = nombre.as_str();
```

## Declarando y llamando funciones en Rust

El concepto de Funciones es un pedazo de código que puedes llamar y utilizar cuando necesites.

La declaración de funciones en Rust se realiza con la palabra reservada `fn`. Ya la has visto antes en `fn main() {}` que es la función principal de tu aplicación y que se ejecuta siempre al iniciar la misma.
Declara tu propia función de la siguiente manera:

```rs
fn main() {
    sumar_numeros(10, 10);          // 20
}

fn sumar_numeros(num1: i32, num2: i32) {
    let sum: i32 = num1 + num2;
    println!("{}", sum);
}
```

Las funciones reciben argumentos, puedes especificar el tipo de los mismos a lo igual que con una variable, para luego realizar una determinada lógica como la suma de dos números.

### Retorno de valores de una función en Rust

Las funciones pueden o no retornar un valor. Para hacerlo, indica el tipo de retorno de la siguiente manera:

```rs
fn main() {
    let sum = sumar_numeros(10, 10);
    println!("{}", sum);                // 20
}

fn sumar_numeros(num1: i32, num2: i32) -> i32 {
    let sum: i32 = num1 + num2;
    return sum;
}
```

Indica el tipo de retorno luego del `->` y retorna una variable con la palabra reservada `return` para volver al flujo principal de tu código y utilizar ese valor.
