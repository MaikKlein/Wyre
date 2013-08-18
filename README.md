# Wyre *Work In Progress*
The purpose of Wyre is to run linear code concurrently. If you have a function that looks like `f(g(h(x)))` it will look
something like `x -> [Task 1, h] -> [Task 2, g] -> [Task 3, f] -> y` where f,h and g are your functions from
type fn(T)->U.

## Example code
<script src="https://gist.github.com/MaikKlein/6262167.js"></script>

As you can see it takes an int, transforms it to a float and transforms it to a str.

## Instructions

### Building Wyre



### Building the examples

1. `$ cd wyre/examples`
2. `$ make` or for a specific example `$ make <example name>` (eg. `$ make window`)


