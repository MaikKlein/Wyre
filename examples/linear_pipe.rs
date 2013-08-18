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

  do 5.times(){
    pipe.send(21);
  }
  do 5.times(){
    printfln!(pipe.recv_wait());
  }
   

  pipe.shutdown();
}