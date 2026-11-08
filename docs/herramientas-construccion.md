# Herramientas de construcción

El proyecto necesita automatizar tres tareas: instalar las dependencias, comprobar
que los fuentes compilan y ejecutar los tests. El entorno de desarrollo es Linux (en
Windows, WSL con Ubuntu) y la versión de Rust es la 1.99.0, fijada en
`rust-toolchain.toml`.

## Gestor de dependencias

### Criterios

Por orden de prioridad:

1. **Estándar**: que la herramienta sea la oficial del lenguaje o esté normalizada.
2. **Deuda técnica**: que su formato no obligue a rehacer el fichero más adelante.
3. **Coste de instalación**, medido en el entorno del proyecto.
4. **Coste de ejecución**, medido en el entorno del proyecto.

### Herramientas consideradas

- **Cargo**:
  - **Estándar**: es el gestor de paquetes oficial de Rust.
  - **Deuda técnica**: viene con el lenguaje, y las dependencias se declaran en
    `Cargo.toml` y sus versiones exactas quedan fijadas en `Cargo.lock`, sus formatos
    estándar.
  - **Coste de instalación**: viene con la toolchain de Rust, sin nada más que
    instalar. En la imagen `rust:1.99-slim` ya está Cargo 1.99.1, pero
    `rust-toolchain.toml` fija la 1.99.0, así que el primer `cargo fetch` la descarga:
    12,1 s y 593 MB ([captura](../img/cargo-rust-slim.png)). El coste viene de la versión
    fijada, no de Cargo.
  - **Coste de ejecución**: con esa versión ya instalada, `cargo fetch` tarda 0,03 s
    ([captura](../img/cargo-rust-slim.png)).

Al buscar gestores de dependencias para Rust también aparecen estas opciones, que no se
consideran porque no son gestores de dependencias:

