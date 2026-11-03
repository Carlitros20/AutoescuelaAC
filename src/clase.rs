use std::time::SystemTime;

use crate::alumno::Alumno;

pub struct Clase {
    alumno: Alumno,
    aviso_cancelacion: Option<SystemTime>,
}

impl Clase {
    pub(crate) fn new(alumno: Alumno) -> Self {
        Clase {
            alumno,
            aviso_cancelacion: None,
        }
    }
}
