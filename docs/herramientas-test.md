# Herramientas de test

## Criterios de búsqueda

Se buscan aserciones, marcos de test y runners en la documentación oficial de Rust y en crates.io. Solo se tienen en cuenta los que hayan publicado una versión en el último año.

## Criterios de aceptación

Por orden de prioridad:

1. **No escribe en los fuentes**: funciona con el proyecto montado en solo lectura, configurando dónde escribe.
2. **Estándar**: viene con el lenguaje.
3. **Coste de instalación**: tiempo medido en `rust:1.99.0-slim`.
4. **Coste de ejecución**: tiempo de ejecutar los tests, medido en `rust:1.99.0-slim`.
