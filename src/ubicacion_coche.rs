use crate::alumnos_registrados::AlumnosRegistrados;
use crate::codigos_postales::CodigosPostales;
use crate::error_codigo_postal::ErrorCodigoPostal;
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

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

    pub(crate) fn alumnos_alcanzables(
        &self,
        alumnos: &AlumnosRegistrados,
        codigos: &CodigosPostales,
        margen: Duration,
    ) -> Result<HashSet<u32>, ErrorCodigoPostal> {
        alumnos.alcanzables(codigos, &self.codigo_postal, margen)
    }
}
