# Herramientas de test

## Criterios de búsqueda

Se buscan aserciones, marcos de test y runners en la documentación oficial de Rust y en crates.io. Solo se tienen en cuenta los que hayan publicado una versión en el último año.

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
