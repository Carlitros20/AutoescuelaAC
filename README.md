# AutoescuelaAC

## Rol
Soy un profesor de autoescuela.

## Problema
Los alumnos a menudo cancelan las prácticas a última hora, por lo tanto pierdo tiempo y dinero
si no consigo reasignar esa hora a otro alumno que esté cerca y libre rapidamente.

![Fotografía de la tarjeta de rol](img/rol-cliente.jpg)

## Un caso concreto
Tengo una práctica de 17:00 a 18:00 y el alumno
avisa a las 16:10 de que no puede venir. Tengo 50 minutos para
encontrar a alguien que necesite prácticas, esté libre a esa hora y
pueda llegar al punto de recogida a tiempo. Hoy eso significa ir
escribiendo a alumnos uno a uno sin saber a priori cuál es el candidato
adecuado; si no responde nadie a tiempo, la hora se pierde.

## Mi relación con el problema
Debido a que mi padre es profesor de autoescuela he podido vivir esta situación
de primera mano y la mayoria de las veces acaba perdiendo ese hueco en el
que podria tener otro alumno en el que dar clase, a no ser de que este avise con
suficiente antelación, lo que provoca una perdida
monetaria y de tiempo al ahora no tener nada que hacer.

## Por qué el método actual falla
- No sé quién está libre sin preguntar uno a uno.
- No sé quién está cerca sin consultar a mano su ubicación.
- No sé a quién le conviene más la hora (prácticas pendientes).
- Cada consulta lleva minutos, y el margen es de decenas de minutos.

## A quién afecta
- Profesor: hora sin ingreso y tiempo dedicado a gestionarla.
- Autoescuela: vehículo parado en esa franja.
- Alumnos con ganas de más prácticas: no saben que ha surgido un hueco.

## Datos de partida
Según la tarjeta de rol de desarrollador (ver abajo), se dispone de:
- Un listado de alumnos apuntados, mantenido en un Excel.
- Una lista de alumnos libres, a los que se avisa por correo cuando
  queda una clase libre.

Los alumnos que cancelan no son siempre los mismos, por lo que no hay un
patrón previo que permita anticipar cancelaciones: el sistema tiene que
reaccionar a cada una.

![Tarjeta de rol de desarrollador](img/rol-desarrollador.jpg)

## Por qué no basta una aplicación local
Profesor y alumnos usan dispositivos distintos y actúan en momentos
distintos: la cancelación, la oferta y la aceptación deben encontrarse
en un punto común y accesible en tiempo real.

## Configuración
La configuración del entorno de desarrollo se detalla en [CONFIGURACION.md](CONFIGURACION.md).