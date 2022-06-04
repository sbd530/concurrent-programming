fn main() {
    println!("call let_example() = {}", let_example());
    my_func1();
    println!("is_even(3) = {}", is_even(3));
    print_pred(0);
    print_pred(3);
    even_odd_for();
    even_odd_loop();
    my_func2();
    my_func3();
    my_func4();
    my_func5();
    my_func6();
    my_func7();
    my_func8();
    my_func9();
    my_func10();
    my_func11();
    my_func12();
}
//* 변수
fn let_example() -> u32 {
    let x = 100; // immutable
    let mut y = 20; // mutable
    let z: u32 = 5; // explicit type
    let w;
    y *= x + z;
    w = 8;
    y + w // return (세미콜론 x)
}
//* 함수 호출
fn hello(v: u32) {
    println!("Hello!: v={}", v);
}
fn add(x: u32, y: u32) -> u32 {
    // -> 반환 타입
    x + y
}
fn my_func1() {
    let n = add(100, 5);
    hello(n);
}
//* if
fn is_even(v: u32) -> bool {
    if v % 2 == 0 {
        true
    } else {
        false
    }
}
//* match
fn pred(v: u32) -> Option<u32> {
    // Option type : Node or Some wrapping
    if v == 0 {
        None
    } else {
        Some(v - 1)
    }
}
fn print_pred(v: u32) {
    match pred(v) {
        Some(w) => {
            println!("pred({}) = {}", v, w);
        }
        None => {
            println!("pred({}) is undefined", v);
        }
    }
}
//* for
fn even_odd_for() {
    for n in 0..10 {
        println!("{} is {}", n, if is_even(n) { "even" } else { "odd" });
    }
}
//* loop
fn even_odd_loop() {
    let mut n = 0;
    loop {
        println!("{} is {}", n, if is_even(n) { "even" } else { "odd" });
        n += 1;
        if n >= 10 {
            break;
        }
    }
}
//* 참조 취득과 참조 제외
fn mul(x: &mut u64, y: &u64) {
    *x *= *x * *y; // (*x) *= (*x) * ((*x) * (*y))
}
fn my_func2() {
    let mut n = 10;
    let m = 20;
    println!("n = {}, m = {}", n, m); // n = 10, m = 20
    mul(&mut n, &m);
    println!("n = {}, m = {}", n, m); // n = 2000, m = 20
}
//* 함수 포인터 ( fn(u64) -> u64 )
fn app_n(f: fn(u64) -> u64, mut n: u64, mut x: u64) -> u64 {
    loop {
        if n == 0 {
            return x;
        }
        x = f(x);
        n -= 1;
    }
}
fn mul2(x: u64) -> u64 {
    x * 2
}
fn my_func3() {
    println!("app_n(mul2, 4, 3) = {}", app_n(mul2, 4, 3));
}
//* 클로저
fn mul_x(x: u64) -> Box<dyn Fn(u64) -> u64> {
    // dyn : trait의 작동이 동적으로 결정되는 것을 나타낸다.
    // Box : 힙상에 데이터 배치. 스코프에서 벗어나면 확보된 데이터가 자동으로 파기된다.
    // 클로저 정의 : "|변수1, 변수2, ...| 식" 변수가 클로저의 인수, 식이 클로저의 본체
    // move : 소유권 이동
    Box::new(move |y| x * y)
}
fn my_func4() {
    // mul_x 을 호출하면서 |y| 3 * y 라는 클로저를 힙상에 생성한다.
    let f = mul_x(3);
    println!("f(5) = {}", f(5));
}
//* 소유권
struct Apple {} // 사과
struct Gold {} // 돈
struct FullStomach {} // 포만감

