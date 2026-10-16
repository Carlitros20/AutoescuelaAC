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
- **Coste de instalación**: ninguno. Se instala con la propia toolchain que fija
  `rust-toolchain.toml`.
- **Alternativas**: no hay otro gestor de paquetes para el registro de crates de Rust;
  prescindir de Cargo supondría descargar y compilar las dependencias a mano.
- **Instalar dependencias**: `cargo fetch` descarga las dependencias de `Cargo.lock`
  sin compilarlas.

## Fuentes

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
