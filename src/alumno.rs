#[derive(Debug, PartialEq)]
pub struct Alumno {
    nombre: String,
    codigo_postal: String,
}

impl Alumno {
    pub(crate) fn new(nombre: String, codigo_postal: String) -> Self {
        Alumno {
            nombre,
            codigo_postal,
        }
    }
}
