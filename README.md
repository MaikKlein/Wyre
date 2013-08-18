# Wyre *Work In Progress*
The purpose of Wyre is to run linear code concurrently. If you have a function that looks like `f(g(h(x)))` it will look
something like `x -> [Task 1, h] -> [Task 2, g] -> [Task 3, f] -> y` where f,h and g are your functions from
type fn(T)->U.

## Example code


[]



[![IMAGE ALT TEXT HERE](http://img.youtube.com/vi/YOUTUBE_VIDEO_ID_HERE/0.jpg)](https://github.com/MaikKlein/Wyre/blob/master/examples/linear_pipe.rs)

As you can see it takes an int, transforms it to a float and transforms it to a str.

## Instructions

### Building Wyre



### Building the examples

1. `$ cd wyre/examples`
2. `$ make` or for a specific example `$ make <example name>` (eg. `$ make window`)


