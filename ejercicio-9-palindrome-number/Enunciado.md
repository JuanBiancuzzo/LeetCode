# Ejercicio 9 - Palindrome Number
Given an integer x, return true if x is a palindrome, and false otherwise.

> [!NOTE] Example 1:
> Input: x = 121
> Output: true
> Explanation: 121 reads as 121 from left to right and from right to left.

> [!NOTE] Example 2:
> Input: x = -121
> Output: false
> Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

> [!NOTE] Example 3:
> Input: x = 10
> Output: false
> Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

Constraints:
 * -2^31 <= x <= 2^31 - 1

Follow up: Could you solve it without converting the integer to a string?
Teniendo cuantos digitos tiene, que se calcula como `dig = ceil(log_10(x))` y después avanzar de la siguiente forma:
 * Se puede obtener los extremos para compararlos
    * El extremo inferior esta dado por `inf = x % 10`
    * El extremo superior esta dado por `sup = x // 10^(dig - 1)`
 * Se reduce los extremos
    * Eliminar el extremo superior esta dado por `x = x - sup`
    * Eliminar el extremo inferior esta dado por `x = x / 10`
    * Reducir los digitos por dos `dig = dig - 2`
