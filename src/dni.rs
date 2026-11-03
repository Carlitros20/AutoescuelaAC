#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Dni {
    numero: u32,
    letra: char,
}

impl Dni {
    pub(crate) fn new(numero: u32, letra: char) -> Self {
        Dni { numero, letra }
    }
}
