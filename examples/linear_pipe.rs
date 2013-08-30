extern mod wyre;
use wyre::pipe::*;
use wyre::wire::*;
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
    printfln!(pipe.recv());
  }
  // closes all open tasks that are associated with 'pipe'
  pipe.shutdown();
}