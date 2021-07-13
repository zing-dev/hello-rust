# 类型系统

编译期进行类型检查的语言属于**静态类型**,运行期进行类型检查的语言属于**动态类型**.如果一门语言不允许类型的自动隐式转换,在强制转换前不同类型无法进行计算,则该语言属于强类型,反之则属于弱类型

- 在编译期就能检查出数组是否越界访问
- 自动推导
- 类型安全的语言
- 显式静态类型

## 类型系统与多态性

如果一个类型系统允许一段代码在不同的上下文中具有不同的类型，这样的类型系统就叫作多态类型系统

- 参数化多态`Parametric polymorphism`
- `Ad-hoc`多态`Ad-hoc polymorphism`
- 子类型多态`Subtype polymorphism`

- 静多态`Static Polymorphism`
- 动多态`Dynamic Polymorphism`

`静多态`发生在编译期，`动多态`发生在运行时。`参数化多态`和`Ad-hoc多态`一般是`静多态`， `子类型多态`一般是`动多态`。`静多态`牺牲灵活性获取性能，`动多态`牺牲性能获取灵活性。`动多态`在运行时需要查表，
占用较多空间，所以一般情况下都使用`静多态`。Rust语言同时支持静多态和动多态，静多态就是一种零成本抽象。

**参数化多态实际就是指泛型**

**Ad-hoc 多态也叫特定多态,是指同一种行为定义,在不同的上下文中会响应不同的行为实现**

**Rust中的类型系统目前只支持参数化多态和Ad-hoc多态，也就是`泛型`和`trait`**

### 类型大小

编译器需要事先知道类型的大小，才能分配合理的内存

- 编译期可确定大小的类型`Sized Type`,原生整数类型
- 动态大小的类型`Dynamic Sized Type, DST`,`str`类型的字符串字面量
- 零大小类型`Zero Sized Type, ZST` 单元类型,单元结构体
- 底类型`Bottom Type` 没有值,是其他任意类型的子类型。

对于编译器来说,`str`类型的大小是无法确定的,字符串切片`&str`是`str`的引用，它由指针和长度信息组成,`&str`存储于**栈**上,`str`字符串序列存储于**堆**上,`&str`由**指针和长度**
信息组成,指针是固定大小的,存储的是`str`字符串序列的起始地址,长度信息是固定大小的整数.`&str`就可确定类型大小,编译器就可以正确地为其分配栈内存空间,`str`也会在运行时在堆上开辟内存空间。

字符串字面量`str`,通过`as_ptr()`和`len()`,获取该字符串字面量存储的地址和长度信息。包含动态大小类型地址信息和携带了长度信息的指针,叫作胖指针(Fat Pointer)，`&str` 是胖指针

**单元类型和单元结构体大小为零,由单元类型组成的数组大小也为零.`ZST`类型的特点是,它们的值就是其本身,运行时并不占用内存空间**

### 类型推导

**只能在局部范围内进行类型推导**,标注类型`::<>`的形式就叫作`turbofish`操作符

## 泛型

泛型`Generic`是一种参数化多态。使用泛型可以编写更为抽象的代码，泛型就是把一个泛化的类型作为参数，单个类型就可以抽象化为一簇类型

```rust
struct Point<T> {
    x: T,
    y: T
}
```

结构体名称旁边的<T>叫作泛型声明。泛型只有声明后才可以使用

Rust中的泛型属于静多态,它是一种编译期多态.在编译期,不管是泛型枚举,还是泛型函数和泛型结构体,都会被单态化(Monomorphization)
.单态化是编译器进行静态分发的一种策略,单态化意味着编译器要将一个泛型函数生成两个具体类型对应的函数

泛型及单态化是Rust的最重要的两个功能.单态化静态分发的好处是性能好,没有运行时开销,缺点是容易造成编译后生成的二进制文件膨胀

## `trait`

- 接口抽象 接口是对类型行为的统一约束。
- 泛型约束 泛型的行为被`trait`限定在更有限的范围内。
- 抽象类型 在运行时作为一种间接的抽象类型去使用， 动态地分发给具体的类型。
- 标签trait 对类型的约束，可以直接作为一种`标签`使用。

### 接口抽象

- 接口中可以定义方法，并支持默认实现。
- 接口中不能实现另一个接口，但是接口之间可以继承。
- 同一个接口可以同时被多个类型实现， 但不能被同一个类型实现多次。
- 使用`impl`关键字为类型实现接口方法。
- 使用`trait`关键字来定义接口。

```rust
// 关联类型
pub mod r#trait {
    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // Notice that the implementation uses the associated type `Output`.
    impl<T: Add<Output=T>> Add for Point<T> {
        // 关联类型Output 必须指定具体类型
        type Output = Self;
        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }


    // Rust遵循一条重要的规则：孤儿规则(Orphan Rule) 
    // 如果要实现某个trait,该trait和要实现该trait的类型至少有一个要在当前crate中定义
    trait MyAdd<RHS = Self> {
        type Output;
        fn my_add(self, rhs: RHS) -> Self::Output;
    }

    impl MyAdd<u64> for u32 {
        type Output = u64;
        fn my_add(self, rhs: u64) -> Self::Output {
            self as u64 + rhs
        }
    }

    // trait继承
    trait Page {
        fn set_page(&mut self, p: i32) {
            println!("page default:{}", p)
        }
    }

    trait PerPage {
        fn set_per_page(&mut self, p: i32) {
            println!("per page default:{}", p)
        }
    }

    trait Paginate: Page + PerPage {
        fn set_skip_page(&mut self, num: i32) {
            println!("skip page: {}", num)
        }
    }

    struct MyPage {
        page: i32,
    }

    impl Page for MyPage {}

    impl PerPage for MyPage {}

    //impl Paginate for MyPage {}

    impl<T: Page + PerPage> Paginate for T {}

    #[test]
    fn run() {
        println!("{:?}", Point { x: 1, y: 2 } + Point { x: 11, y: 12 });
        println!("{:?}", 1.my_add(2));

        let mut p = MyPage { page: 10 };
        p.set_page(1);
        p.set_per_page(2);
        p.set_skip_page(20)
    }
}

```

### 泛型约束

使用`trait`对泛型进行约束，叫作`trait`限定(`trait Bound`)

```rust
fn generic<T: MyTrait + MyOtherTrait + SomeStandardTrait>(t: T) {}

fn foo<T , K, R> (a: T , b : K, c : R) where T: A , K : B+C , R: D { . . }
```

需要一个类型T,并且该类型T必须实现MyTrait,MyOtherTrait和SomeStandardTrait 中定义的全部方法,才能使用该泛型函数。

**类型可以看作具有相同属性值的集合**

**`trait`也是一种类型，是一种方法集合，或者说，是一种行为的集合**

### 抽象约束