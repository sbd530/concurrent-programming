use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    // 스레드 핸들러를 저장하는 벡터
    let mut v = Vec::new();

    // 10 스레드만큼의 배리어 동기를 Arc로 감싼다
    let barrier = Arc::new(Barrier::new(10));

    // 10 스레드 실행
    for i in 0..10 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            b.wait();
            println!("finished barrier {}", i);
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }
}
