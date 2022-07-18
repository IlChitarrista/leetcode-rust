# Palindrome Number
## The Problem
This Problem is quite simple, we get a `i32` and we need to return True of False based on if it is a Palindrome or not.
## Examples
-  100 -> 001 -> False
-  121 -> 121 -> True
- -121 -> 121- -> False

## The Solution
- Good Time Complexity `O(n)`
- Good Memory Usage `O(n)`

The Solution here is quite simple, we just need to invert it and check whether or not it is identical. We can just skip the negative numbers entirely.

I also included the best possible solution which doesn't use and Array of `Chars` but rather an entirely mathematical process which, although many time faster, isn't as clearly readable. 