- [Buck2 con Reindeer](https://github.com/facebookincubator/reindeer): según su
  documentación, Reindeer genera reglas para Buck2, otra herramienta de construcción, a
  partir de las dependencias de un monorepo, y este proyecto no lo es.
- [Bazel con rules_rust (crate_universe)](https://github.com/bazelbuild/rules_rust/tree/main/crate_universe):
  según su documentación, usa Cargo para generar reglas de Bazel, otra herramienta de
  construcción, así que no sustituye a Cargo.

### Decisión: Cargo

Cargo es el gestor oficial de Rust y el único gestor de dependencias considerado. Sus
formatos son los estándar del lenguaje, y el único coste medido, 12,1 s la primera vez en
`rust:1.99-slim`, se debe a la versión que fija `rust-toolchain.toml`, no a Cargo.

## Gestor de tareas

### Criterios

Por orden de prioridad:

1. **Estándar**: que la herramienta sea la oficial del lenguaje o esté normalizada.
2. **Deuda técnica**: que su formato no obligue a rehacer el fichero más adelante.
3. **Coste de instalación**, medido en el entorno del proyecto.
4. **Coste de ejecución**, medido en el entorno del proyecto.

### Herramientas consideradas

Las medidas se hacen en `rust:1.99-slim`. La ejecución se mide con el proyecto recién
limpiado (`cargo clean`), comparando `check` con `cargo check` a secas.

- **make**:
  - **Estándar**: sí, forma parte de POSIX (IEEE Std 1003.1).
  - **Deuda técnica**: las tareas van en un `Makefile`, un formato estable.
  - **Coste de instalación**: no viene en `rust:1.99-slim`. `apt-get update` e instalar
    el paquete `make`, sin compilar nada, tarda 3,3 s y ocupa 1,8 MB
    ([captura](../img/make-rust-slim.png)).
  - **Coste de ejecución**: `make check` tarda 0,062 s, y `cargo check` 0,065 s: no
    añade un tiempo apreciable ([captura](../img/make-check-rust-slim.png)).
- **just**:
  - **Estándar**: no.
  - **Deuda técnica**: las tareas van en un `justfile`, un formato propio de la
    herramienta.
  - **Coste de instalación**: `cargo install just --locked` compila 96 crates en 61,5 s
    y deja un binario de 6,2 MB (v1.59.0) ([captura](../img/just-rust-slim.png)).
  - **Coste de ejecución**: `just check` tarda 0,082 s, y `cargo check` 0,079 s: no
    añade un tiempo apreciable ([captura](../img/just-check-rust-slim.png)).
- **cargo-make**:
  - **Estándar**: no.
  - **Deuda técnica**: las tareas van en un `Makefile.toml`, un formato propio de la
    herramienta.
  - **Coste de instalación**: `cargo install cargo-make --locked` compila 182 crates en
    74,2 s y deja un binario de 9,7 MB más un segundo ejecutable, `makers` (v0.37.24)
    ([captura](../img/cargo-make-rust-slim.png)).
  - **Coste de ejecución**: `cargo make check` tarda 0,210 s, y `cargo check` 0,069 s:
    añade unos 0,14 s en cada ejecución ([captura](../img/cargo-make-check-rust-slim.png)).
- **cargo-xtask**:
  - **Estándar**: no.
  - **Coste de instalación**: no hay nada que instalar, pero el crate se compila la
    primera vez: 0,15 s y un binario de 4,7 MB ([captura](../img/cargo-xtask-rust-slim.png)).
  - **Coste de ejecución**: en el proyecto con el crate `xtask` añadido, `cargo check`
    tarda 0,076 s.
    Tras un `cargo clean`, que también borra el binario de `xtask`, `cargo xtask check`
    tarda 0,193 s, porque vuelve a compilarlo: unos 0,12 s más, que es lo que costaría
    en cada contenedor nuevo. Con `xtask` ya compilado tarda 0,050 s, sin tiempo añadido
    apreciable ([captura](../img/cargo-xtask-check-rust-slim.png)).
- **mise**:
  - **Estándar**: no.
  - **Deuda técnica**: las tareas van en un `mise.toml`, un formato propio de la
    herramienta.
  - **Coste de instalación**: `cargo install mise --locked` falla porque le falta un
    compilador de C++ ([captura](../img/mise-sin-paquetes-rust-slim.png)). Instalando
    antes `g++`, `cmake`, `pkg-config` y `libssl-dev`, todo el proceso tarda 10 min 45 s,
    compila 995 crates y deja un binario de 275 MB (v2026.11.2)
    ([captura](../img/mise-rust-slim.png)).
  - **Coste de ejecución**: `mise run check` tarda 0,095 s, y `cargo check` 0,067 s:
    añade unos 0,03 s en cada ejecución ([captura](../img/mise-rust-slim.png)).

### Decisión: make

Por orden de criterios, make es la única de las cinco que es un estándar. En deuda
técnica, su formato es estable, mientras que just, cargo-make y mise usan uno propio, y
cargo-xtask convierte las tareas en código Rust que hay que mantener y obliga a un
*workspace*. En coste de instalación es la que menos cuesta de las que hay que instalar:
3,3 s y 1,8 MB, sin compilar nada, frente a los 61,5 s de just (96 crates), los 74,2 s de
cargo-make (182 crates) y los 10 min 45 s de mise (995 crates, 275 MB y cuatro paquetes
del sistema). Al ejecutar `check` en un contenedor recién creado, make y just no añaden
tiempo apreciable a `cargo check`, mientras que cargo-make añade unos 0,14 s, cargo-xtask
unos 0,12 s y mise unos 0,03 s, que se pagarían en cada ejecución de la CI.

## Tareas

- `make install` ejecuta `cargo fetch`, que descarga las dependencias.
- `make check` ejecuta `cargo check`, que comprueba que los fuentes compilan, sin
  generar el binario ni ejecutar los tests.
- `make test` ejecuta `cargo test`, que ejecuta los tests.
