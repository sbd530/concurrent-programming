use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

//* 코루틴 : 함수를 임의의 시점에 중단하고, 중단한 위치에서 함수를 재개할 수 있다. COBOL과 ALGOL 언어에 처음 적용됨.

struct Hello {
    state: StateHello,
}
// 상태
enum StateHello {
    HELLO,
    WORLD,
    END,
}
impl Hello {
    fn new() -> Self {
        Hello {
            // 생성시 초기상태 HELLO
            state: StateHello::HELLO,
        }
    }
}

impl Future for Hello {
    type Output = ();

    // 실행 함수 : js의 제너레이터처럼 함수 내부의 yield를 순차적 실행하는 것처럼 동작한다.
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                println!("Hello, ");
                // WORLD 상태로 전이
                (*self).state = StateHello::WORLD;
                // 다시 호출 가능
                Poll::Pending
            }
            StateHello::WORLD => {
                println!("World!");
                // END 상태로 전이
                (*self).state = StateHello::END;
                // 다시 호출 가능
                Poll::Pending
            }
            StateHello::END => {
                // 종료
                Poll::Ready(())
            }
        }
    }
}

// 실행 단위
struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}
impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

// 아무것도 하지 않음
impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

fn main() {
    // 초기화
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    // 정지와 재개의 반복
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
}
