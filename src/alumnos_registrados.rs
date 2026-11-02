use crate::{alumno::Alumno, dni::Dni};
use std::collections::HashMap;

pub struct AlumnosRegistrados {
    alumnos: HashMap<Dni, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
        }
    }
}
