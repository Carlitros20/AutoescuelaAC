# Herramientas de test

## Criterios de búsqueda

Se buscan aserciones, marcos de test y runners en la documentación oficial de Rust y en crates.io. Solo se tienen en cuenta los que hayan publicado una versión en el último año. Como el guion pide buscar bibliotecas que digan ofrecer *specs* o estilo BDD, también se buscan en crates.io los términos «bdd», «spec» y «rspec».

## Criterios de aceptación

Por orden de prioridad:

1. **No escribe en los fuentes**: funciona con el proyecto montado en solo lectura, configurando dónde escribe.
2. **Estándar**: viene con el lenguaje.
3. **Coste de instalación**: tiempo medido en `rust:1.99.0-slim`.
4. **Coste de ejecución**: tiempo de ejecutar los tests, medido en `rust:1.99.0-slim`.

## Herramientas consideradas

### Runner

**`cargo test`**

- **No escribe en los fuentes**: con el proyecto montado en solo lectura falla si no se configura dónde escribir, y pasa con `CARGO_TARGET_DIR` fuera de los fuentes ([captura](../img/cargo-test-solo-lectura.png)).
- **Estándar**: viene con Cargo.
- **Coste de instalación**: ninguno, viene en la imagen.
- **Coste de ejecución**: 0,183 s la primera vez, porque compila, y 0,032 s y 0,035 s las siguientes ([captura](../img/cargo-test-tiempo.png)). Todavía no hay tests, así que esto solo mide compilar el proyecto y arrancar el runner.

**`cargo-nextest`**

- **No escribe en los fuentes**: pasa con el proyecto montado en solo lectura y `CARGO_TARGET_DIR` fuera de los fuentes ([captura](../img/cargo-nextest.png)).
- **Estándar**: no, hay que instalarlo aparte.
- **Coste de instalación**: 1,514 s con el binario, que es lo que recomienda su documentación, y 3,085 s más para instalar `curl`, que no viene en la imagen.
- **Coste de ejecución**: con un solo test de prueba, 0,259 s la primera vez, y 0,074 s y 0,064 s las siguientes. `cargo test` tarda con ese mismo test 0,176 s, 0,030 s y 0,030 s.

### Aserciones y marcos de test

`assert!` y `assert_eq!`, `#[test]` con libtest, `assert2`, `assertables`, `googletest`, `rstest` y `test-case` pasan los tests con el proyecto montado en solo lectura y `CARGO_TARGET_DIR` fuera de los fuentes ([captura](../img/bibliotecas-solo-lectura.png)). `cucumber` se ha compilado así, pero no se ha ejecutado, porque necesita su propio `main`. Este criterio no descarta ninguna.

- **`assert!`, `assert_eq!` y `assert_ne!`**: aserciones estándar, vienen con el lenguaje.
- **`assert2`**: aserciones con mensajes más detallados y `check!`, que no para el test en el primer fallo. No es estándar.
- **`assertables`**: macros de aserción específicas. No es estándar.
- **`googletest`**: matchers que se combinan, con `assert_that!`, que para el test al fallar, y `expect_that!`, que lo marca como fallido pero lo deja seguir. Trae su propio atributo, `#[gtest]`. No es estándar.
- **`#[test]` con libtest**: marco estándar, viene con el lenguaje.
- **`rstest`**: añade fixtures y tablas sobre libtest. No es estándar.
- **`test-case`**: añade tablas sobre libtest. No es estándar.
- **`cucumber`**: BDD con ficheros de escenarios, sustituye a libtest. No es estándar.

`rstest-bdd` y `spectacular`, las que salen al buscar specs y BDD, también pasan sus tests con el proyecto montado en solo lectura ([captura](../img/specs-solo-lectura.png)).

- **`rstest-bdd`**: escenarios BDD en ficheros `.feature`, sobre `rstest`. No es estándar.
- **`spectacular`**: tests al estilo RSpec, con `describe` e `it`. No es estándar.

## Decisión

- **Aserciones**: `assert!`, `assert_eq!` y `assert_ne!`.
- **Marco de test**: `#[test]` con libtest.
- **Runner**: `cargo test`.

Todas las opciones pasan el primer criterio, así que decide el segundo, que va antes que el coste: son las estándar. El coste no cambia la decisión: `cargo test` no cuesta nada de instalar, frente a los 4,6 s de `cargo-nextest` con `curl`, y con un solo test tarda menos. Según su documentación, `cargo-nextest` gana en proyectos grandes, con muchos binarios de test, que no es el caso. Hay que configurar `CARGO_TARGET_DIR` para no escribir en los fuentes.

## Principios F.I.R.S.T.

- **Rápidos**: no usan red, disco ni esperas.
- **Independientes**: cada test crea su propia agenda.
- **Repetibles**: usan horas fijas en lugar del reloj del sistema, así que siempre dan el mismo resultado.
- **Autoevaluables**: cada uno comprueba el resultado con `assert_eq!`, sin que nadie tenga que leer la salida.
- **Oportunos**: se han escrito junto al código de #13.
