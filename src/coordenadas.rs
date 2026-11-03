pub struct Coordenadas {
    latitud: f64,
    longitud: f64,
}

impl Coordenadas {
    pub(crate) fn new(latitud: f64, longitud: f64) -> Self {
        Coordenadas { latitud, longitud }
    }
}
