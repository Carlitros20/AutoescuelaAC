# Herramientas de construcción

El proyecto necesita automatizar tres tareas: instalar las dependencias, comprobar
que los fuentes compilan y ejecutar los tests. El entorno de desarrollo es Linux (en
Windows, WSL con Ubuntu) y la versión de Rust es la 1.99.0, fijada en
`rust-toolchain.toml`.

## Gestor de dependencias

### Criterios

Por orden de prioridad:

1. **Estándar**: que la herramienta sea la oficial del lenguaje o esté normalizada.
2. **Deuda técnica**: que esté mantenida y que su formato no obligue a rehacer el
   fichero más adelante.
3. **Coste de instalación y de ejecución**, medido en el entorno del proyecto.

### Herramientas consideradas

- **Cargo**:
  - **Estándar**: es el gestor de paquetes oficial de Rust.
  - **Deuda técnica**: viene con el lenguaje, y las dependencias se declaran en
    `Cargo.toml` y sus versiones exactas quedan fijadas en `Cargo.lock`, sus formatos
    estándar.
  - **Coste de instalación**: viene con la toolchain de Rust, sin nada más que
    instalar. En la imagen `rust:1.99-slim` ya está Cargo 1.99.1, pero
    `rust-toolchain.toml` fija la 1.99.0, así que el primer `cargo fetch` la descarga:
    12,1 s y 593 MB. El coste viene de la versión fijada, no de Cargo.
  - **Coste de ejecución**: con esa versión ya instalada, `cargo fetch` tarda 0,03 s
    ([captura](../img/cargo-rust-slim.png)).

Al buscar gestores de dependencias para Rust también aparecen estas opciones, que no se
consideran porque no son gestores de dependencias:

- [Buck2 con Reindeer](https://github.com/facebookincubator/reindeer): según su
  documentación, Reindeer genera reglas para Buck2, otra herramienta de construcción,
  y está pensado para gestionar dependencias en un monorepo, y este proyecto no lo es.
- [Bazel con rules_rust (crate_universe)](https://github.com/bazelbuild/rules_rust/tree/main/crate_universe):
  según su documentación, usa Cargo para generar reglas de Bazel, otra herramienta de
  construcción, así que no sustituye a Cargo.

### Decisión: Cargo

Es el gestor estándar de Rust, y las alternativas medidas no lo sustituyen: las dos
siguen necesitando Cargo y además añaden otro sistema de construcción. Buck2 con
Reindeer descarga las dependencias sin Cargo, pero lo necesita para generar las reglas
de Buck2, y suma 164 MB de binarios. Bazel con rules_rust resuelve las dependencias con
Cargo, y la primera descarga tarda 57,3 s, deja 1,4 GB en la caché y trae una versión
de Cargo distinta de la fijada, así que habría que fijarla en otro sitio más. El único
coste medido de Cargo, 12,1 s la primera vez en `rust:1.99-slim`, se debe a la versión
que fija `rust-toolchain.toml`, no a Cargo.

## Gestor de tareas

### Criterios

Por orden de prioridad:

1. **Estándar**: que la herramienta sea la oficial del lenguaje o esté normalizada.
2. **Deuda técnica**: que esté mantenida y que su formato no obligue a rehacer el
   fichero más adelante.
3. **Coste de instalación y de ejecución**, medido en el entorno del proyecto.

### Herramientas consideradas

Las medidas se hacen en `rust:1.99-slim`. La ejecución se mide con el proyecto recién
limpiado (`cargo clean`), comparando `check` con `cargo check` a secas.

- **make**:
  - **Estándar**: sí, forma parte de POSIX (IEEE Std 1003.1).
  - **Deuda técnica**: las tareas van en un `Makefile`, un formato estable.
  - **Coste de instalación**: no viene en la imagen. `apt-get update` e instalar el
    paquete `make`, sin compilar nada, tarda 3,3 s y ocupa 1,8 MB
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
  - **Estándar**: no, es una convención: un crate del propio proyecto al que se llama
    con un alias de Cargo.
  - **Deuda técnica**: las tareas son un programa en Rust (19 líneas para las tres
    tareas), con su `Cargo.toml` y el alias, y obliga a convertir el proyecto en un
    *workspace*.
  - **Coste de instalación**: no hay nada que instalar, pero el crate se compila la
    primera vez: 0,15 s y un binario de 4,7 MB ([captura](../img/cargo-xtask-rust-slim.png)).
  - **Coste de ejecución**: con un prototipo equivalente, `cargo check` tarda 0,076 s.
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
    compila 995 crates y deja un binario de 275 MB (v2026.11.2).
  - **Coste de ejecución**: `mise run check` tarda 0,095 s, y `cargo check` 0,067 s:
    añade unos 0,03 s en cada ejecución ([captura](../img/mise-rust-slim.png)).

### Decisión: make

make es la única de las cinco que es un estándar, y su formato no ha cambiado en
décadas. De las que hay que instalar, es la que menos cuesta en `rust:1.99-slim`:
3,3 s y 1,8 MB, sin compilar nada, frente a los 61,5 s de just (96 crates), los
74,2 s de cargo-make (182 crates) y los 10 min 45 s de mise (995 crates, 275 MB y
cuatro paquetes del sistema). cargo-xtask no instala nada, pero cada tarea pasa a ser
código en Rust que hay que mantener, y obliga a convertir el proyecto en un
*workspace*: es la deuda técnica que se quiere evitar. Al ejecutar `check` en un
contenedor recién creado, make y just no añaden tiempo apreciable a `cargo check`,
mientras que cargo-make añade unos 0,14 s, cargo-xtask unos 0,12 s y mise unos 0,03 s,
que se pagarían en cada ejecución de la CI. El proyecto solo necesita tres tareas de
una orden cada una, así que las funciones de más de just, cargo-make y mise no
compensan.

Inconvenientes asumidos:

- make está pensado para generar ficheros, así que las tareas que no generan ninguno
  se declaran con `.PHONY`.
- Las órdenes de cada tarea tienen que ir sangradas con tabuladores.
- No viene en todas partes: en la imagen `rust:1.99-slim` hay que instalarlo, y en
  Windows no viene de serie (el proyecto se desarrolla en WSL, donde sí está).

## Tareas

- `make install` ejecuta `cargo fetch`, que descarga las dependencias.
- `make check` ejecuta `cargo check`, que comprueba que los fuentes compilan, sin
  generar el binario ni ejecutar los tests.
- `make test` ejecuta `cargo test`, que ejecuta los tests.

## Fuentes

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [The Rust Programming Language, cap. 1.1: Installation](https://rust-lang.github.io/book/ch01-01-installation.html)
- [make, especificación POSIX (IEEE Std 1003.1)](https://man.archlinux.org/man/make.1p.en)
- [cargo-xtask](https://github.com/matklad/cargo-xtask)
