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

| | make | just | cargo-make | cargo-xtask |
|---|---|---|---|---|
| Estándar | Sí, forma parte de POSIX (IEEE Std 1003.1) | No | No | No, es una convención: un crate del propio proyecto llamado con un alias de Cargo |
| Instalación en el entorno del proyecto | Ya instalado (ver abajo). En la imagen `rust:1.99-slim` no viene: `apt-get update` e instalar el paquete `make` (sin compilar) tarda 3,3 s y ocupa 1,8 MB ([captura](../img/make-rust-slim.png)) | `cargo install just --locked` en `rust:1.99-slim`: 96 crates compilados, 61,5 s, binario de 6,2 MB (v1.59.0) ([captura](../img/just-rust-slim.png)) | `cargo install cargo-make --locked` en `rust:1.99-slim`: 182 crates compilados, 74,2 s, binario de 9,7 MB más un segundo ejecutable, `makers` (v0.37.24) ([captura](../img/cargo-make-rust-slim.png)) | Nada que instalar, pero el crate se compila la primera vez; en `rust:1.99-slim`, 0,15 s y un binario de 4,7 MB ([captura](../img/cargo-xtask-rust-slim.png)) |
| Formato del fichero | `Makefile`, estable | `justfile`, propio de la herramienta | `Makefile.toml`, propio de la herramienta | Un programa en Rust (19 líneas para las tres tareas), su `Cargo.toml` y el alias; obliga a convertir el proyecto en un *workspace* |

**Por qué make no tiene coste de instalación**: para compilar, Rust necesita un
enlazador, y la documentación oficial de Rust indica que en Ubuntu se obtiene
instalando el paquete `build-essential`. Ese paquete depende de `make`
(`apt-cache depends build-essential`), así que cualquier entorno capaz de compilar el
proyecto ya tiene make. En el entorno de desarrollo está GNU Make 4.4.1.

### Decisión: make

make es la única de las cuatro que es un estándar, no añade nada que instalar en un
entorno donde el proyecto ya compila y su formato no ha cambiado en décadas. just y
cargo-make ofrecen más funciones, pero el proyecto solo necesita tres tareas de una
orden cada una, así que esas funciones no compensan compilar e instalar otra
herramienta. cargo-xtask no instala nada, pero cada tarea pasa a ser código que hay
que mantener y cambia la estructura del proyecto, que es justo la deuda técnica que
se quiere evitar.

Inconvenientes asumidos:

- make está pensado para generar ficheros, así que las tareas que no generan ninguno
  se declaran con `.PHONY`.
- Las órdenes de cada tarea tienen que ir sangradas con tabuladores.
- En Windows no viene de serie; el proyecto se desarrolla en WSL, donde sí está.

## Tareas

| Tarea | Orden | Qué hace |
|---|---|---|
| `make install` | `cargo fetch` | Descarga las dependencias |
| `make check` | `cargo check` | Comprueba que los fuentes compilan, sin generar binario ni ejecutar tests |
| `make test` | `cargo test` | Ejecuta los tests |

## Fuentes

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [The Rust Programming Language, cap. 1.1: Installation](https://rust-lang.github.io/book/ch01-01-installation.html)
- [make, especificación POSIX (IEEE Std 1003.1)](https://man.archlinux.org/man/make.1p.en)
- [cargo-xtask](https://github.com/matklad/cargo-xtask)
