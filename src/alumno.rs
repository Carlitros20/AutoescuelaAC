use std::{collections::BTreeMap, time::SystemTime};

use crate::{clase::Clase, coordenadas::Coordenadas, ubicacion_coche::UbicacionCoche};

pub struct Alumno {
    nombre: String,
    direccion: Coordenadas,
    clases: BTreeMap<SystemTime, Clase>,
    ubicacion_coche: Option<UbicacionCoche>,
}

impl Alumno {
    pub(crate) fn new(nombre: String, direccion: Coordenadas) -> Self {
        Alumno {
            nombre,
            direccion,
            clases: BTreeMap::new(),
            ubicacion_coche: None,
        }
    }
}
