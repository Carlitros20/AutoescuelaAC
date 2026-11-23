#[derive(Debug, PartialEq)]
pub enum ErrorCargaAlumnos {
    FilaIncompleta(usize),
    NumeroMalEscrito(usize),
    CodigoPostalMalEscrito(usize),
    NumeroRepetido(u32),
}
