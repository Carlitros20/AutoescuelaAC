use crate::alumno::Alumno;
use crate::direccion::Direccion;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dni(String);

pub struct AlumnosRegistrados {
    alumnos: HashMap<Dni, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
        }
    }

    pub fn registrar(&mut self, dni: Dni, nombre: String, calle: String, numero: String) {
        let direccion = Direccion::new(calle, numero);
        let alumno = Alumno::new(nombre, direccion);
        self.alumnos.insert(dni, alumno);
    }
}
