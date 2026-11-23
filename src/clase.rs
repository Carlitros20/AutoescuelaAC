use std::time::{Duration, SystemTime};

use crate::alumnos_registrados::AlumnosRegistrados;
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

    pub(crate) fn es_de_un_alumno_registrado(&self, alumnos: &AlumnosRegistrados) -> bool {
        alumnos.alumno(self.numero_alumno).is_some()
    }

    pub(crate) fn es_del_alumno(&self, numero_alumno: u32) -> bool {
        self.numero_alumno == numero_alumno
    }

    pub(crate) fn margen(&self, inicio: SystemTime) -> Result<Duration, ErrorMargen> {
        let aviso = self.aviso_cancelacion.ok_or(ErrorMargen::SinAviso)?;
        match inicio.duration_since(aviso) {
            Ok(margen) if !margen.is_zero() => Ok(margen),
            _ => Err(ErrorMargen::AvisoTardio),
        }
    }
}
