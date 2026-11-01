pub struct Direccion {
    latitud: f64,
    longitud: f64,
}

impl Direccion {
    pub(crate) fn new(latitud: f64, longitud: f64) -> Self {
        Direccion { latitud, longitud}
    }
}
