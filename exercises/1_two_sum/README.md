# Two Sums
## The Problem
Let's imagine that we have an Array of Numbers (Type `i32`) (Unknown at compile time; In Rust we'll use a `Vec<T>` so that memory gets addressed in the Heap and the size of the array isn't fixed) and that there is only a sum of two of these numbers which corresponds to a Target (Type `i32`).

Our Job is to find the index of the items which will, when summed, give us the required Target.
## The Solutions
There are, of course, multiple solutions to this problem but in modern day programming we usually want to focus on Three Aspects which are:

 - Execution Time
 - Memory Usage
 - Readability

The last one is achievable by writing dense code logically in such a way that one can read it with the least amount of comments possible.

In my opinion when we build actual complete solutions of a certain problem we want to focus on Execution Time as that is usually going to be the limiting factor in the performance of our software although that will depend on the context.
### Easiest Solution
 - Bad Scalability `O(n^2)`
 - Constant Memory Usage `O(1)`

The First Solution that comes to mind is that of simply looping through each element of the `Vec` comparing it to each other element to check whether or not it is equal to `target`.

In order to do this we'll need two in-nested loops through the entire array at worst giving us a time complexity of O(n^2) with the only advantage of using a constant memory which is that required to keep track of the indexes.
### Best Solution
- Good Scalability `O(n)`
- Good Memory Usage `O(n)`

The Best Solution would be that of using a `HashMap` to keep track of those elements which we've already seen throught the array because for each element there is only one that can be summed up to the `target`.
#### Example
	let target = 5;
	let nums = vec![-1,0,2,6];

In this case we can simply check whether or not `5-(-1) = 6` exists inside of the HashMap (We can include it using `use std::collections::HashMap`) as that is the only number which, when summed to `-1`, will give us the `target`.

If it does then we have a `solution`, if it doesn't then we should add `-1` to that `HashMap` so that we will when we find `6`. We're now using these numbers as a `key` (`K`) to quickly access the data through the hashing of them but remember that we also need the index to return which is why we memorize it inside the `value` (`V`) of each `key` so that we can return them when we find a solution.

For further details the documentation is of key importance.