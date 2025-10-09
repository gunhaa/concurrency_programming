use std::ptr::{read_volatile, write_volatile};
use std::sync::atomic::{Ordering, fence};
use std::thread;

const NUM_THREADS: usize = 8;
const NUM_LOOP: usize = 100000;

macro_rules! read_mem {
    ($addr: expr) => {
        unsafe { read_volatile($addr) }
    };
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => {
        unsafe { write_volatile($addr, $val) }
    };
}

struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    fn lock(&mut self, thread_idx: usize) -> LockGuard {
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[thread_idx], true);
        fence(Ordering::SeqCst);

        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }

        let ticket = max + 1;
        write_mem!(&mut self.tickets[thread_idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[thread_idx], false);
        fence(Ordering::SeqCst);

        for i in 0..NUM_THREADS {
            if i == thread_idx {
                continue;
            }

            // entering시 spin 대기
            while read_mem!(&self.entering[i]) {}

            loop {
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        if ticket < t || (ticket == t && thread_idx < i) {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        fence(Ordering::SeqCst);
        LockGuard { thread_idx }
    }
}

struct LockGuard {
    thread_idx: usize,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.thread_idx], None);
    }
}

// global mut var는 권장되지 않는 패턴이다
#[allow(clippy::mutable_static)] // clippy linter 경고 무시
#[allow(unsafe_code)]             // unsafe 사용 경고 무시
static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

pub fn run_bakery_algorithm_example() {
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                #[allow(static_mut_refs)]
                unsafe {
                    let _lock = LOCK.lock(i);
                    let c = read_volatile(&COUNT);
                    write_volatile(&mut COUNT, c + 1);
                }
            }
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        unsafe { COUNT },
        NUM_LOOP * NUM_THREADS
    );
}
