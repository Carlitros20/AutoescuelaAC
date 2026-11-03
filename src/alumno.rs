use crate::coordenadas::Coordenadas;

pub struct Alumno {
    nombre: String,
    direccion: Coordenadas,
}

impl Alumno {
    pub(crate) fn new(nombre: String, direccion: Coordenadas) -> Self {
        Alumno { nombre, direccion }
    }
}
