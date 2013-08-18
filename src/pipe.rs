extern mod extra;


pub struct LinearPipe<T,U>{
  priv chan: Chan<T>,
  priv port: Port<U>
}

pub enum Message<T> {
    Exit,
    Value(T)
}
trait Wire<T: Send,U: Send, R: Send> {
  fn connect(&self, f: ~fn(T)->U,port: Port<Message<T>>) -> R;
}
trait Sender<T: Send>{
  fn send(&self, t: T);
  fn shutdown(&self);

}
trait Receiver<U: Send> {
  fn try_recv(&self) -> Option<U>;
  fn recv(&self) -> U;
}

impl <T: Send,U: Send> LinearPipe<Message<T>,Message<U>>{
  pub fn new(f: ~fn(Port<T>) -> Port<U>)-> LinearPipe<T,U> {
    let (in_port, chan): (Port<T>, Chan<T>) = stream();
    let out_port = f(in_port);
    LinearPipe { chan: chan,
           port: out_port} 
  }
}
impl <T: Send,U: Send> Sender<T> for LinearPipe<Message<T>,Message<U>>{
  fn send(&self, t: T) {
      self.chan.send(Value(t));
  }

  fn shutdown(&self) {
     self.chan.send(Exit);
     loop {
       match self.port.recv() {
         Exit => return,
         _    => ()
       }
     }
  }
}
impl <T: Send,U: Send> Receiver<U> for LinearPipe<Message<T>,Message<U>> {
  fn try_recv(&self) -> Option<U>{
    match self.port.peek() {
      true  => Some(self.recv()),
      false => None
    }
  }

  fn recv(&self) -> U {
    match self.port.recv(){
      Exit => fail!(~"Tried to receive on 'Exit'"),
      Value(x) => x
    }
  }
}
pub fn one_to_many_wire<T: Send,U: Send + Clone>(count: uint, f: ~fn(T)->U,port: Port<Message<T>>) -> ~[Port<Message<U>>] {
  let mut pvec = ~[];
  let mut cvec = ~[];
  do count.times(){
    let (out_port, out_chan): (Port<Message<U>>, Chan<Message<U>>) = stream();
    pvec.push(out_port);
    cvec.push(out_chan);
  }
  let cvec = cvec;
  do ::std::task::spawn_unlinked {  
    loop {  
      let msg = port.recv();
      match msg {
        Exit => { 
          for chan in cvec.iter() {
            chan.send(Exit);
          }
          fail!(~"Exit"); 
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

fn single_wire<T: Send,U: Send>(f: ~fn(T)->U,port: Port<Message<T>>) -> Port<Message<U>> {
  let (out_port, out_chan): (Port<Message<U>>, Chan<Message<U>>) = stream();
  do ::std::task::spawn_unlinked {  
    loop {  
      let msg = port.recv();
      match msg {
        Exit => { 
          out_chan.send(Exit);
          fail!(~"Exit"); 
        }
        Value(x) => out_chan.send(Value(f(x)))
      }
    }
  }
  out_port
}
fn many_to_one<T: Send,U: Send>(f: ~fn(T)->U,pvec: ~[Port<Message<T>>]) -> Port<Message<U>> {
  let (out_port, out_chan): (Port<Message<U>>, Chan<Message<U>>) = stream();
  do ::std::task::spawn_unlinked {  
    loop {  
      for port in pvec.iter(){
        if port.peek(){        
          let msg = port.recv();
          match msg {
            Exit => { 
              out_chan.send(Exit);
              fail!(~"Exit"); 
            }
            Value(x) => out_chan.send(Value(f(x)))
          }
        }
      }
    }
  }
  out_port
}


