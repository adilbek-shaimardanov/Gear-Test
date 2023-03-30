use std::{sync::{Arc, Mutex}, thread};

const THRESHOLD: usize = 5;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let len = data.len();
    let num_threads = ((len as f32) / (THRESHOLD as f32)).ceil() as i32;

    let data = Arc::new(Mutex::new(data));
    let data_idx = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let data = Arc::clone(&data);
        let idx = Arc::clone(&data_idx);
        let handle = thread::spawn(move || {
            loop {
                let mut i = idx.lock().unwrap();
                if *i >= len {
                    return
                }
                let i_ = (*i).clone();
                *i += 1;
                drop(i);
                let _data = data.lock().unwrap();
                let mut d: i32 = (*_data)[i_];
                drop(_data);
                d = d.pow(2);
                let mut _data = data.lock().unwrap();
                (*_data)[i_] = d;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}