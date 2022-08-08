# Longest Common Prefix
## The Problem
We're given a Vector of strings out of which we need to find the longest sequence of common chars between the words.
## Examples
- ["flow", "flight", "flowey"] -> "fl"
- ["cat", "dog", "shish"] -> ""
- ["aaaa", "a", "aa"] -> "a"
## The Solution
- "Excellent" Time Complexity `O(n^2)`
- Excellent Memory Usage `O(1)`

The only logically clear solution I could find was to first locate the smallest string inside of the Vector and to then iterate on its chars whilst iterating on each item to compare the chars at the same location. When we find a difference we stop and return the output which was obtained by pushing the current chars as they go by.

I've also implemented an initial check to directly return an empty string when we find a length zero string.
