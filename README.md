# Curso Básico de Rust

## Creando una calculadora

En esta primera versión vamos a permitir interpretar cadenas que tengan el operador de suma `+`, resta `-`, multiplicación `*` y división `/` a través de la consola y utilizando expresiones regulares.

- Suma: `(\d+)\s?\+\s?(\d+)`
- Resta: `(\d+)\s?\-\s?(\d+)`
- Multiplicación: `(\d+)\s?\*\s?(\d+)`
- División: `(\d+)\s?/\s?(\d+)`

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
