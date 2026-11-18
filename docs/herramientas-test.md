# Herramientas de test

## Criterios de búsqueda

Se buscan aserciones, marcos de test y runners en la documentación oficial de Rust y en crates.io. Solo se tienen en cuenta los que hayan publicado una versión en el último año. Como el guion pide buscar bibliotecas que digan ofrecer *specs* o estilo BDD, también se buscan en crates.io los términos «bdd», «spec» y «rspec».

## Criterios de aceptación

Por orden de prioridad:

1. **No escribe en los fuentes**: funciona con el proyecto montado en solo lectura, configurando dónde escribe.
2. **Coste de ejecución**: tiempo de ejecutar los tests, medido en `rust:1.99.0-slim`.

## Herramientas consideradas

### Runner

**`cargo test`**

- **No escribe en los fuentes**: con el proyecto montado en solo lectura falla si no se configura dónde escribir, y pasa con `CARGO_TARGET_DIR` fuera de los fuentes ([captura](../img/cargo-test-solo-lectura.png)).
- **Coste de ejecución**: 0,183 s la primera vez, porque compila, y 0,032 s y 0,035 s las siguientes ([captura](../img/cargo-test-tiempo.png)). Todavía no hay tests, así que esto solo mide compilar el proyecto y arrancar el runner.

**`cargo-nextest`**

- **No escribe en los fuentes**: pasa con el proyecto montado en solo lectura y `CARGO_TARGET_DIR` fuera de los fuentes ([captura](../img/cargo-nextest.png)).
- **Coste de ejecución**: con un solo test de prueba, 0,259 s la primera vez, y 0,074 s y 0,064 s las siguientes. `cargo test` tarda con ese mismo test 0,176 s, 0,030 s y 0,030 s.

### Aserciones

`assert!` y `assert_eq!`, `assert2`, `assertables` y `googletest` pasan los tests con el proyecto montado en solo lectura y `CARGO_TARGET_DIR` fuera de los fuentes ([captura](../img/bibliotecas-solo-lectura.png)). Este criterio no descarta ninguna. El tiempo de `cargo test` se ha medido con el mismo test en cada una; la primera vez incluye compilar ([captura](../img/bibliotecas-coste.png)).

- **`assert!`, `assert_eq!` y `assert_ne!`**: vienen con el lenguaje. 0,124 s la primera vez y 0,026 s las siguientes.
- **`debug_assert!`, `debug_assert_eq!` y `debug_assert_ne!`**: las mismas aserciones, pero se desactivan al compilar en release, así que no sirven para tests.
- **`assert2`**: aserciones con mensajes más detallados y `check!`, que no para el test en el primer fallo. 2,688 s la primera vez, y 0,033 s y 0,034 s las siguientes.
- **`assertables`**: macros de aserción específicas. 0,265 s la primera vez, y 0,029 s las siguientes.
- **`googletest`**: matchers que se combinan, con `assert_that!`, que para el test al fallar, y `expect_that!`, que lo marca como fallido pero lo deja seguir. Trae su propio atributo, `#[gtest]`. 3,341 s la primera vez, y 0,031 s y 0,034 s las siguientes.

### Marcos de test

`#[test]` con libtest, `rstest` y `test-case` pasan los tests con el proyecto montado en solo lectura y `CARGO_TARGET_DIR` fuera de los fuentes ([captura](../img/bibliotecas-solo-lectura.png)). `cucumber` se ha compilado así, pero no se ha ejecutado, porque necesita su propio `main`. Este criterio no descarta ninguno. El tiempo de `cargo test` se ha medido igual que en las aserciones ([captura](../img/bibliotecas-coste.png)).

- **`#[test]` con libtest**: viene con el lenguaje. 0,124 s la primera vez y 0,026 s las siguientes.
- **`rstest`**: marco de test que funciona sobre libtest y añade fixtures y tablas de casos. 3,923 s la primera vez, y 0,042 s y 0,039 s las siguientes.
- **`test-case`**: añade tablas sobre libtest. 2,397 s la primera vez, y 0,029 s las siguientes.
- **`cucumber`**: BDD con ficheros de escenarios, sustituye a libtest. 11,667 s la primera vez, y 0,085 s y 0,389 s las siguientes.

`rstest-bdd` y `spectacular`, los marcos que salen al buscar specs y BDD, también pasan sus tests con el proyecto montado en solo lectura ([captura](../img/specs-solo-lectura.png)), y se han medido igual que los demás ([captura](../img/bibliotecas-coste.png)).

- **`rstest-bdd`**: escenarios BDD en ficheros `.feature`, sobre `rstest`. 12,225 s la primera vez, y 0,089 s y 0,087 s las siguientes.
- **`spectacular`**: tests al estilo RSpec, con `describe` e `it`. 2,104 s la primera vez, y 0,030 s y 0,028 s las siguientes.

## Decisión

- **Aserciones**: `assert!`, `assert_eq!` y `assert_ne!`.
- **Marco de test**: `#[test]` con libtest.
- **Runner**: `cargo test`.

Todas las opciones pasan el primer criterio, así que decide el coste de ejecución. Las aserciones y el marco del lenguaje son los que menos tardan: 0,124 s la primera vez, frente a 0,265 s de `assertables` y 2,104 s de `spectacular`, las siguientes más rápidas de cada tipo, y en las demás ejecuciones ninguna tarda menos. Con el mismo test, `cargo test` tarda menos que `cargo-nextest`. Según su documentación, `cargo-nextest` gana en proyectos grandes, con muchos binarios de test, que no es el caso. Hay que configurar `CARGO_TARGET_DIR` para no escribir en los fuentes.
