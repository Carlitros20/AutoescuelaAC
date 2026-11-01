use std::time::SystemTime;

pub struct Clase {
    dni_alumno: String,
    aviso_cancelacion: Option<SystemTime>,
}

impl Clase {
    pub(crate) fn new(dni_alumno: String) -> Self {
        Clase {
            dni_alumno,
            aviso_cancelacion: None,
        }
    }
}
