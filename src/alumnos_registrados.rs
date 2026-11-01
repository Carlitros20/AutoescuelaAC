use crate::{alumno::Alumno, coordenadas::Coordenadas};
use std::collections::HashMap;

pub struct AlumnosRegistrados {
    alumnos: HashMap<String, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
        }
    }

    pub fn registrar(&mut self, dni: String, nombre: String, latitud: f64, longitud: f64) {
        let coordenadas = Coordenadas::new(latitud, longitud);
        self.alumnos.insert(dni, Alumno::new(nombre, coordenadas));
    }
}
