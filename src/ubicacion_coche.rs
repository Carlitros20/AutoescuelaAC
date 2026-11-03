use crate::coordenadas::Coordenadas;
use std::time::SystemTime;

pub struct UbicacionCoche {
    ubi: Coordenadas,
    instante: SystemTime,
}

impl UbicacionCoche {
    pub(crate) fn new(ubi: Coordenadas, instante: SystemTime) -> Self {
        UbicacionCoche { ubi, instante }
    }
}
