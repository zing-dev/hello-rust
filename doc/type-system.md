# 类型系统

编译期进行类型检查的语言属于**静态类型**,运行期进行类型检查的语言属于**动态类型**.如果一门语言不允许类型的自动隐式转换,在强制转换前不同类型无法进行计算,则该语言属于强类型,反之则属于弱类型

- 在编译期就能检查出数组是否越界访问
- 自动推导
- 类型安全的语言
- 显式静态类型

## 类型系统与多态性

如果一个类型系统允许一段代码在不同的上下文中具有不同的类型,这样的类型系统就叫作多态类型系统

- 参数化多态`Parametric polymorphism`
- `Ad-hoc`多态`Ad-hoc polymorphism`
- 子类型多态`Subtype polymorphism`

- 静多态`Static Polymorphism`
- 动多态`Dynamic Polymorphism`

`静多态`发生在编译期,`动多态`发生在运行时.`参数化多态`和`Ad-hoc多态`一般是`静多态`, `子类型多态`一般是`动多态`.`静多态`牺牲灵活性获取性能,`动多态`牺牲性能获取灵活性.`动多态`在运行时需要查表,
占用较多空间,所以一般情况下都使用`静多态`.Rust语言同时支持静多态和动多态,静多态就是一种零成本抽象.

**参数化多态实际就是指泛型**

**Ad-hoc 多态也叫特定多态,是指同一种行为定义,在不同的上下文中会响应不同的行为实现**

**Rust中的类型系统目前只支持参数化多态和Ad-hoc多态,也就是`泛型`和`trait`**

### 类型大小

编译器需要事先知道类型的大小,才能分配合理的内存

- 编译期可确定大小的类型`Sized Type`,原生整数类型
- 动态大小的类型`Dynamic Sized Type, DST`,`str`类型的字符串字面量
- 零大小类型`Zero Sized Type, ZST` 单元类型,单元结构体
- 底类型`Bottom Type` 没有值,是其他任意类型的子类型.

对于编译器来说,`str`类型的大小是无法确定的,字符串切片`&str`是`str`的引用,它由指针和长度信息组成,`&str`存储于**栈**上,`str`字符串序列存储于**堆**上,`&str`由**指针和长度**
信息组成,指针是固定大小的,存储的是`str`字符串序列的起始地址,长度信息是固定大小的整数.`&str`就可确定类型大小,编译器就可以正确地为其分配栈内存空间,`str`也会在运行时在堆上开辟内存空间.

字符串字面量`str`,通过`as_ptr()`和`len()`,获取该字符串字面量存储的地址和长度信息.包含动态大小类型地址信息和携带了长度信息的指针,叫作胖指针(Fat Pointer),`&str` 是胖指针

**单元类型和单元结构体大小为零,由单元类型组成的数组大小也为零.`ZST`类型的特点是,它们的值就是其本身,运行时并不占用内存空间**

### 类型推导

**只能在局部范围内进行类型推导**,标注类型`::<>`的形式就叫作`turbofish`操作符

## 泛型

泛型`Generic`是一种参数化多态.使用泛型可以编写更为抽象的代码,泛型就是把一个泛化的类型作为参数,单个类型就可以抽象化为一簇类型

```rust
struct Point<T> {
    x: T,
    y: T
}
```

结构体名称旁边的<T>叫作泛型声明.泛型只有声明后才可以使用

Rust中的泛型属于静多态,它是一种编译期多态.在编译期,不管是泛型枚举,还是泛型函数和泛型结构体,都会被单态化(Monomorphization)
.单态化是编译器进行静态分发的一种策略,单态化意味着编译器要将一个泛型函数生成两个具体类型对应的函数

泛型及单态化是Rust的最重要的两个功能.单态化静态分发的好处是性能好,没有运行时开销,缺点是容易造成编译后生成的二进制文件膨胀

## `trait`

- 接口抽象 接口是对类型行为的统一约束.
- 泛型约束 泛型的行为被`trait`限定在更有限的范围内.
- 抽象类型 在运行时作为一种间接的抽象类型去使用, 动态地分发给具体的类型.
- 标签trait 对类型的约束,可以直接作为一种`标签`使用.

### 接口抽象

- 接口中可以定义方法,并支持默认实现.
- 接口中不能实现另一个接口,但是接口之间可以继承.
- 同一个接口可以同时被多个类型实现, 但不能被同一个类型实现多次.
- 使用`impl`关键字为类型实现接口方法.
- 使用`trait`关键字来定义接口.

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

