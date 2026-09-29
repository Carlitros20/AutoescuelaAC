# Historias de usuario

## [HU001] — Con poco margen, calcular de cabeza qué alumnos llegan a tiempo me hace perder la hora
Como profesor de autoescuela, cuando un alumno cancela una clase con menos de una
hora de margen, tengo que averiguar a qué alumnos podría recoger antes de que
empiece esa clase. Como puedo desviar la ruta de la clase en curso para ir a por el
sustituto, lo que cuenta es a quién puedo llegar desde donde está el coche en ese
momento. Lo calculo de cabeza, comparando direcciones del Excel entre clase y clase,
y ese cálculo consume el propio margen: si se agota antes de dar con un candidato,
la hora se pierde.

### Datos y conceptos
- **Alumnos:** Excel con nombre, DNI, teléfono y dirección de cada uno (25 en
  prácticas). La dirección es el único dato disponible sobre dónde se puede recoger
  a cada alumno.
- **Clases:** duran 45 minutos y se dan entre 10 y 12 al día. Cada una tiene una hora
  de inicio fijada en la agenda del profesor, que es la que determina el margen.
- **Ubicación del coche:** es la del profesor, que va en él, y se obtiene en cada
  momento con la localización de su móvil.
- **Margen:** tiempo que queda desde que llega el aviso de cancelación hasta la hora
  de inicio de la clase cancelada.

**Jornadas relacionadas:** UJ1 y UJ2.

## [HU002] — Con varias cancelaciones el mismo día, cada hueco condiciona la ruta del resto
Como profesor de autoescuela, cuando se me cancelan dos o más clases el mismo día,
ya no puedo resolver cada hueco por separado: lo que decida para uno cambia dónde
estará el coche en los siguientes, así que tengo que reorganizar de cabeza la ruta
de toda la jornada. En esos casos intento citar a todos los sustitutos en un punto
de encuentro céntrico antes de empezar, en lugar de recogerlos durante las clases,
para simplificarme la ruta.

Los datos y conceptos son los mismos que en HU001.

**Jornada relacionada:** UJ3.