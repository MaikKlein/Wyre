extern mod wyre;
use wyre::message::{Value,Exit};
use wyre::wire::*;
fn main(){
  let (port,chan) = stream();
  let end_port  = many_to_one_wire(|x| x,
                    one_to_many_wire(10,|x| x * 2,
                      single_wire(|x| x * 2,
                        single_wire(|x: int| x * 2 ,
                          port
                        )
                      )
                    )
                  );

  chan.send(Value(5));
  do 10.times(){
    printfln!(end_port.recv());
  }
  chan.send(Exit);
}