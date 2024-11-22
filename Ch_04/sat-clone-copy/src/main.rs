// #[derive(Debug,Copy,Clone)] Automatically derive
#[derive(Debug)]
struct CubeSat {
  id: u64,
}
#[derive(Debug)]
enum StatusMessage {
  Ok,
}

// Manual implementation of Copy and Clone
impl Copy for CubeSat { }

impl Copy for StatusMessage { }

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat { id: self.id }
    }
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn check_status(_sat_id: CubeSat) -> StatusMessage {
  StatusMessage::Ok
}

fn main() {
  let sat_a = CubeSat { id: 0 };
  let a_status = check_status(sat_a.clone());
  println!("a: {:?}", a_status.clone());
  let a_status = check_status(sat_a);
  println!("a: {:?}", a_status);
}
