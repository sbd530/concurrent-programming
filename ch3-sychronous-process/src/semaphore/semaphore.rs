use std::sync::{Condvar, Mutex};

// Rust 는 표준으로 세마포어를 제공하지 않는다.
// 하지만 뮤텍스와 조건 변수를 이용해서 구현할 수 있다.

// 세마포어용 타입
pub struct Semaphore {
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    pub fn new(max: isize) -> Self {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max,
        }
    }

    pub fn wait(&self) {
        // 락
        let mut cnt = self.mutex.lock().unwrap();
        // 카운터가 최대값 이상이면 대기
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap();
        }
        // 카운터  증가 후 크리티컬 섹션으로 이동
        *cnt += 1;
    }

    pub fn post(&self) {
        // 락
        let mut cnt = self.mutex.lock().unwrap();
        // 카운터 감소
        *cnt -= 1;
        // 카운터가 최대값 이하면 조건 변수로 대기 중인 스레드에 알린다.
        if *cnt <= self.max {
            self.cond.notify_one();
        }
    }
}

//* 세마포어 테스트 코드
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADs: usize = 8;
const SEM_NUM: isize = 4;

static mut CNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut v = Vec::new();
    // SEM_NUM 만틈 동시 실행 가능한 세마포어
    let sem = Arc::new(Semaphore::new(SEM_NUM));

    for i in 0..NUM_THREADs {
        let s = sem.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                s.wait();

                // 아토믹하게 증가 및 감소
                unsafe { CNT.fetch_add(1, Ordering::SeqCst) };
                let n = unsafe { CNT.load(Ordering::SeqCst) };
                println!("semaphore: i = {}, CNT = {}", i, n);
                unsafe { CNT.fetch_sub(1, Ordering::SeqCst) };

                s.post();
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
