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
//* 차용
