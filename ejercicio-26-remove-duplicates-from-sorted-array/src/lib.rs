pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0
    }

    let mut indice_limpio = 1;
    for i in 1..nums.len() {
        let numero_anterior = nums[indice_limpio - 1];
        let numero_actual = nums[i];

        if numero_anterior < numero_actual {
            nums[indice_limpio] = nums[i];
            indice_limpio += 1;
        }
    }


    let eliminados = (nums.len() - indice_limpio) as i32;
    _ = nums.split_off(indice_limpio);
    eliminados
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_1() {
        let mut array = vec![1, 1, 2];
        let resultado = remove_duplicates(&mut array);

        assert_eq!(resultado, 1);
        assert_eq!(array, vec![1, 2]);
    }

    #[test]
    fn ejemplo_2() {
        let mut array = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let resultado = remove_duplicates(&mut array);

        assert_eq!(resultado, 5);
        assert_eq!(array, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn sin_duplicados() {
        let mut array = vec![1, 2, 3];
        let resultado = remove_duplicates(&mut array);

        assert_eq!(resultado, 0);
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn vacio() {
        let mut array = vec![];
        let resultado = remove_duplicates(&mut array);

        assert_eq!(resultado, 0);
        assert_eq!(array, vec![]);
    }
}
