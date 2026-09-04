/*
 * Constraints:
 *  2 <= nums.length <= 104
 *  -109 <= nums[i] <= 109
 *  -109 <= target <= 109
 *  Only one valid answer exists.
 * */
const MIN_LEN: usize = 2;
const MAX_LEN: usize = 104;
const MIN_NUM: i32 = -109;
const MAX_NUM: i32 = 109;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>  {
    if target < MIN_NUM || MAX_NUM < target {
        return vec!{}

    } else if nums.len() < MIN_LEN || MAX_LEN < nums.len() {
        return vec!{}
    }

    for i in 0..nums.len() {
        let primero = match nums.get(i) {
            Some(primero) if MIN_NUM <= *primero && *primero <= MAX_NUM => primero,
            _ => return vec!{},
        };

        for j in i+1..nums.len() {
            let segundo = match nums.get(j) {
                Some(segundo) if MIN_NUM <= *segundo && *segundo <= MAX_NUM => segundo,
                _ => return vec!{},
            };

            if primero + segundo != target {
                continue
            }

            return vec![i as i32, j as i32]
        }
    }

    vec!{}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_1() {
        let resultado = two_sum(vec!{2, 7, 11, 15}, 9);
        let resultado_esperado = vec!{0, 1};

        assert_eq!(resultado, resultado_esperado);
    }

    #[test]
    fn ejemplo_2() {
        let resultado = two_sum(vec!{3, 2, 4}, 6);
        let resultado_esperado = vec!{1, 2};

        assert_eq!(resultado, resultado_esperado);
    }
 
    #[test]
    fn ejemplo_3() {
        let resultado = two_sum(vec!{3, 3}, 6);
        let resultado_esperado = vec!{0, 1};

        assert_eq!(resultado, resultado_esperado);
    }   

    #[test]
    fn no_existe_combinacion() {
        let resultado = two_sum(vec!{3, 2}, 6);
        let resultado_esperado = vec!{};

        assert_eq!(resultado, resultado_esperado);
    }
}
