extern mod extra;

pub struct Pipe<T,U>{
  priv chan: Chan<T>,
  priv port: Port<U>
}
pub struct SingleWire;

enum Message<T> {
    Exit,
    Value(T)
}
trait Wire<T: Send,U: Send> {
  fn connect(&self, f: ~fn(T)->U,port: Port<Message<T>>) -> Port<Message<U>>;
}
trait Sender<T: Send, U: Send>{
  fn send(&self, t: T);
  fn shutdown(&self);

}
trait Receiver<T: Send, U: Send> {
  fn recv(&self) -> Option<U>;
  fn recv_wait(&self) -> U;
}
impl <T: Send,U: Send> Pipe<Message<T>,Message<U>>{
    fn drop(&self) {
        self.shutdown(); // doesn't work
    }
}
impl <T: Send,U: Send> Pipe<Message<T>,Message<U>>{
  pub fn new(f: ~fn(Port<T>) -> Port<U>)-> Pipe<T,U> {
    let (in_port, chan): (Port<T>, Chan<T>) = stream();
    let out_port = f(in_port);
    Pipe { chan: chan,
           port: out_port} 
  }
}
impl <T: Send,U: Send> Sender<T,U> for Pipe<Message<T>,Message<U>>{
  

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
impl <T: Send,U: Send> Receiver<T,U> for Pipe<Message<T>,Message<U>> {
  fn recv(&self) -> Option<U>{
    match self.port.peek() {
      true  => Some(self.recv_wait()),
      false => None
    }
  }

  fn recv_wait(&self) -> U {
    match self.port.recv(){
      Exit => fail!(~"Tried to receive on 'Exit'"),
      Value(x) => x
    }
  }
}

impl<T: Send,U: Send> Wire<T,U> for SingleWire  {
      fn connect(&self, f: ~fn(T)->U,port: Port<Message<T>>) -> Port<Message<U>> {
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
}


