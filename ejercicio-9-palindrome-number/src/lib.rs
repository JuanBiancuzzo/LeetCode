/*
 * Constraints:
 *  -231 <= x <= 231 - 1
*/
const MIN_NUM: i32 = i32::MIN;
const MAX_NUM: i32 = i32::MAX;
const BASE: i32 = 10;

pub fn is_palindrome(x: i32) -> bool {
    if x < MIN_NUM || MAX_NUM < x {
        return false
    }

    if x < 0 {
        return false
    }

    let mut num = x;
    let mut digitos = (x as f32).log(BASE as f32).ceil() as i32;

    while digitos >= 1 {
        let limite_inferior = num % BASE;
        let limite_superior = num / (BASE.pow((digitos - 1) as u32));

        if limite_inferior != limite_superior {
            return false
        }

        num -= limite_superior * BASE.pow((digitos - 1) as u32);
        num /= BASE;
        digitos -= 2;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_1() {
        let resultado = is_palindrome(121);
        assert_eq!(resultado, true);
    }

    #[test]
    fn ejemplo_2() {
        let resultado = is_palindrome(-121);
        assert_eq!(resultado, false);
    }

    #[test]
    fn ejemplo_3() {
        let resultado = is_palindrome(10);
        assert_eq!(resultado, false);
    }
}
