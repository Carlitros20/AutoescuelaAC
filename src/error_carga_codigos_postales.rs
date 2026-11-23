#[derive(Debug, PartialEq)]
pub enum ErrorCargaCodigosPostales {
    FilaIncompleta(usize),
    CodigoPostalMalEscrito(usize),
}
