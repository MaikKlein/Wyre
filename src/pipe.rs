extern mod extra;

use message::*;
pub struct LinearPipe<T,U>{
  priv chan: Chan<T>,
  priv port: Port<U>
}

trait Wire<T: Send,
           U: Send, 
           R: Send> {
  fn connect(&self, 
             f: ~fn(T)->U,
             port: Port<Message<T>>) 
             -> R;
}
trait Sender<T: Send>{
  fn send(&self, t: T);
  fn shutdown(&self);

}
trait Receiver<U: Send> {
  fn try_recv(&self) -> Option<U>;
  fn recv(&self) -> U;
}

impl <T: Send,
      U: Send> 
      LinearPipe<Message<T>,Message<U>>{
  pub fn new(f: ~fn(Port<T>) -> Port<U>)-> LinearPipe<T,U> {
    let (in_port, chan): (Port<T>, Chan<T>) = stream();
    let out_port = f(in_port);
    LinearPipe { chan: chan,
           port: out_port} 
  }
}
impl <T: Send,
      U: Send> 
      Sender<T> for 
      LinearPipe<Message<T>,Message<U>>{
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
impl <T: Send,
      U: Send> 
      Receiver<U> for 
      LinearPipe<Message<T>,Message<U>> {
  fn try_recv(&self) 
              -> Option<U>{
    match self.port.peek() {
      true  => Some(self.recv()),
      false => None
    }
  }

  fn recv(&self) 
          -> U {
    match self.port.recv(){
      Exit => fail!(~"Tried to receive on 'Exit'"),
      Value(x) => x
    }
  }
}
