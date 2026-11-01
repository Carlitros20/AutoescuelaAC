use crate::{alumno::Alumno, coordenadas::Coordenadas};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dni(String);

impl Dni {
    pub fn new(valor: String) -> Result<Self, String> {
        let caracteres: Vec<char> = valor.chars().collect();
        let valido = caracteres.len() == 9
            && caracteres[..8].iter().all(|c| c.is_ascii_digit())
            && caracteres[8].is_ascii_alphabetic();

        if !valido {
            return Err(format!("'{}' no tiene 8 cifras y una letra", valor));
        }
        Ok(Dni(valor))
    }
}

pub struct AlumnosRegistrados {
    alumnos: HashMap<Dni, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
        }
    }

    pub fn registrar(
        &mut self,
        dni: Dni,
        nombre: String,
        latitud: f64,
        longitud: f64,
    ) -> Result<(), String> {
        if self.alumnos.contains_key(&dni) {
            return Err(format!("ya existe un alumno con DNI {:?}", dni));
        }
        let ubi = Coordenadas::new(latitud, longitud);
        let alumno = Alumno::new(nombre, ubi);
        self.alumnos.insert(dni, alumno);
        Ok(())
    }
}
