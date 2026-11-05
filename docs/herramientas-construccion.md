# Herramientas de construcción

El proyecto necesita automatizar tres tareas: instalar las dependencias, comprobar
que los fuentes compilan y ejecutar los tests. El entorno de desarrollo es Linux (en
Windows, WSL con Ubuntu) y la versión de Rust es la 1.99.0, fijada en
`rust-toolchain.toml`.

## Criterios

Por orden de prioridad:

1. **Estándar**: que la herramienta sea la oficial del lenguaje o esté normalizada.
2. **Deuda técnica**: que esté mantenida y que su formato no obligue a rehacer el
   fichero más adelante.
3. **Coste de instalación y de ejecución**, medido en el entorno del proyecto.
4. **No añadir herramientas que no hagan falta** para lo que se necesita ahora.

## Gestor de dependencias: Cargo

- **Estándar**: es el gestor de paquetes oficial de Rust. Las dependencias se declaran
  en `Cargo.toml` y sus versiones exactas quedan fijadas en `Cargo.lock`.
- **Coste de instalación**: viene con la toolchain de Rust, sin nada más que instalar.
  En la imagen `rust:1.99-slim` ya está Cargo 1.99.1, pero `rust-toolchain.toml` fija
  la 1.99.0, así que el primer `cargo fetch` la descarga: 12,1 s y 593 MB. Con ella ya
  instalada, `cargo fetch` tarda 0,03 s ([captura](../img/cargo-rust-slim.png)). El
  coste viene de la versión fijada, no de Cargo.
- **Alternativas**: las que aparecen al buscar alternativas a Cargo para Rust no lo
  sustituyen como gestor de dependencias:
  - [Buck2 con Reindeer](https://github.com/facebookincubator/reindeer): las
    dependencias se declaran en un `Cargo.toml`. `reindeer vendor` las resuelve y
    descarga sin Cargo, pero `reindeer buckify`, que genera las reglas de Buck2, falla
    si Cargo no está, porque ejecuta `cargo metadata`
    ([captura](../img/reindeer-sin-cargo-rust-slim.png)). En `rust:1.99-slim`, instalar
    `curl` y `zstd` y descargar los binarios ya compilados, sin compilar nada, tarda
    11,6 s y ocupa 130 MB `buck2` y 34 MB `reindeer` (v2026.10.05.00). Sin dependencias
    que descargar, como el proyecto ahora, `reindeer vendor` tarda 0,080 s y
    `reindeer buckify` 0,093 s ([captura](../img/buck2-reindeer-rust-slim.png)). No se
    ha medido `buck2 build`, porque aquí se comparan las dependencias, no la compilación.
  - [Bazel con rules_rust (crate_universe)](https://github.com/bazelbuild/rules_rust/tree/main/crate_universe):
    resuelve las dependencias con Cargo por debajo: al resolverlas llama a `cargo fetch`,
    `cargo metadata` y `cargo tree`, y si se le quita el binario de Cargo, `bazel fetch`
    falla (es el error del final de la captura). Además, no usa la versión 1.99.0
    fijada en `rust-toolchain.toml`, sino que descarga su propia Cargo 1.99.1. En
    `rust:1.99-slim`, instalar `curl` y descargar Bazel 9.3.1 tarda 19,2 s y ocupa 74 MB.
    El primer `bazel fetch` de rules_rust 0.75.0, sin dependencias que descargar, tarda
    57,3 s y deja 1,4 GB en la caché de Bazel; el segundo tarda 0,188 s
    ([captura](../img/bazel-rules-rust-rust-slim.png)).
- **Instalar dependencias**: `cargo fetch` descarga las dependencias de `Cargo.lock`
  sin compilarlas.

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

### Herramientas consideradas

- **make**: es un estándar, forma parte de POSIX (IEEE Std 1003.1). No viene en
  `rust:1.99-slim`: `apt-get update` e instalar el paquete `make`, sin compilar nada,
  tarda 3,3 s y ocupa 1,8 MB ([captura](../img/make-rust-slim.png)). Las tareas van en
  un `Makefile`, un formato estable. En `rust:1.99-slim`, con el proyecto recién
  limpiado (`cargo clean`), `make check` tarda 0,062 s, y `cargo check` a secas 0,065 s:
  make no añade un tiempo apreciable ([captura](../img/make-check-rust-slim.png)).
- **just**: no es un estándar. `cargo install just --locked` en `rust:1.99-slim`
  compila 96 crates en 61,5 s y deja un binario de 6,2 MB (v1.59.0)
  ([captura](../img/just-rust-slim.png)). Las tareas van en un `justfile`, un formato
  propio de la herramienta. En `rust:1.99-slim`, con el proyecto recién limpiado
  (`cargo clean`), `just check` tarda 0,082 s, y `cargo check` a secas 0,079 s: just
  no añade un tiempo apreciable ([captura](../img/just-check-rust-slim.png)).
- **cargo-make**: no es un estándar. `cargo install cargo-make --locked` en
  `rust:1.99-slim` compila 182 crates en 74,2 s y deja un binario de 9,7 MB más un
  segundo ejecutable, `makers` (v0.37.24) ([captura](../img/cargo-make-rust-slim.png)).
  Las tareas van en un `Makefile.toml`, un formato propio de la herramienta. En
  `rust:1.99-slim`, con el proyecto recién limpiado (`cargo clean`), `cargo make check`
  tarda 0,210 s, y `cargo check` a secas 0,069 s: cargo-make añade unos 0,14 s en cada
  ejecución ([captura](../img/cargo-make-check-rust-slim.png)).
- **cargo-xtask**: no es un estándar, sino una convención: un crate del propio
  proyecto al que se llama con un alias de Cargo. No hay nada que instalar, pero el
  crate se compila la primera vez; en `rust:1.99-slim` tarda 0,15 s y deja un binario
  de 4,7 MB ([captura](../img/cargo-xtask-rust-slim.png)). Las tareas son un programa
  en Rust (19 líneas para las tres tareas), con su `Cargo.toml` y el alias, y obliga a
  convertir el proyecto en un *workspace*. En `rust:1.99-slim`, con un prototipo
  equivalente, `cargo check` a secas tarda 0,076 s. Tras un `cargo clean`, que también
  borra el binario de `xtask`, `cargo xtask check` tarda 0,193 s, porque vuelve a
  compilarlo: unos 0,12 s más, que es lo que costaría en cada contenedor nuevo. Con
  `xtask` ya compilado tarda 0,050 s, sin tiempo añadido apreciable
  ([captura](../img/cargo-xtask-check-rust-slim.png)).
- **mise**: no es un estándar. En `rust:1.99-slim`, `cargo install mise --locked` falla
  porque le falta un compilador de C++ ([captura](../img/mise-sin-paquetes-rust-slim.png)).
  Instalando antes `g++`, `cmake`, `pkg-config` y `libssl-dev`, todo el proceso tarda
  10 min 45 s, compila 995 crates y deja un binario de 275 MB (v2026.11.2). Las tareas
  van en un `mise.toml`, un formato propio de la herramienta. Con el proyecto recién
  limpiado (`cargo clean`), `mise run check` tarda 0,095 s, y `cargo check` a secas
  0,067 s: mise añade unos 0,03 s en cada ejecución
  ([captura](../img/mise-rust-slim.png)).

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
