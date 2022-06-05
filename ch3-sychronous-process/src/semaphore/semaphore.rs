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
