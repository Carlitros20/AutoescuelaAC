use std::time::SystemTime;

pub struct Clase {
    aviso_cancelacion: Option<SystemTime>,
}

impl Clase {
    pub(crate) fn new() -> Self {
        Clase {
            aviso_cancelacion: None,
        }
    }
}
