#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

// #[derive(Debug)]
// enum StatusMessage {
    // Ok,
// }

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

impl GroundStation {
    fn send(
        &self,
        to: &mut CubeSat,
        msg: Message,
    ) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

struct GroundStation;

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox {
            messages: vec![],
        },
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a,
                Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}

// fn check_status(sat_id: CubeSat) -> CubeSat {
    // println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    // sat_id
// }

// fn main() {
//     let sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] } };
//     let sat_b = CubeSat { id: 1, mailbox: Mailbox { messages: vec![] } };
//     let sat_c = CubeSat { id: 2, mailbox: Mailbox { messages: vec![] } };
//
//     let sat_a = check_status(sat_a);
//     let sat_b = check_status(sat_b);
//     let sat_c = check_status(sat_c);
//
//     // "waiting" ...
//     let sat_a = check_status(sat_a);
//     let sat_b = check_status(sat_b);
//     let sat_c = check_status(sat_c);
// }
