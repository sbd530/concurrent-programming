use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// Condvar 타입의 변수가 조건 변수이며
// Mutext와 Condvar를 포함하는 튜플이 Arc에 포함되어 전달된다.

// 대기 스레드용 함수
fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    // 스레드 고유의 번호 id
    let &(ref lock, ref cvar) = &*p;

    // 뮤텍스 락
    let mut started = lock.lock().unwrap();
    while !*started {
        // Mutex 안 공유변수가 false인 동안 루프
        // wait
        started = cvar.wait(started).unwrap();
    }

    // 위와 같지만 wait_while도 가능
    // cvar.wait_while(started, |started| !*started).unwrap();

    println!("child {}", id);
}

// 알림 스레드용 함수
fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    // 뮤텍스 락
    let mut started = lock.lock().unwrap();
    // 공유 변수 업데이트
    *started = true;
    // 알림
    cvar.notify_all();
}

fn main() {
    // 뮤텍스와 전건 변수 작성
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || child(0, pair0));
    let c1 = thread::spawn(move || child(1, pair1));
    let p = thread::spawn(move || parent(pair2));

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}
