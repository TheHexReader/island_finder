# Island finder

Generate an island of bool values, then find islands of "true" values.

## Example

Generating an island of size (5, 5) will generate something like:
```
0 1 0 1 1
1 1 1 0 0
0 0 0 1 0
0 0 0 1 0
1 1 0 0 1
```
In this map you can find 5 islands, the form of islands will be:
```
       1   |          |      1 |
x1 = 1 1 1 | x2 = 1 1 | x1 = 1 | x1 = 1
           |          |        |
```
The program will return the number of those islands that it could find. In this case, the program will return "Found 5 islands".