使用`trait`对泛型进行约束,叫作`trait`限定(`trait Bound`)

```rust
fn generic<T: MyTrait + MyOtherTrait + SomeStandardTrait>(t: T) {}

fn foo<T, K, R>(a: T, b: K, c: R) where T: A, K: B + C, R: D { .. }
```

需要一个类型T,并且该类型T必须实现MyTrait,MyOtherTrait和SomeStandardTrait 中定义的全部方法,才能使用该泛型函数.

**类型可以看作具有相同属性值的集合**

**`trait`也是一种类型,是一种方法集合,或者说,是一种行为的集合**

**Rust编程的哲学是组合优于继承**,Rust 并不提供类型层面上的继承,Rust中所有的类型都是独立存在的,所以Rust中的类型可以看作语言允许的最小集合,不能再包含其他子集.而trait限定可以对这些类型集合进行组合,也就是求交集.

### 抽象类型

`trait`还可以用作抽象类型`Abstract Type`.抽象类型属于类型系统的一种,也叫作存在类型`Existential Type`.相对于具体类型而言,抽象类型无法直接实例化,它的每个实例都是具体类型的实例.
对于抽象类型而言,编译器可能无法确定其确切的功能和所占的空间大小.目前有两种方法来处理抽象类型： `trait对象`和`impl Trait` .

#### `trait对象`

在泛型中使用`trait`限定,可以将任意类型的范围根据类型的行为限定到更精确可控的范围内.从这个角度出发,也可以将共同拥有相同行为的类型集合抽象为一个类型,这就是`trait对象trait Object`
.`对象`这个词来自面向对象编程语言,因为`trait`对象是对具有相同行为的一组具体类型的抽象,等价于面向对象中一个封装了行为的对象,所以称其为`trait对象`.

```rust
/// 抽象类型
pub mod abstract_type {
    #[derive(Debug)]
    struct Foo;

    trait Bar {
        fn baz(&self);
    }

    impl Bar for Foo {
        fn baz(&self) {
            println!("{:?}", self);
        }
    }

    //带trait限定的泛型函数
    fn static_dispatch<T>(t: &T)
        where
            T: Bar,
    {
        t.baz();
    }

    //trait对象
    fn dynamic_dispatch(t: &dyn Bar) {
        t.baz();
    }

    #[test]
    fn trait_object() {
        let foo = Foo;
        static_dispatch(&foo);
        dynamic_dispatch(&foo);
    }
}
```

`trait`本身也是一种类型,但它的类型大小在编译期是无法确定的,所以**`trait对象`必须使用指针**.可以利用引用操作符`&或Box<T>`来制造一个`trait对象`

并不是每个`trait`都可以作为`trait对象`被使用,这依旧和类型大小是否确定有关系.每个`trait`都包含一个**隐式的类型参数`Self`**,代表实现该trait的类型.`Self`
默认有一个隐式的`trait限定 ?Sized` , 形如`<Self: ?Sized>`, `?Sized trait`包括了**所有的动态大小类型和所有可确定大小的类型**
.Rust中大部分类型都默认是可确定大小的类型,也就是`<T: Sized>`,这也是泛型代码可以正常编译的原因.

当`trait对象`在运行期进行动态分发时,也必须确定大小,否则无法为其正确分配内存空间.所以必须同时满足以下两条规则的`trait`才可以作为`trait对象`使用.

- `trait`的`Self`类型参数不能被限定为`Sized`
- `trait`中所有的方法都必须是对象安全的

当把`trait`当作对象使用时,其内部类型就默认为`Unsize类型`, 也就是`动态大小类型`,只是将其置于编译期可确定大小的胖指针背后,以供运行时动态调用.对象安全的本质就是为了让`trait`
对象可以安全地调用相应的方法.如果给`trait`加上`Self: Sized`限定,在动态调用`trait对象`的过程中,如果碰到了`Unsize类型`, 在调用相应方法时,可能引发段错误.所以,就无法将其作为`trait 对象`
.反过来,当不希望`trait`作为`trait对象`时,可以使用`Self: Sized`进行限定.

对象安全的方法必须满足以下三点之一

