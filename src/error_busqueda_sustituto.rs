use crate::error_codigo_postal::ErrorCodigoPostal;
use crate::error_margen::ErrorMargen;

#[derive(Debug, PartialEq)]
pub enum ErrorBusquedaSustituto {
    Margen(ErrorMargen),
    SinUbicacionCoche,
    CodigoPostal(ErrorCodigoPostal),
}

impl From<ErrorMargen> for ErrorBusquedaSustituto {
    fn from(error: ErrorMargen) -> Self {
        ErrorBusquedaSustituto::Margen(error)
    }
}

impl From<ErrorCodigoPostal> for ErrorBusquedaSustituto {
    fn from(error: ErrorCodigoPostal) -> Self {
        ErrorBusquedaSustituto::CodigoPostal(error)
    }
}