// 사과를 팔아 돈을 얻는 함수
fn get_gold(a: Apple) -> Gold {
    Gold {}
}
// 사과를 먹고 포만감을 얻는 함수
fn get_full_stomach(a: Apple) -> FullStomach {
    FullStomach {}
}
fn my_func5() {
    // 사과 1개
    let a = Apple {};
    // 사과를 팔아 돈을 번다
    let g = get_gold(a);
    // 사과를 팔아 돈을 얻었으므로 컴파일 에러 발생
    // let s = get_full_stomach(a); -> use of moved value: `a`. value used here after moverustc(E0382)
}
//* 라이프타임
struct Foo {
    val: u32,
}
// ' 는 원시타입에는 쓸 수 없다.
fn add2<'a>(x: &'a Foo, y: &'a Foo) -> u32 {
    x.val + y.val
}
fn my_func6() {
    // x의 라이프타임은 [152, 158]
    // y의 라이프타임은 [154, 157] -> 서브타이핑으로 [154, 157]로 합쳐진다.
    let x = Foo { val: 10 };
    {
        let y = Foo { val: 20 };
        let z = add2(&x, &y);
        println!("z = {}", z);
    }
}
/*
차용
어떤 객체에 파괴적 대입을 수행할 수 있는 프로세스는 동시에 2개 이상 존재하지 않는다.
어떤 시각에 어떤 객체에 파괴적 대입을 수행할 수 있는 프로세스가 존재하는 경우, 그 시각에는 해당 객체의 읽기쓰기가 가능한 프로세스가 더이상 존재하지 않는다.
*/
fn add_val(x: Foo, y: Foo) -> (u32, Foo, Foo) {
    // x,y의 소유권 반환
    (x.val + y.val, x, y)
}
fn mul_val(x: Foo, y: Foo) -> (u32, Foo, Foo) {
    // x,y의 소유권 반환
    (x.val * y.val, x, y)
}
fn my_func7() {
    let x = Foo { val: 3 };
    let y = Foo { val: 6 };
    // 반환된 x,y의 소유권을 xn,yn에 저장
    let (a, xn, yn) = add_val(x, y);
    let (b, _, _) = mul_val(xn, yn);
    println!("a = {}, b = {}", a, b);
}
// Rust의 차용을 이해하기 위해서는 변수를 뮤터블 변수, 이뮤터블 변수, 뮤터블 참조, 이무터블 참조 4종류로 나눌 수 있다는 점과
// 각 변수의 상태 전이를 알아야한다.
// 차용의 예
fn my_func8() {
    // x는 뮤터블 변수
    let mut x = Foo { val: 10 };
    {
        // a는 뮤터블 참조
        let a = &mut x;
        println!("a.val = {}", a.val);
        // x는 '&mut 대여중' 상태이므로 에러
        // println!("x.val = {}", x.val);

        // b는 이뮤터블 참조
        let b: &Foo = a;
        // a는 '& 대여중' 상태이므로 에러
        // a.val = 20;
        println!("b.val = {}", b.val);
        // 여기서 b가 차용중인 소유권이 반환된다.

        a.val = 30;
    }

    {
        // c는 이뮤터블 참조
        let c = &x;
        println!("c.val = {}", c.val);
        println!("x.val = {}", x.val);

        // x는 '& 대여중' 상태이므로 에러
        // let d = &mut x;
        // d.val = 40;

        println!("c.val = {}", c.val);
    }

    println!("x.val = {}", x.val);
}
//* 메서드 정의
struct Vec2 {
    x: f64,
    y: f64,
}
impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
    // Vec2 인스턴스의 이뮤터블 참조를 &self 변수로 한다.
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    // self를 뮤터블 참조로 쓴다.
    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}
fn my_func9() {
    let mut v = Vec2::new(10.0, 5.0);
    println!("v.norm = {}", v.norm());
    v.set(3.8, 9.1);
    println!("v.norm = {}", v.norm());
}
//* 트레이트
// 제네릭으로 되어잇어 타입 인수를 받는다. RHS가 타입인수이고, Self가 기본타입 인수이다.
trait AddExample<RHS = Self> {
    // 트레이트 안에서 이용하는 타입을 정의한다.
    type Output;
    // 구현할 add 함수 타입을 정의한ㄷ.
    fn add(self, rhs: RHS) -> Self::Output;
}
// Add 트레이트를 표준 라이브러리에서 임포트
use std::ops::Add;
// Vec2 타입을 위한 Add 트레이트 구현
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
fn my_func10() {
    let v1 = Vec2 { x: 10.0, y: 5.0 };
    let v2 = Vec2 { x: 3.1, y: 8.7 };
    // + 연산자를 사용할수있다. v1,v2의 소유권은 이동
    let v = v1 + v2;
    println!("v.x = {}, v.y = {}", v.x, v.y);
}
//* ? 연산자와 unwrap
/*
'?'는 match와 return의 syntactic sugar 이다.

#'?' 연산자의 예
let a = get(expr)?;

# get()이 만약 Option 타입 반환한다면
let a = match get(expr) {
    Some(e) => e,
    None => return None,
};

# get()이 만약 Result 타입 반환한다면
let a = match get(expr) {
    Ok(e) => e,
    Err(e) => return Err(e),
}
*/
/*
unwrap은 Option이나 Result 타입등에 구현되어, 성공해 값을 꺼낼수 있으면 꺼내고,
꺼낼 수 없으면 panic으로 종료시키는 작동을 기술할 수 있다.

# unwrap 함수의 예
let a = get(expr).unwrap();

# get()이 만약 Option 타입 반환한다면
let a = match get(expr) {
    Some(e) => e,
    None => { panic!() },
};

# get()이 만약 Result 타입 반환한다면
let a = match get(expr) {
    Ok(e) => e,
    Err(e) => { panic!() },
}
*/
//* 스레드
// 스레드를 생성하기 위한 spawn 함수 임포트
use std::thread::spawn;
fn helloT() {
    println!("Hello World! from helloT");
}
fn my_func11() {
    // spawn 함수의 인수로 helloT 라는 함수 포인터를 전달
    // Rust의 스레드는 기본적으로 attach thread이므로 join할 필요는 없지만
    // join 함수를 이용해 스레드가 종료되기까지 대기할 수 있다.
    spawn(helloT).join();
    let h = || println!("Hello World! from Clousure");
    // 클로저로도 전달하여 스레드를 생성할 수 있다.
    spawn(h).join();
}
// 좀더 복잡한 스레드 이용의 예
fn my_func12() {
    let v = 10;
    let f = move || v * 2;
    // OK(10 * 2)를 얻는다.
    let result = spawn(f).join();
    println!("result = {:?}", result);
    // result = Ok(20)

    // 스레드가 panic인 경우 Err(패닉값)을 얻을 수 있다.
    match spawn(|| panic!("I'm panicked!")).join() {
        Ok(_) => {
            println!("successed");
        }
        Err(a) => {
            let s = a.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }
    // thread '<unnamed>' panicked at 'I'm panicked!', main.rs:338:20
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // failed: Some("I'm panicked!")
}
