# Wyre *Work In Progress*
The purpose of Wyre is to run linear code concurrently. If you have a function that looks like `f(g(h(x)))` it will look
something like `x -> [Task 1, h] -> [Task 2, g] -> [Task 3, f] -> y` where f,h and g are your functions from
type fn(T)->U.

## Example code


~~~rust
extern mod wyre;
use wyre::pipe::*;
fn main(){
  let pipe = do LinearPipe::new()|p|{ 
                single_wire(|x| fmt!("%?",x) ,
                  single_wire(|x| x as float ,
                    single_wire(|x: int| x * 2 ,
                      p
                    )
                  )
                )    
              };

  // queue your messages
  do 5.times(){
    pipe.send(21);
  }
  do 5.times(){
    // receives and waits for the result
    // alternative: use .recv() to get the result immediately.
    // prints '42' 5 times.
    printfln!(pipe.recv_wait());
  }
  // closes all open tasks that are associated with 'pipe'
  pipe.shutdown();
}
~~~

As you can see it takes an int, transforms it to a float and transforms it to a str.

## Instructions

### Building Wyre



### Building the examples

1. `$ cd wyre/examples`
2. `$ make` or for a specific example `$ make <example name>` (eg. `$ make window`)


