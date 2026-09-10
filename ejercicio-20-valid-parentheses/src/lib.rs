enum Agrupador {
    ParentesisAbierto,
    ParentesisCerrado,
    CorcheteAbierto,
    CorcheteCerrado,
    LlaveAbierto,
    LlaveCerrado,
}

impl Agrupador {
    fn es_abierto(self: &Self) -> bool {
        match self {
            Agrupador::ParentesisAbierto => true,
            Agrupador::ParentesisCerrado => false,
            Agrupador::CorcheteAbierto => true,
            Agrupador::CorcheteCerrado => false,
            Agrupador::LlaveAbierto => true,
            Agrupador::LlaveCerrado => false,
        }
    }

    fn es_cerrado(self: &Self) -> bool {
        !self.es_abierto()
    }

    fn compatible(self: &Self, otro: &Agrupador) -> bool {
        match (self, otro) {
            (Agrupador::ParentesisAbierto, Agrupador::ParentesisCerrado) => true,
            (Agrupador::CorcheteAbierto, Agrupador::CorcheteCerrado) => true,
            (Agrupador::LlaveAbierto, Agrupador::LlaveCerrado) => true,
            _ => false,
        }
    }
}

pub fn is_valid(s: String) -> bool {
    let mut stack_pointer: Option<usize> = None;
    let mut stack: Vec<Agrupador> = Vec::with_capacity(s.len());

    for rep in s.chars() {
        let agrupador = match rep {
            '(' => Agrupador::ParentesisAbierto,
            ')' => Agrupador::ParentesisCerrado,
            '[' => Agrupador::CorcheteAbierto,
            ']' => Agrupador::CorcheteCerrado,
            '{' => Agrupador::LlaveAbierto,
            '}' => Agrupador::LlaveCerrado,
            _ => return false,
        };

        if let Some(puntero) = stack_pointer {
            match stack.get(puntero) {
                Some(previo) if previo.compatible(&agrupador) => {
                    stack_pointer = if puntero == 0 {
                        None
                    } else {
                        Some(puntero - 1)
                    };

                    stack.pop();
                },
                Some(previo) if previo.es_abierto() && agrupador.es_cerrado() => {
                    return false;
                },
                Some(_) => {
                    stack_pointer = Some(puntero + 1);
                    stack.push(agrupador);
                },
                None => return false,
            }

        } else if agrupador.es_abierto() {
            let nuevo_puntero = 0;
            stack_pointer = Some(nuevo_puntero);
            stack.push(agrupador);

        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_1() {
        let resultado = is_valid("()".to_string());
        assert_eq!(resultado, true);
    }

    #[test]
    fn ejemplo_2() {
        let resultado = is_valid("()[]{}".to_string());
        assert_eq!(resultado, true);
    }

    #[test]
    fn ejemplo_3() {
        let resultado = is_valid("(]".to_string());
        assert_eq!(resultado, false);
    }

    #[test]
    fn ejemplo_4() {
        let resultado = is_valid("([])".to_string());
        assert_eq!(resultado, true);
    }

    #[test]
    fn ejemplo_5() {
        let resultado = is_valid("([)]".to_string());
        assert_eq!(resultado, false);
    }
}
