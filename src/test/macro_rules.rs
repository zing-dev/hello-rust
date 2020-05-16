// item：条目，例如函数、结构、模块等
// block：代码块
// stmt：语句
// pat：模式
// expr：表达式
// ty：类型
// ident：标识符
// path：路径，例如 foo、 ::std::mem::replace, transmute::<_, int>, …
// meta：元信息条目，例如 #[…]和 #![rust macro…] 属性
// tt：词条树

macro_rules! first {
    () => {};
}

#[test]
fn first() {
    first!();
}

macro_rules! hello {
    ($name:expr) => {
        println!("hello {}", $name)
    };
}
#[test]
fn hello() {
    hello!("rust");
    hello!("world");
}

macro_rules! foo {
    (x => $e:expr) => {
        println!("mode X: {}", $e)
    };
    (y => $e:expr) => {
        println!("mode Y: {}", $e)
    };
}

#[test]
fn foo() {
    foo!(y => 3);
    foo!(x => 4);
}

macro_rules! o_o {
($($x:expr; [ $( $y:expr ),* ]);*) => {&[ $($( $x + $y ),*),* ]}}

#[test]
fn o_o() {
    let a: &[i32] = o_o!(10; [1, 2, 3]; 20; [4, 5, 6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);
}

macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

#[test]
fn five_times() {
    assert_eq!(25, five_times!(2 + 3));
}
macro_rules! log {
    ($msg:expr) => {{
        let state: i32 = 10;
        if state > 0 {
            println!("log({}): {}", state, $msg);
        }
    }};
}

#[test]
fn log() {
    let state: &str = "reticulating splines";
    log!(state);
}

macro_rules! foo2 {
    ($v:ident) => {
        let $v = 3;
    };
}

#[test]
fn foo2() {
    foo2!(x);
    println!("{}", x);
    println!("{}", x + 1);
}
