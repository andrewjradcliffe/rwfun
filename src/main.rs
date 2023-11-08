use std::cmp::Ord;
use std::env;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time;

fn is_sorted<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    let n: usize = env::args()
        .nth(1)
        .map_or(10, |s| s.parse::<usize>().unwrap_or(10));
    let x = Arc::new(RwLock::new(Vec::<usize>::new()));
    let mut handles = Vec::with_capacity(n);
    for i in 0..n {
        let x = Arc::clone(&x);
        let handle = thread::spawn(move || {
            {
                let guard = x.read().unwrap();
                if guard.contains(&i) {
                    return;
                }
            }
            let mut guard = x.write().unwrap();
            if guard.contains(&i) {
                return;
            } else {
                guard.push(i);
            }
        });
        handles.push(handle);
    }
    thread::sleep(time::Duration::from_millis(100));
    for handle in handles {
        handle.join().unwrap();
    }
    println!(
        "len: {}; is x sorted?: {}",
        x.read().unwrap().len(),
        is_sorted(x.read().unwrap().as_slice()),
    );
}
