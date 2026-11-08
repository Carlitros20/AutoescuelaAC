# Milestones

## Milestone 0: modelo del dominio de HU001
### Historia de usuario
HU001.

### Cómo se hace
A partir de HU001 se plantean issues que enuncian los problemas de su dominio,
identificando los conceptos clave que aparecen en ella. Esos issues guían la
aplicación de **domain driven design**: en cada uno se discute qué conceptos son
objetos valor, cuáles entidades y si hay agregados, y se codifican en el lenguaje
elegido, siguiendo sus buenas prácticas e incluyendo los errores que puedan darse al
crearlos. Cada commit resuelve un issue y lo referencia.

### Cómo se comprueba su validez
Recorriendo la relación entre HU001, sus issues y el código: todo lo que hay en el
código responde a un issue, cada issue enuncia un problema de HU001, y en cada uno
queda razonado por qué un concepto se ha modelado como objeto valor, entidad o
agregado. Con eso, quien revise puede juzgar si el código es una modelización
suficiente y mínima de HU001 sobre la que programar su lógica de negocio.

## Milestone 1: primera lógica de negocio de HU001
### Historia de usuario
HU001.

### Cómo se hace
Sobre el modelo del Milestone 0 se plantean issues con los problemas de lógica de
negocio de HU001, y se resuelven en las entidades del modelo escribiendo, para cada
uno, tests que comprueban la solución, incluidos los casos de error.

### Cómo se comprueba su validez
Los tests se ejecutan automáticamente y se superan, y cada test se relaciona con un
issue y este con HU001.