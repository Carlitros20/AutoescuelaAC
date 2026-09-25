# AutoescuelaAC

## Rol
Soy un profesor de autoescuela.

## Problema

Los alumnos a menudo cancelan las prácticas a última hora, por lo tanto pierdo tiempo y 
dinero si no consigo reasignar esa hora a otro alumno que esté cerca y libre rapidamente.

**Magnitud del problema:** con 25 alumnos en prácticas, 10-12 clases de 45 minutos al día y
una media de 3 cancelaciones de última hora por semana, se pierden unas 2 h 15 min de clase
semanales, entre el 5 % y el 6 % de las clases de la semana,
además del tiempo dedicado a buscar un sustituto.

## Mi relación con el problema
Mi padre es profesor de autoescuela, por lo que he visto esta situación de primera mano:
cuando el alumno no avisa con suficiente antelación, el hueco casi siempre se pierde, con la
pérdida de dinero y de tiempo que eso supone al quedarse sin nada que hacer en esa franja.

## Datos de partida
En un Excel de alumnos se registran: nombre y apellidos, DNI, teléfono y dirección. Esta
última es la que hoy permite, de forma manual, estimar qué alumnos están razonablemente
cerca de la ruta en curso. Aparte, en papel, se anota cuántas clases lleva cada alumno.

## Por qué el método actual falla
- No sé quién está libre sin preguntar uno a uno.
- No se sabe quién está cerca sin repasar direcciones y rutas.
- No hay un registro fiable de prácticas pendientes por alumno.
- Cada consulta lleva minutos, y el margen es de decenas de minutos.

## A quién afecta
- Profesor: hora sin ingreso y tiempo dedicado a gestionarla.
- Autoescuela: vehículo parado en esa franja.
- Alumnos con ganas de más prácticas: no saben que ha surgido un hueco.

## Sobre la lógica de negocio

El sistema no se limita a almacenar y consultar datos:

- **Filtra** la lista de alumnos para quedarse con los que están libres en la franja
  cancelada y todavía necesitan prácticas.
- **Calcula** si cada candidato puede llegar al punto de recogida a tiempo, según su
  dirección y el margen que queda hasta la hora de inicio.
- **Valida** que la reasignación es posible: duración de la franja, que el alumno no tenga
  otra clase a esa hora y que el tipo de práctica encaje.
- **Analiza** a los candidatos restantes para ordenarlos por prioridad (por ejemplo,
  prácticas realizadas).
- **Genera** una propuesta ordenada y la **resume** para el profesor, de modo que pueda
  decidir en segundos a quién ofrecer la hora.

Avisar al alumno es solo el último paso; el valor está en decidir a quién avisar.

## Por qué no basta una aplicación local
El profesor recibe la cancelación y el sustituto debe recibir y confirmar la propuesta desde su propio móvil, 
en un momento distinto y sin estar juntos. Sin un punto de acceso común, 
disponible desde ambos dispositivos en tiempo real, esos dos hechos
no pueden encontrarse a tiempo dentro del margen disponible.

## Planificación del proyecto
El proyecto se ha planificado en historias de usuario, milestones y user
journeys, siguiendo el objetivo 1 de la asignatura:
- [User journeys](docs/user-journeys.md)
- [Personas](docs/personas.md)
- [Milestones](docs/milestones.md)

## Configuración
La configuración del entorno de desarrollo se detalla en [CONFIGURACION.md](CONFIGURACION.md).