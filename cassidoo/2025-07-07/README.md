# Cassidoo 2025-07-07

**Given an array of fireworks representing a series going off, write a function
to find the "grand finale" of the show! A grand finale is defined as the longest
subarray where the average `size` is at least 5, the minimum `velocity` is 3,
and the difference between the min and max `height` is no more than 10. Return
the starting index of the grand finale.**

```javascript
const fireworks = [
  {height: 10, size: 6, velocity: 4},
  {height: 13, size: 3, velocity: 2},
  {height: 17, size: 6, velocity: 3},
  {height: 21, size: 8, velocity: 4},
  {height: 19, size: 5, velocity: 3},
  {height: 18, size: 4, velocity: 4}
];

> grandFinaleStart(fireworks)
> 2
```
