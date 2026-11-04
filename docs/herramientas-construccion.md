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
3. **Coste de instalación**, medido en el entorno del proyecto.
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
  - [Buck2 con Reindeer](https://github.com/facebookincubator/reindeer): Reindeer
    resuelve y descarga las dependencias con Cargo, a partir de `Cargo.toml` y
    `Cargo.lock`, y solo sustituye la compilación. Añade Buck2 y Reindeer.
  - [Bazel con rules_rust (crate_universe)](https://docs.aspect.build/bazelbuild/rules_rust/eb06a7974f448b3a2199d490282906d698c2d316/docs/crate_universe):
    resuelve las dependencias con Cargo por debajo, añade Bazel y está marcado como
    experimental.
- **Instalar dependencias**: `cargo fetch` descarga las dependencias de `Cargo.lock`
  sin compilarlas.

### Decisión: Cargo

Es el gestor estándar de Rust, y las alternativas encontradas no lo sustituyen: Buck2
con Reindeer y Bazel con rules_rust resuelven las dependencias con Cargo por debajo y
además añaden otro sistema de construcción. El único coste medido, 12,1 s la primera
vez en `rust:1.99-slim`, se debe a la versión que fija `rust-toolchain.toml`, no a
Cargo.

## Gestor de tareas

### Cargo no sirve como gestor de tareas

Cargo tiene órdenes sueltas (`cargo fetch`, `cargo check`, `cargo test`), pero no un
fichero de tareas. Lo más parecido son los alias de `.cargo/config.toml`, y se ha
comprobado que no cubren lo que hace falta:

- Un alias solo se expande a una orden de Cargo con sus argumentos. Un alias
  `todo = "check && test"` falla con `error: unexpected argument '&&' found`.
- Un alias no puede llamarse como una orden de Cargo: con `check = "build"`, Cargo
  avisa de que `user-defined alias 'check' is ignored, because it is shadowed by a
  built-in command`.

Por eso las tareas van en una herramienta aparte, y Cargo se usa solo como gestor de
dependencias.

### Alternativas

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

### Decisión: make

make es la única de las cuatro que es un estándar, y su formato no ha cambiado en
décadas. De las que hay que instalar, es la que menos cuesta en `rust:1.99-slim`:
3,3 s y 1,8 MB, sin compilar nada, frente a los 61,5 s de just (96 crates) y los
74,2 s de cargo-make (182 crates). cargo-xtask no instala nada, pero cada tarea pasa
a ser código en Rust que hay que mantener, y obliga a convertir el proyecto en un
*workspace*: es la deuda técnica que se quiere evitar. El proyecto solo necesita tres
tareas de una orden cada una, así que las funciones de más de just y cargo-make no
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
