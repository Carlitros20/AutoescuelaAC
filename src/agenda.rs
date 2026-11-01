use crate::clase::Clase;
use crate::coordenadas::Coordenadas;
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

    pub fn agregar_clase(&mut self, hora_inicio: SystemTime, dni_alumno: String) {
        self.clases.insert(hora_inicio, Clase::new(dni_alumno));
    }

    pub fn modificar_ubicacion_coche(&mut self, latitud: f64, longitud: f64, instante: SystemTime) {
        let coordenadas = Coordenadas::new(latitud, longitud);
        self.ubicacion_coche = Some(UbicacionCoche::new(coordenadas, instante));
    }
}
