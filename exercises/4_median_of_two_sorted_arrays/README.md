# Median Of Two Sorted Arrays
## The Problem
We have two sorted arrays (`Vec`) containing integers (`i32`) and we want to find the median (the value in the middle) of the two arrays merged into one. 
## The Solution
- Good Time Complexity `O(log(n+m))`
- Good Memory Usage `O(n)`

Logically this one is quite simple, we just need to merge the arrays and get the middle value or the medium value of the two middle values in the case of an even array.

The only thing that we should keep in mind is that, very often, as this is the case, it's better to use the integrated functions rather than a custom one such as to get an overall better readability as well as speed.

I did try to write an Algorithm that merges the two by checking the relations between the values but it is much simpler to `.concat` the two arrays together and use the `.sort()` method instead of writing a lot of code that, at least in this case, won't run as fast (Probably because there are non-comparative sorts on integers which are a lot faster than traditional ones).

I suppose that the Time Complexity is `O(log(n+m))` because the arrays are already basically sorted.