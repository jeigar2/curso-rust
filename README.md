# Curso Básico de Rust

## Condicionales

Rust permite desarrollar programas con lógica y condicionantes para realizar una u otra acción dependiendo el valor de las variables o el estado de ejecución del programa.

### if en Rust

Para escribir una condición en Rust hay que escribir `if`, y si quieres tratar la condición opuesta sería `else`

En Rust los paréntesis no deben ponerse en la condición del if.

La condición utiliza operadores lógicos, igualdad de datos `==` o verificar si un número es mayor, menor o igual con `>, <, >= y <=`. Puedes tener varias condiciones con `&&` para un **y** lógico o un `||` para un **o** lógico.

## Reto 3

Pedir el la pastilla roja o azul de Matrix, en función del color seleccionado mostrar un mensaje u otro, si pone cualquier otro valor, será incorrecto.

salida esperada si escribe Azul:

```log
¡Hola! Neo:
Debes elegir si tomar:
 - la pildora "roja" y seguir en Matrix
 - la pildora "azul" y conocer el mundo real:
AzUl
Ha escogido la pildora AzUl
Estamos en un mundo nuevo y descubrirás que has estado controlado por Matrix y las máquinas, ahora que conoces la verdad, eres parte de la resistencia.
```

salida esperada si escribe Roja:

```log
¡Hola! Neo:
Debes elegir si tomar:
 - la pildora "roja" y seguir en Matrix
 - la pildora "azul" y conocer el mundo real:
ROJA
Ha escogido la pildora ROJA
Todo esto que ha pasado, lo olvidarás y vivirás tu misma vida dentro de Matrix
```

salida esperada si escribes algo diferente a Roja o Azul, ejemplo verde

```log
¡Hola! Neo:
Debes elegir si tomar:
 - la pildora "roja" y seguir en Matrix
 - la pildora "azul" y conocer el mundo real:
verde
Ha escogido la pildora verde
solo puedes escoger el color de la pastilla "roja" o "azul" este mesanje se autodestruirá en 5 segundos
```
