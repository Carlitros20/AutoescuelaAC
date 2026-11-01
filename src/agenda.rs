use crate::alumnos_registrados::Dni;
use crate::clase::Clase;
use crate::ubicacion_coche::UbicacionCoche;
use std::collections::BTreeMap;
use std::time::SystemTime;

pub struct Agenda {
    clases: BTreeMap<SystemTime, Clase>,
    ubicacion_coche: Option<UbicacionCoche>,
}

impl Agenda {
    pub fn new() -> Self {
        Agenda {
            clases: BTreeMap::new(),
            ubicacion_coche: None,
        }
    }

    pub fn agregar_clase(&mut self, hora_inicio: SystemTime, dni_alumno: Dni) {
        self.clases.insert(hora_inicio, Clase::new(dni_alumno));
    }

    pub fn modificar_ubicacion_coche(&mut self, latitud: f64, longitud: f64, instante: SystemTime) {
        self.ubicacion_coche = Some(UbicacionCoche::new(latitud, longitud, instante));
    }
}
