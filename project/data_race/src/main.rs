use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            if let Some(elem) = data.get_mut(i) {
                *elem += 1;
            } else {
                println!("Index out of bounds!");
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *data.lock().unwrap());
}

// fn main() {
//     let mut data = vec![1, 2, 3];
//     for i in 0..3{
//         thread::spawn(move || {
//             data[i] += 1;
//         });
//     }

// }
