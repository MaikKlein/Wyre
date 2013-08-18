extern mod wyre;
use wyre::pipe::*;
fn main(){
  let (port,chan) = stream();
  let p= one_to_many_wire(10,|x| x,
           single_wire(|x| fmt!("%?",x) ,
             single_wire(|x| x as float ,
               single_wire(|x: int| x * 2 ,
                 port
               )
             )
           )
         );

  // closes all open tasks that are associated with 'LinearPipe'
  chan.send(Value(21));
  for port in p.iter() {
    printfln!("%?",port.recv());
  }
  chan.send(Exit);
}