use message::*;
pub fn one_to_many_wire<T: Send,
                        U: Send + Clone>(
                        count: uint, 
                        f: ~fn(T)->U,
                        port: Port<Message<T>>) 
                        -> ~[Port<Message<U>>] {
  let mut pvec = ~[];
  let mut cvec = ~[];
  do count.times(){
    let (out_port, out_chan): (Port<Message<U>>, Chan<Message<U>>) = stream();
    pvec.push(out_port);
    cvec.push(out_chan);
  }
  let cvec = cvec;
  do ::std::task::spawn_unlinked {  
    'loop: loop {  
      let msg = port.recv();
      match msg {
        Exit => { 
          for chan in cvec.iter() {
            chan.send(Exit);
          }
          break 'loop;
        }
        Value(x) => {
          let msg = f(x);
          for chan in cvec.iter() {
            chan.send(Value(msg.clone()));
          }
        }
      }
    }
  }
  pvec
}

fn single_wire<T: Send,
               U: Send>(
               f: ~fn(T)->U,
               port: Port<Message<T>>) 
               -> Port<Message<U>> {
  let (out_port, out_chan): (Port<Message<U>>, Chan<Message<U>>) = stream();
  do ::std::task::spawn_unlinked {  
    loop {  
      let msg = port.recv();
      match msg {
        Exit => { 
          out_chan.send(Exit);
          break;
        }
        Value(x) => out_chan.send(Value(f(x)))
      }
    }
  }
  out_port
}
fn many_to_one_wire<T: Send,
               U: Send>(
               f: ~fn(T)->U,
               pvec: ~[Port<Message<T>>]) 
               -> Port<Message<U>> {

  let (out_port, out_chan): (Port<Message<U>>, Chan<Message<U>>) = stream();
  
  do ::std::task::spawn_unlinked {  
    'main: loop {
      for port in pvec.iter(){
        if port.peek(){        
          let msg = port.recv();
          match msg {
            Value(x) => out_chan.send(Value(f(x))),
            Exit => { 
              out_chan.send(Exit);
              break 'main;
            }
          }
        }
      }
      ::std::task::deschedule();
    }
  }
  out_port
}



