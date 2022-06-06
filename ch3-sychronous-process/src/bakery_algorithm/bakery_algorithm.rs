//* 베이커리 알고리즘
//* 병원같은 곳에서 줄서기위해, 먼저 접수를 하고 번호가 적힌 티켓을 받는다.
//* 다른 대기중인 사람이 가진 티켓 번호보다 자신의 번호가 작을 때 진료를 받을 수 있다.

// 최적화 억제 읽기/쓰기용
use std::ptr::{read_volatile, write_volatile};
// 메모리 배리어용
use std::sync::atomic::{fence, Ordering};
use std::thread;

// 스레드 수
const NUM_THREADS: usize = 4;
// 각 스레드에서 루프 수
const NUM_LOOP: usize = 100000;

// volatile용 매크로
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

// 베이커리 알고리즘용 타입
struct BakeryLock {
    // i번째 스레드가 티켓을 획득 중이면 entering[i]는 true
    entering: [bool; NUM_THREADS],
    // i번째 스레드의 티켓은 ticket[i]
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    // 락 함수. idx는 스레드 번호
    fn lock(&mut self, idx: usize) -> LockGuard {
        // 티켓 취득 처리
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        // 현재 배포되어 있는 티켓의 최댓값 취득
        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }
        // 최댓값 + 1 을 가진의 티켓 번호로 한다.
        let ticket = max + 1;
        write_mem!(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], false);
        fence(Ordering::SeqCst);

        // 대기 처리
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            // 스레드 i가 티켓 취득 중이면 대기
            while read_mem!(&self.entering[i]) {}

            loop {
                // 스레드 i와 자신의 순서를 비교해 자신의 순서가 높거나
                // 스레드 i가 처리 중이 아니면 대기 종료
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        // 스레드 i의 티켓번호보다 자신의 번호가 낮거나 티켓번호가 같고 자신의 스레드 번호가 작으면 대기 종료
                        if ticket < t || (ticket == t && idx < i) {
                            break;
                        }
                    }
                    None => {
                        // 스레드 i가 처리중이 아니면 대기 종료
                        break;
                    }
                }
            }
        }

        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

// 락 관리용 타입
struct LockGuard {
    idx: usize,
}
impl Drop for LockGuard {
    // 락 해제 처리
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
    }
}

// 글로벌 변수
static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

fn main() {
    // NUM_THREADS만큼 스레드 생성
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            // NUM_LOOP만큼 루프 만복하면서 COUNT증가
            for _ in 0..NUM_LOOP {
                // 락 획득
                let _lock = unsafe { LOCK.lock(i) };
                unsafe {
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
