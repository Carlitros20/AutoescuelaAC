use std::time::{Duration, SystemTime};

use crate::error_margen::ErrorMargen;

pub struct Clase {
    numero_alumno: u32,
    aviso_cancelacion: Option<SystemTime>,
}

impl Clase {
    pub(crate) fn new(numero_alumno: u32) -> Self {
        Clase {
            numero_alumno,
            aviso_cancelacion: None,
        }
    }

    pub(crate) fn registrar_aviso(&mut self, instante: SystemTime) {
        self.aviso_cancelacion = Some(instante);
    }

    pub(crate) fn margen(&self, inicio: SystemTime) -> Result<Duration, ErrorMargen> {
        let aviso = self.aviso_cancelacion.ok_or(ErrorMargen::SinAviso)?;
        match inicio.duration_since(aviso) {
            Ok(margen) if !margen.is_zero() => Ok(margen),
            _ => Err(ErrorMargen::AvisoTardio),
        }
    }
}
