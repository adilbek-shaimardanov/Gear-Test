use std::{sync::{Arc, Mutex}, thread};
use rand::prelude::*;

const THRESHOLD: usize = 5;

fn main() {
    let mut vec = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        vec.push((rng.gen::<f32>() * 50.0).ceil() as i32)
    }
    let f = |x: i32| x.pow(2);
    let vec = Arc::new(vec);
    let result = task1(&vec, f);
    println!("{:?}", vec);
    println!("{:?}", Arc::try_unwrap(result).unwrap().into_inner().unwrap());
}

fn task1<
    T: Clone + Sync + Send + 'static,
    R: Default + Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static
>(vec: &Arc<Vec<T>>, f: F) -> Arc<Mutex<Vec<R>>> {
    let len = vec.len();
    let num_threads = ((len as f32) / (THRESHOLD as f32)).ceil() as i32;

    let data_idx = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mut result_vector = Vec::with_capacity(len);
    for _ in 0..len {
        result_vector.push(R::default());
    }
    let result_vector = Arc::new(Mutex::new(result_vector));
    let f_arc = Arc::new(f);

    for _ in 0..num_threads {
        let data = Arc::clone(&vec);
        let result_vector = Arc::clone(&result_vector);
        let idx = Arc::clone(&data_idx);
        let f = Arc::clone(&f_arc);
        let handle = thread::spawn(move || {
            loop {
                let mut i = idx.lock().unwrap();
                if *i >= len {
                    return
                }
                let i_ = (*i).clone();
                *i += 1;
                drop(i);
                let input = &data[i_];
                let output = f(input.clone());
                let mut result_vector = result_vector.lock().unwrap();
                result_vector[i_] = output;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    result_vector
}