# Milestones

## Milestone 0
### Cómo se empaqueta
Producto interno, que el profesor todavía no usa: código fuente en este
repositorio que aún no decide qué alumno puede cubrir una clase cancelada, sin
interfaz de usuario. Lo acompaña un documento de análisis en docs/, donde se razona cómo se
ha pasado del problema descrito en HU001 al código.

### Cómo se comprueba su validez
Cada término del apartado «Datos y conceptos» de HU001 (alumnos, clases,
ubicación del coche, margen) tiene, en un documento de análisis en docs/, 
una decisión anotada: si se lleva al código o se descarta, y por qué.
Un revisor puede comprobarlo recorriendo esa lista, y comprobando también
que en el código no aparece nada sin una decisión detrás ni ningún cambio sin un
issue derivado de HU001.

## Milestone 1
### Cómo se empaqueta
Producto interno: el código del Milestone 0 con una primera lógica de negocio
orientada al problema de la clase cancelada, sin pretender resolverlo entero, junto
con tests que se lanzan solos.

### Cómo se comprueba su validez
Los tests se ejecutan automáticamente y se superan, y los casos que comprueban
salen de situaciones descritas en HU001 y en sus jornadas de usuario.