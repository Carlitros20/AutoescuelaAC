use std::time::SystemTime;

use crate::dni::Dni;

pub struct Clase {
    dni_alumno: Dni,
    aviso_cancelacion: Option<SystemTime>,
}

impl Clase {
    pub(crate) fn new(dni_alumno: Dni) -> Self {
        Clase {
            dni_alumno,
            aviso_cancelacion: None,
        }
    }
}
