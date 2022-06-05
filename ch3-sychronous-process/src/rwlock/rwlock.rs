use std::sync::RwLock;

fn main() {
    // 보호 대상 값의 초기값 10
    let lock = RwLock::new(10);
    {
        // Read 락
        // 이뮤터블 참조 획득 (RwLockReadGuard 타입으로 감싼 참조)
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }

    {
        // Write 락
        // 뮤터블한 참조 획득 (RwLockWriteGuard 타입으로 감싼 참조)
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
}
