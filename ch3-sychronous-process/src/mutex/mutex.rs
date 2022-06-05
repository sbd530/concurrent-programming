use std::sync::{Arc, Mutex};
use std::thread;
// Arc : 스레드 세이프한 참조 카운터 타입의 포인터
fn some_func(lock: Arc<Mutex<u64>>) {
    loop {
        // Mutext용 변수는 보호 대상 데이터를 보존하도록 되어 있어 lock을 하지 않으면 데이터에 접근할 수 없다.
        // try_lock 함수는 락의 획득을 시험해서 획득 가능하면 락을 걸지만, 그렇지 않다면 처리를 되돌린다.
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
    }
}

fn main() {
    // 뮤텍스용 변수의 초기값을 0으로 설정
    let lock0 = Arc::new(Mutex::new(0));

    // 참조 카운터가 증가될 뿐이며 내용은 클론되지 않음
    let lock1 = lock0.clone();

    // 스레드 생성. 클로저 내 변수로 이동
    let th0 = thread::spawn(move || {
        some_func(lock0);
    });

    // 스레드 생성. 클로저 내 변수로 이동
    let th1 = thread::spawn(move || {
        some_func(lock1);
    });

    // 약속
    th0.join().unwrap();
    th1.join().unwrap();
}
