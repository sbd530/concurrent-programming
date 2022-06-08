/**
 * 비대칭 코루틴을 이용하면 중단된 함수를 프로그래머 측에서 자유롭게 재개할 수 있고, 중단과 재개를 스케줄링해서 실행할 수도 있다.
 * Task: 스케줄링의 대상이 되는 계산의 실행 단위인 프로세스
 * Executor: 실행 가능한 Task를 적당한 순서로 실행
 * Waker: Task를 스케줄링할 때 이용
 */
use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender}; // 통신채널을 위한 함수와 타입
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

//* Task 타입
struct Task {
    // 실행하는 코루틴
    future: Mutex<BoxFuture<'static, ()>>,
    // Executor에 스케줄링하기 위한 채널
    sender: SyncSender<Arc<Task>>,
}
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 자신을 스케줄링
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

//* Executor 타입
struct Executor {
    // 실행 큐
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}
impl Executor {
    fn new() -> Self {
        // 채널 생성. 큐의 사이즈는 최대 1024개
        let (sender, receiver) = sync_channel(1024);
        Executor {
            sender: sender.clone(),
            receiver,
        }
    }

    // 새롭게 Task를 생성하기 위한 Spawner를 작성
    fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone(),
        }
    }

    fn run(&self) {
        // 채널에서 Task를 수신하고 순서대로 실행
        while let Ok(task) = self.receiver.recv() {
            // 컨텍스트를 생성
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            // poll을 호출해서 실행
            let _ = future.as_mut().poll(&mut ctx);
        }
    }
}

//* Spawner 타입 : 단순히 실행 큐에 추가하기 위해 채널의 송수신 엔드포인트를 저장
struct Spawner {
    sender: SyncSender<Arc<Task>>,
}
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed(); // Future를 Box화
        let task = Arc::new(Task {
            // Task 생성
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });

        // 실행 큐에 인큐
        self.sender.send(task).unwrap();
    }
}

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
            state: StateHello::HELLO, // 초기 상태
        }
    }
}
impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref(); // 자신을 실행 큐에 인큐
                return Poll::Pending;
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref(); // 자신을 실행 큐에 인큐
                return Poll::Pending;
            }
            StateHello::END => {
                return Poll::Ready(());
            }
        }
    }
}

fn main() {
    let executor = Executor::new();
    executor.get_spawner().spawn(Hello::new());
}
