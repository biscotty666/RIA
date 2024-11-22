use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
// struct GroundStation {}
struct GroundStation {
    radio_freq: f64 // Mhz
}

fn main() {
//    let base = Rc::new(GroundStation {});
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new (
        GroundStation {
            radio_freq: 87.5
        }
    ));

    println!("base: {:?}", base);
    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}

// Rc<T> implements Clone. It does not allow mutation.
