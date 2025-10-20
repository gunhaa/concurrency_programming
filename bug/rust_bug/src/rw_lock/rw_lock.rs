
use std::sync::{Arc, RwLock};
use std::thread;

// B.Qin이 보고한 데드락을 발생시키는 실제 사례
pub fn deadlock_problem_1() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        // read lock을 획득한다, 동시에 여러 스레드가 가질 수 있다
        // 성공하면 락이 걸린 상태에서 RwLock 안의 데이터를 읽을 수 있는 참조를 반환
        let flag = val.read().unwrap();
        if *flag {
            // 스레드가 이미 read lock을 가지고 있을 때 write lock을 기다리면 데드락이 발생
            // write lock의 기본 논리는 read락이 없을때 획득 가능한 것이다
            *val.write().unwrap() = false;
            // write lock을 얻고, 그 값을 false로 바꾼다
            println!("flag is true");
        }
    });

    t.join().unwrap();
}

// 해당 코드는 데드락이 발생하지 않는다
pub fn deadlock_solution_1() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        // *연산을 통해 값을 복사하고, guard는 drop된다
        // Rust의 Arc 사용법이다
        // *연산으로 가져오면 guard가 drop
        let flag = *val.read().unwrap();
        if flag {
            *val.write().unwrap() = false;
            println!("flag is true");
        }
    });
    t.join().unwrap();
}