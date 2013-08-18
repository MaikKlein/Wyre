# Wyre *Work In Progress*
The purpose of Wyre is to run linear code concurrently. 

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
    // alternative: use .try_recv() to get the result immediately.
    // prints '42' 5 times.
    printfln!(pipe.recv());
  }
  // closes all open tasks that are associated with 'pipe'
  pipe.shutdown();
}
~~~

As you can see it takes an int, transforms it to a float and transforms it to a str.
~~~rust
extern mod wyre;
use wyre::pipe::*;
fn main(){
  let (port,chan) = stream();
  let end_port =  many_to_one(|x|"Result is : "+x,
                    one_to_many_wire(10,|x| x,
                      single_wire(|x| fmt!("%?",x),
                        single_wire(|x: int| x * 2,
                          port
                        )
                      )
                    )
                  );
  
  chan.send(Value(21));
  
  while end_port.peek() {
    printfln!("%?",end_port.recv());
  }
  
  chan.send(Exit);
}
~~~
## Instructions

### Building Wyre



### Building the examples

1. `$ cd wyre/examples`
2. `$ make` or for a specific example `$ make <example name>` (eg. `$ make window`)