- 方法受`Self: Sized`约束.
- 方法签名同时满足以下三点
    - 必须不包含任何泛型参数.如果包含泛型,`trait对象`在虚表(Vtable)中查找方法时将不确定该调用哪个方法
    - 第一个参数必须为`Self`类型或可以`解引用为Self`的类型(必须有接收者,比如`self &self &mut self self: Box<Self>`, 没有接收者的方法对`trait对象`来说毫无意义)
    - `Self`不能出现在除第一个参数之外的地方, 包括返回值中.这是因为如果出现`Self`, 那就意味着`Self self &self &mut self`的类型相匹配.但是对于`trait对象`
      来说,根本无法做到保证类型匹配,因此,这种情况下的方法是对象不安全的.

**没有额外`Self类型参数`的非泛型成员方法.**

```rust

//对象不安全的trait
trait Foo {
    fn bad<T>(&self, x: T);
    fn new() - > Self;
}
```

```rust
 //对象安全的trait,将不安全的方法拆分出去
trait Foo {
    fn bad<T>(&self, x: T);
}
```

```rust

trait Foo: Bar {
    fn new() - > Self;
}
```

```rust
// 对象安全的trait ,使用where 子句
trait Foo {
    fn bad<T>(&self, x: T);
    fn new() - > Self where Self: Sized;)
}
```

#### `impl trait`

如果说`trait对象`是装箱抽象类型`Boxed Abstract Type`的话,那么`impl Trait`就是拆箱抽象类型`Unboxed Abstract Type`.`装箱`和`拆箱`是业界的抽象俗话, 其中**`装箱`
代表将值托管到堆内存,而`拆箱`则是在栈内存中生成新的值**

目前`impl Trait`只可以在 **输入的参敬** 和 **返回值** 这两个位置使用

```rust

pub mod abstract_type {
    use std::fmt::Debug;

    trait Fly {
        fn fly(&self) -> bool;
    }

    #[derive(Debug)]
    struct Duck;

    #[derive(Debug)]
    struct Pig;

    impl Fly for Duck {
        fn fly(&self) -> bool {
            true
        }
    }

    impl Fly for Pig {
        fn fly(&self) -> bool {
            false
        }
    }

    fn fly_static(s: impl Fly + Debug) -> bool {
        s.fly()
    }

    //将impl Trait语法用于返回值位置的时候,实际上等价于给返回类型增加一种trait限定范围
    fn can_fly(s: impl Fly + Debug) -> impl Fly {
        if s.fly() {
            println!("{:?} can fly", s)
        } else {
            println!("{:?} can't fly", s)
        }
        s
    }

    #[test]
    fn impl_trait() {
        let pig = Pig;
        let duck = Duck;
        assert_eq!(fly_static(pig), false);
        assert_eq!(fly_static(duck), true);
        can_fly(Pig);
        can_fly(Duck);
    }
}

```

- `impl Trait`代表静态分发
- `dyn Trait`代表动态分发

#### 标签`trait`

- `Sized trait` ,用来标识编译期可确定大小的类型.
- `Unsize trait` ,目前该`trait`为实验特性,用于标识动态大小类型`DST`.
- `Copy trait` ,用来标识可以按位复制其值的类型.
- `Send trait` ,用来标识可以跨线程安全通信的类型.
- `Sync trait`,用来标识可以在线程间安全共享引用的类型.

##### **Sized trait**

**编译器用它来识别可以在编译期确定大小的类型**

```rust
// 属性lang 表示Sized trait 供Rust语言本身使用,
// 声明为sized,称为语言项Lang Item
#[lang = "sized"]
pub trait Sized {}
```

`Sized`标识的是在编译期可确定大小的类型,而`Unsize`标识的是动态大小类型,在编译期无法确定其大小,目前Rust中的动态类型有`trait`和`[T］`,其中`[T]`代表一定数量的`T`在内存
中依次排列,但不知道具体的数量,所以它的大小是未知的,用`Unsize`来标记.比如`str`字符串和定长数组`[T;N]` .`[T]`其实是`[T;N]` 的特例, 当`N`的大小未知时就是`[T]` .

`?Sized` 标识的类型包含了`Sized`和`Unsize`所标识的两种类型

- 只可以通过胖指针来操作`Unsize`类型,比如`&[T]` 或`&Trait`
- 变量、参数和枚举变量不能使用动态大小类型.
- 结构体中只有最后一个字段可以使用动态大小类型,其他字段不可以使用.

##### **Copy trait**

`Copy trait`用来标记可以按位复制其值的类型,按位复制等价于C语言中的`memcpy`

```rust
#[lang = "copy"]
pub trait Copy: Clone {}

pub trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
```

`Copy`的行为是一个隐式的行为,开发者不能重载`Copy`行为,它永远都是一个简单的位复制.

