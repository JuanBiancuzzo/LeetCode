/*
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] consists of only lowercase English letters if it is non-empty.
 */
pub fn longest_common_prefix(strs: Vec<String>) -> String  {
    if strs.len() < 1 || strs.len() > 200 {
        return "".to_string();
    }

    let mut largo_actual = 0;
    let mut hay_diferencia = false;
    while !hay_diferencia {
        let mut caracter_buscado: Option<char> = None;
        for string in (&strs).into_iter() {
            match (caracter_buscado, string.chars().nth(largo_actual)) {
                (_, None) => { hay_diferencia = true; },
                (None, Some(caracter)) => {
                    caracter_buscado = Some(caracter);
                },
                (Some(caracter_buscado), Some(caracter)) => {
                    if caracter_buscado != caracter {
                        hay_diferencia = true;
                    }
                }
            }
        }
        
        if !hay_diferencia {
            largo_actual += 1;
        }
    }

    strs[0][..largo_actual].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_1() {
        let resultado = longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!(resultado, "fl".to_string());
    }

    #[test]
    fn ejemplo_2() {
        let resultado = longest_common_prefix(vec![
            "dog".to_string(),
            "rececar".to_string(),
            "car".to_string(),
        ]);
        assert_eq!(resultado, "".to_string());
    }

    #[test]
    fn todos_iguales() {
        let resultado = longest_common_prefix(vec![
            "dog".to_string(),
            "dog".to_string(),
            "dog".to_string(),
        ]);
        assert_eq!(resultado, "dog".to_string());
    }

    #[test]
    fn limitado_por_largo() {
        let resultado = longest_common_prefix(vec![
            "dog owner".to_string(),
            "dog".to_string(),
        ]);
        assert_eq!(resultado, "dog".to_string());
    }
}
