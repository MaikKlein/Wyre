extern mod wyre;
use wyre::*;
fn main(){
  let pipe = do Pipe::new()|p|{ 
                SingleWire.connect(|x| fmt!("%?",x) ,
                  SingleWire.connect(|x| x as float ,
                    SingleWire.connect(|x: int| x * 2 ,
                      p
                    )
                  )
                )    
              };
   
   // queue your messages           
  do 100.times() {
    pipe.send(21);
  }
  // collect your messages
  do 100.times(){
    // receives and waits for the result
    // alternative: use .recv() to get the result immediately.
    // prints '42' 100 times.
    println(pipe.recv_wait());
  }
  // closes all open tasks that are associated with 'pipe'
  pipe.shutdown();

}