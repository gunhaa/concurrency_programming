use std::sync::{Arc, Mutex};
use std::thread;

pub fn default_implements_philosopers() {
    let c0_p0 = Arc::new(Mutex::new( 0 ));
    let c1_p0 = Arc::new(Mutex::new( 0 ));

    let c0_p1 = c0_p0.clone();
    let c1_p1 = c1_p0.clone();

    let p0 = thread::spawn(move || {
        for _ in 0..100000 {
            let mut _n0 = c0_p0.lock().unwrap();
            let _n1 = c1_p0.lock().unwrap();
            *_n0 += 1;
            println!("p0: eating : {}", _n0);
        }
    });

    let p1 = thread::spawn(move || {
        for _ in 0..100000 {
            // 이 구조라면, 데드락이 생기지 않는다
            // let _n1 = c0_p1.lock().unwrap();
            // let _n2 = c1_p1.lock().unwrap();

            // 데드락 발생
            let _n1 = c1_p1.lock().unwrap();
            let _n0 = c0_p1.lock().unwrap();
            println!("p1: eating");
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();
}
