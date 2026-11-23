use crate::alumno::Alumno;
use std::collections::HashMap;

pub struct AlumnosRegistrados {
    alumnos: HashMap<u32, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
        }
    }
}
