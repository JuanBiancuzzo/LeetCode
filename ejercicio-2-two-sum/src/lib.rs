/*
 * Constraints:
 *  2 <= nums.length <= 104
 *  -109 <= nums[i] <= 109
 *  -109 <= target <= 109
 *  Only one valid answer exists.
 * */

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>  {
    vec!{0, 1}
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
 
    #[test]
    fn ejemplo_3() {
        let resultado = two_sum(vec!{3, 3}, 6);
        let resultado_esperado = vec!{0, 1};

        assert_eq!(resultado, resultado_esperado);
    }   }

    #[test]
    fn no_existe_combinacion() {
        let resultado = two_sum(vec!{3, 2}, 6);
        let resultado_esperado = vec!{};

        assert_eq!(resultado, resultado_esperado);
    }
}
