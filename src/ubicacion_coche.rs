use std::time::SystemTime;

pub struct UbicacionCoche {
    codigo_postal: String,
    instante: SystemTime,
}

impl UbicacionCoche {
    pub(crate) fn new(codigo_postal: String, instante: SystemTime) -> Self {
        UbicacionCoche {
            codigo_postal,
            instante,
        }
    }
}
