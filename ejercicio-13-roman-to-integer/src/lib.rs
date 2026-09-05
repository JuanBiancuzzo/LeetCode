use std::fmt;

enum ErrParseo {
    CaracterInvalido { simbolo: char },
}

impl fmt::Debug for ErrParseo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrParseo::CaracterInvalido { simbolo } => {
                f.debug_tuple("ErrParseo::CaracterInvalido")
                    .field(simbolo)
                    .finish()
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Simbolo {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl Simbolo {
    fn valor(&self) -> i32 {
        match self {
            Simbolo::I => 1,
            Simbolo::V => 5,
            Simbolo::X => 10,
            Simbolo::L => 50,
            Simbolo::C => 100,
            Simbolo::D => 500,
            Simbolo::M => 1000,
        }
    }
}

struct NumeroRomano {
    simbolos: Vec<Simbolo>,
    index: usize,
}

impl NumeroRomano {
    fn new(rep: String) -> Result<NumeroRomano, ErrParseo> {
        let mut simbolos: Vec<Simbolo> = Vec::with_capacity(rep.len());
        for caracter in rep.chars() {
            let simbolo = match caracter {
                'I' => Ok(Simbolo::I), 
                'V' => Ok(Simbolo::V), 
                'X' => Ok(Simbolo::X), 
                'L' => Ok(Simbolo::L), 
                'C' => Ok(Simbolo::C), 
                'D' => Ok(Simbolo::D), 
                'M' => Ok(Simbolo::M), 
                _ => Err(ErrParseo::CaracterInvalido{ simbolo: caracter }),
            };

            simbolos.push(simbolo?);
        }

        Ok(NumeroRomano { 
            simbolos: simbolos,
            index: 0,
        })
    }
}

impl Iterator for NumeroRomano {
    type Item = (Simbolo, Option<Simbolo>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        match (self.simbolos.get(index), self.simbolos.get(index + 1)) {
            (Some(actual), Some(siguiente)) => Some((*actual, Some(*siguiente))),
            (Some(actual), None) => Some((*actual, None)),
            (None, _) => {
                self.index = 0;
                None
            }
        }
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut resultado = 0;
    let mut iterator = NumeroRomano::new(s)
        .unwrap()
        .into_iter();

    while let Some((actual, siguiente)) = iterator.next() {
        resultado += match (actual, siguiente) {
            (Simbolo::I, Some(siguiente) ) if siguiente == Simbolo::V || siguiente == Simbolo::X => {
                _ = iterator.next();
                siguiente.valor() - Simbolo::I.valor()
            }
            (Simbolo::X, Some(siguiente) ) if siguiente == Simbolo::L || siguiente == Simbolo::C => {
                _ = iterator.next();
                siguiente.valor() - Simbolo::X.valor()
            }
            (Simbolo::C, Some(siguiente) ) if siguiente == Simbolo::D || siguiente == Simbolo::M => {
                _ = iterator.next();
                siguiente.valor() - Simbolo::C.valor()
            }
            (actual, _) => actual.valor(),
        }
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_1() {
        let resultado = roman_to_int("III".to_string());
        assert_eq!(resultado, 3);
    }

    #[test]
    fn ejemplo_2() {
        let resultado = roman_to_int("LVIII".to_string());
        assert_eq!(resultado, 58);
    }

    #[test]
    fn ejemplo_3() {
        let resultado = roman_to_int("MCMXCIV".to_string());
        assert_eq!(resultado, 1994);
    }
}
