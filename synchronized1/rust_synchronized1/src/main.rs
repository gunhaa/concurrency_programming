use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

fn mutex_func(lock: Arc<Mutex<u64>>) {
    loop {
        let mut val: MutexGuard<'_, u64> = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
    }
}

fn run_mutex() {
// Arc는 쓰레드 세이프한 참조 카운터 타입의 스마트 포인터
    let lock0= Arc::new(Mutex::new(0));

    // 참조 카운터가 증가될 뿐이며 내용은 클론되지 않는다
    let lock1 = lock0.clone();

    // 스레드 생성 후
    // 클로저 내 변수로 이동
    // Rust의 클로저는 람다와 비슷한 동작을 한다
    // move 키워드를 사용하여 소유권을 클로저로 이동시킨다
    let th0 = thread::spawn(move || {
        mutex_func(lock0);
    });

    let th1 = thread::spawn(move || {
        mutex_func(lock1);
    });

    th0.join().unwrap();
    th1.join().unwrap();
}

fn main() {
    run_mutex();}
