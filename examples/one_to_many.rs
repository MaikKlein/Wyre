extern mod wyre;
use wyre::pipe::*;
fn main(){
  let (port,chan) = stream();
  let end_port  = many_to_one(|x| x,
                    one_to_many_wire(10,|x|{println("3");x * 2},
                      single_wire(|x| {println("2");x * 2},
                        single_wire(|x: int| {println("1");x * 2} ,
                          port
                        )
                      )
                    )
                  );
  chan.send(Value(21));
  printfln!(end_port.recv());
  chan.send(Exit);
}