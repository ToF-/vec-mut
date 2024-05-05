use std::cell::{RefCell, RefMut};
use std::rc::Rc;

fn vector_values_modifier(v:&mut Vec<usize>) {  // v is mutable reference to a vector
    v[0] = 4807
}

fn show_values(v: &Vec<usize>) { // non mutable reference to a vector)
    println!("{:?}", v)
}

fn shared_vec_modifier(shared_vec: &Rc<RefCell<Vec<usize>>>, index: usize, value:usize) {
    let mut v: RefMut<'_,_> = shared_vec.borrow_mut();
    v[index] = value;
}

fn main() {
    let mut a:Vec<usize> = vec![];   // a is mutable, so we can push, pop and set values in a
    a.push(42);
    a.push(23);
    a.push(17);
    vector_values_modifier(&mut a); // we pass a mutable reference to the vector, so the function
                                    // can push, pop, and set values in a
    show_values(&a);

    let shared_vec: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(vec![]));
    {
        let mut v: RefMut<'_,_> = shared_vec.borrow_mut();
        v.push(42);
        v.push(23);
        v.push(17);
    }
    shared_vec_modifier(&shared_vec, 0, 4807);
    println!("{:?}", shared_vec);
    shared_vec_modifier(&shared_vec, 2, 33);
    println!("{:?}", shared_vec);
}
