# Curso Básico de Rust

## Recibiendo datos del usuario

Permitir la interacción del usuario con la aplicación a través de la interfaz de línea de comandos.

### Inputs en Rust

- Para recibir datos del usuario, se hace uso de algunas librerías internas de Rust que acceden al sistema operativo y permiten que el usuario ingrese datos por consola.

- `std::io::stdin().read_line(&mut nombre).unwrap();`
  - **std** es una librería de Rust para acceder al sistema operativo.
  - **io** significa inputs/outputs,
  - **stdin()** permite traer información
  - **read_line()** indica que esa información será recibida por consola.
  - **&mut nombre** es la variable donde guarda el dato ingresado por el usuario (Utiliza mut en las variables para indicar que la misma cambiará de valor).
  - **unwrap()**, ayuda con el manejo de errores.

### Cambiando el tipo de dato en Rust

- Por defecto, los datos que ingresa el usuario por consola son del tipo String.
- Si necesitas números enteros hay que convertir el tipo de dato

- `let edad_int: u8 = edad.trim().parse().unwrap();`
  - **parse()** hace la conversión al tipo de dato definido en la variable que se asigna

## Reto 2

Pedir el nombre del país de nacimiento, y escribir un mensaje de bendición para la persona y el pais.

salida esperada:

```md
Ingrese su nombre:
_Alicia_
Que Dios te bendiga _Alicia_
Ingrese su edad:
_34_
En que país nació:
_Australia_
Que Dios te bendiga _Alicia_ y también a tu país _Australia_
_Alicia_ tienes _34_ años
```
