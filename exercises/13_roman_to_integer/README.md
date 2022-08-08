# Roman to Integer
## The Problem
We need to convert any Roman Numeral to an Arabic Integer up to the number 3999.
## Examples
-  III -> 3
-  LVIII -> 58
- MCMXCIV -> 1994

## The Straightforward Solution
- Good Time Complexity `O(2n)`
- Excellent Memory Usage `O(1)`

My Basic approach to solving this problem is to read through the characters of the Roman Numeral (Given as a `String`) and first push them into an array whilst converting them to the relative integers using a `HashMap` or a `match` to then process them in a second loop where I either push them to the result or first subtract them in cases such as this `[1, 5]` where we're supposed to have a `4` because the next value is greater than the current one.

This could be improved by processing everything in a single loop but we still have a linear scaling here.

## The Best Solution
- Excellent Time Complexity `O(n)`
- Excellent Memory Usage `O(1)`

This Solution is the best and it consists of a single loop where we work from end to start giving us the possibility of not needing more than a single loop by keeping track of the converted value of the current and previous value as well as the final sum.
We first add the current value and then check its relationship with the previous to know whether or not we need to subtract anything `curr = 1 prev = -> prev > curr -> sum += -curr (5 - 1 = 4)`

I'm trying to keep the code more readable by memorizing the curr, prev and sum values in a tuple that's logically connected with the process we're following.