并非所有类型都可以实现`Copy trait`.对于自定义类型来说,必须让所有的成员都实现了`Copy trait`,这个类型才有资格实现`Copy trait`.如果是数组类型,且其内部元素都是`Copy`类型,则数组本身就是`Copy`
类型；如果是元组类型,且其内部元素都是`Copy`类型,则该元组会自动实现`Copy` ,如果是结构体或枚举类型,只有当每个内部成员都实现`Copy`,它才可以实现`Copy`

##### **Send trait 和 Sync trait**

- 实现了`Send`的类型, 可以安全地在线程间传递值,也就是说可以跨线程传递所有权.
- 实现了`Sync`的类型, 可以跨线程安全地传递共享(不可变)引用.

可以安全跨线程传递的值和引用,以及不可以跨线程传递的值和引用,配合所有权机制,`Rust`能够在编译期就检查出数据竞争的隐患,

```rust
#[lang = "send"]
pub unsafe trait Send {}

#[lang = "sync"]
pub unsafe trait Sync {}

```

```rust

mod send_sync {
    use std::rc::Rc;
    use std::thread;

    ///多线程共享不可变变量
    #[test]
    fn share() {
        let x = vec![1, 2, 3, 4];
        let s = thread::spawn(|| x);
        s.join().unwrap();
    }

    /// 多线程之间共享可变变量
    #[test]
    fn share_mut() {
        let mut x = vec![1, 2, 3, 4];
        thread::spawn(move || x.push(1));
        // x.push(2)
    }

    /// 在多线程之间传递没有实现Send和Sync的类型
    #[test]
    fn share_rc() {
        //Re 是用于引用计数的智能指针
        let x = Rc::new(vec![1, 2, 3, 4]);
        thread::spawn(move || x[1]);
        //`Rc<Vec<i32>>` cannot be sent between threads safely
    }
}
```

### 类型转换

- 隐式(强制)类型转换`Implicit Type Conversion`
- 显示类型转换`Explicit Type Conversion`

#### Deref解引用

**如果一个类型`T`实现了`Deref<Target=U>`, 则该类型`T`的引用或智能指针在应用的时候会被自动转换为类型`U`**

```rust
pub trait Deref {
    /// 解引用之后的目标类型
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

**手动解引用**

- `match x.deref()`, 直接调用`deref`方法, 需要`use std::ops::Deref`.
- `match x.as_ref()`, `String`类型提供了`as_ref`方法来返回一个`&str`类似,该方法定义于`AsRef trait`中.
- `match x.borrow()`,方法`borrow`定义于`Borrow trait`中,行为和`AsRef`类型一样.需要`use std::borrow::Borrow` .
- `match &*x`,使用`解引用`操作符,将`String`转换为`str`,然后再用`引用`操作符转为`&str`
- `match &x[..]`,String 类型的index 操作可以返回`&str`类型.

**无歧义完全限定语法**

```rust
mod limit {
    struct S(i32);

    trait A {
        fn test(&self, i: i32);
    }

    trait B {
        fn test(&self, i: i32);
    }

    impl A for S {
        fn test(&self, i: i32) {
            println!("from A: {:?}", i);
        }
    }

    impl B for S {
        fn test(&self, i: i32) {
            println!("from B: {:?}", i);
        }
    }

    #[test]
    fn test() {
        let s = S(1);
        // 直接当作trait静态函数调用
        A::test(&s, 1);
        B::test(&s, 1);
        // 使用as操作符
        <S as A>::test(&s, 1);
        <S as B>::test(&s, 1);
    }
}
```

**类型和子类型互相转换**

```rust
 mod static_as {
    #[test]
    fn test() {
        let a: &'static str = "hello";
        let b: &str = a as &str;
        let c: &'static str = b as &'static str;

        println!("{} {} {}", a, b, c)
    }
}

```

#### From和Into

`From`和`Into`是定义于`std::convert`模块中的两个`trait`.它们定义了`from`和`into`两个方法,这两个方法互为反操作

类型`U`实现了`From<T>`,则`T`类型实例调用`into`方法就可以转换为类型`U`

```rust

mod from_into {
    #[derive(Debug)]
    struct Person {
        name: String,
    }

    impl Person {
        fn new<T: Into<String>>(name: T) -> Person {
            Person { name: name.into() }
        }
    }

    #[test]
    fn test_into() {
        let p1 = Person::new("zing");
        let p2 = Person::new("zing".to_string());
        println!("{:?} {:?}", p1, p2);
    }
}
```