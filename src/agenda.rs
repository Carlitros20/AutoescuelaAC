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
}
