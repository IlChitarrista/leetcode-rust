# Reverse Integer
## The Problem
The Problem is quite simple, we're given an Integer `i32` and tasked with returning the Reverse.
## Examples
- 100 = 1
- -100 = -1
- 123 = 321
- 2222222222222222 = 0 `//Out Of Range`
## The Solution
- Good Time Complexity `O(n)`
- Good Memory Usage `O(n)`

The Solution I came up with seems to be good enough although this could be even more efficiently done without doing the conversions between `i32` -> `String` -> Array of `Chars` and working directly with numbers using Math.

The basic idea is simple, we need to go through the number (as an Array of `Chars`) from the end to the start skipping any 0 we find, after that we can memorize it and convert it back from a number while approximately testing that it isn't going to go out of bound (The Input will always be in range but the Output may not). In order to account for negative numbers we will check for a None value at the end of the array (`-` can't be converted into a digit) which we'll promptly pop and memorize as to multiply the final result by -1 at the end of the process.
