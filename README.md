# Wyre *Work In Progress*
Wyre is a small library built on top of the Rust task model. It allows you to model concurrency on a higher abstraction.

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
`LinearPipe` is a small wrapper on top of `Port` and `Chan` which is designed to have exactly one channel and one end port. In this example we are sending an integer '21' from out main task to another task. The integer gets multiplied by 2 and gets forwarded to another task where it will be transformed to a float. Then it will be forwarded to another task and it gets transformed to a '~str'. Finally we can receive the result in our main task via `pipe.recv()`

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

In this example we introduce `one_to_many` and `many_to_one`. `one_to_many` just receives one incoming message and forwards it to n ports. `many_to_one` takes n ports and forwards n messages to one port.

The example above it will output `~"42"` 10 times.
## Instructions

### Building Wyre



### Building the examples

1. `$ cd wyre/examples`
2. `$ make` or for a specific example `$ make <example name>` 


