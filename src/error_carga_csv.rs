#[derive(Debug, PartialEq)]
pub enum ErrorCargaCsv {
    FilaIncompleta(usize),
    NumeroMalEscrito(usize),
    CodigoPostalMalEscrito(usize),
    NumeroRepetido(u32),
}
