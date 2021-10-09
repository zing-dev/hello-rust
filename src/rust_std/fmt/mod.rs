#[test]
fn positional_parameters_test() {
    println!("{}", format!("{1} {} {0} {}", 1, 2)); // 2 1 1 2
    println!("{}", format!("{1} {} {0} {} {}", 0, 1, 2)); // 1 0 0 1 2
    println!("{} {} {0} {} {0}", 1, 2, 3); // 1 2 1 3 1
}

#[test]
fn named_parameters_test() {
    println!("{argument}", argument = "test");   // => "test"
    println!("{name} {}", 1, name = 2);          // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3);  // => "a 3 b"
    println!("{a} {c} {b} {} {1} {} {}", a = "a", b = 'b', c = 3);
}

#[test]
fn width_test() {
    // All of these print "Hello x    !"
    println!("Hello {:#5}!", "x");
    println!("Hello {:+5}!", "x");
    println!("Hello {:-5}!", "x");
    println!("Hello {0:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
}

// [fill]< - 在参数列中左对齐
// [fill]^ - 在参数列中居中对齐
// [fill]> - 在参数列中右对齐
#[test]
fn fill_alignment_test() {
    assert_eq!(format!("Hello {:<5}!", "x"), format!("Hello {:5}!", "x"));
    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    println!("Hello {:.<5}!", "x");
    println!("Hello {:卧<5}!", "x");
    println!("Hello {:😂<5}!", "x");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
}

// 数值进制格式化
// :格式化
//  :+ 正数前添加+
//  :- 负数前添加-
//  :x 十六进制小写
//  :X 十六进制大写
//  :#x 前置0x十六进制小写
//  :#X 前置0x十六进制大写
//  :b 二进制
//  :#b 前置0b二进制
//  :o 八进制
//  :#o 前置0o八进制
//  :#010o 前置0o八进制总长度为10,不足补0
//  :#10o 前置0o八进制总长度为10,不足补空格
#[test]
fn sign_test() {
    println!("{:+}", 10); // +10
    println!("{:-}", 10); // 10
    println!("{:+}", -10); // -10
    println!("{:-}", -10); // -10
    assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");

    assert_eq!(format!("{:#x}!", 27), "0x1b!");
    println!("{}", 0xff); // 255
    println!("{:x}", 0xff); // ff
    println!("{:X}", 0xff); // FF
    println!("{:#x}", 0xff); // 0xff
    println!("{:#X}", 0xff); // 0xFF
    println!("{:b}", 0xff); // 11111111
    println!("{:#b}", 0xff); // 0b11111111
    println!("{:o}", 0xff); // 377
    println!("{:#o}", 0xff); // 0o377
    println!("{:#010o}", 0xff); // 0o00000377
    println!("{:#10o}", 0xff); //      0o377
    println!("{:+#10o}", 0xff); //     +0o377
    println!("{:-#08o}", 0xff); //0o000377

    assert_eq!(format!("Hello {:05}!", 5), "Hello 00005!");
    assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!");
    assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
}

// 数值精度
// 当前要格式的数值:.保留的精度位数(从前往右截取),位数不够则补充0
#[test]
fn precision_test() {
    // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
    println!("Hello {0} is {1:.5}", "x", 0.01);

    // Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

    // Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {second of next two args (0.01) with precision
    //                          specified in first of next two args (5)}
    println!("Hello {} is {:.*}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {arg 2 (0.01) with precision
    //                          specified in its predecessor (5)}
    println!("Hello {} is {2:.*}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {arg "number" (0.01) with precision specified
    //                          in arg "prec" (5)}
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    println!("{}, `{name:.*}` has 3 fractional digits", "Hello", 3, name = 1234.56);
    println!("{}, `{name:.*}` has 3 characters", "Hello", 3, name = "1234.56");
    println!("{}, `{name:>8.*}` has 3 right-aligned characters", "Hello", 3, name = "1234.56");
    println!("{}, `{name:<8.*}` has 3 right-aligned characters", "Hello", 3, name = "1234.56");
}

pub mod display {
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Debug)]
    struct Vector2D {
        x: isize,
        y: isize,
    }

    impl fmt::Display for Vector2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // The `f` value implements the `Write` trait, which is what the
            // write! macro is expecting. Note that this formatting ignores the
            // various flags provided to format strings.
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // Different traits allow different forms of output of a type. The meaning
    // of this format is to print the magnitude of a vector.
    impl fmt::Binary for Vector2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let magnitude = (self.x * self.x + self.y * self.y) as f64;
            let magnitude = magnitude.sqrt();

            // Respect the formatting flags by using the helper method
            // `pad_integral` on the Formatter object. See the method
            // documentation for details, and the function `pad` can be used
            // to pad strings.
            let decimals = f.precision().unwrap_or(3);
            let string = format!("{:.*}", decimals, magnitude);
            f.pad_integral(true, "", &string)
        }
    }

    impl fmt::Octal for Vector2D {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let magnitude = (self.x * self.x + self.y * self.y) as f64;
            let magnitude = magnitude.sqrt();
            let decimals = f.precision().unwrap_or(2);
            let string = format!("{:.*}", decimals, magnitude);
            f.pad_integral(true, "", &string)
        }
    }

    #[test]
    fn test() {
        let v = Vector2D { x: 3, y: 4 };

        println!("{}", v);       // => "(3, 4)"
        println!("{:?}", v);     // => "Vector2D {x: 3, y:4}"
        println!("{:10.3b}", v); // => "     5.000"
        println!("{:05o}", v); // => "     5.000"
    }
}