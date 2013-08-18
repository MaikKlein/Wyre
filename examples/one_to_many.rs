extern mod wyre;
use wyre::pipe::*;
fn main(){
  let (port,chan) = stream();
  let end_port =  many_to_one(|x|"Result is : "+x,
                    one_to_many_wire(10,|x| x,
                      single_wire(|x| fmt!("%?",x) ,
                        single_wire(|x: int| x * 2 ,
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