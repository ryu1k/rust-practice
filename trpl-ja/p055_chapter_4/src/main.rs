
fn p055() {
    let _x = 5;
}

fn p056() {
    let (_a, _b) = (1, 2);
    let _c: i32 = 3;
}

fn p057() {
    let mut _x : i32 = 5;
    _x = 10;
}

fn p058() {
    let _x : i32;
    // println!("x is {}", _x);
}

#[allow(dead_code)]
fn p059() {
    let x: i32 = 7;
    {
        let y: i32 = 3;
        println!("x is {}, y is {}", x, y);
    }
    // println!("x is {}, y is {}", x, y);
}

#[allow(dead_code)]
fn p060() {
    let x: i32 = 8;
    {
        println!("x is {}", x);
        let x = 12;
        println!("x is {}", x);
    }
    println!("x is {}", x);
    let x = 42;
    println!("x is {}", x);
}

#[allow(dead_code)]
fn p061() {
    #[allow(unused_assignments)]
    let mut x: i32 = 8;
    x = 12;
    let x = x;
    println!("x is {}", x);

    let y = 7;
    println!("y is {}", y);
    let y = "string";
    println!("y is {}", y);
}


fn p061_proc(x: i32)
{
    println!("value is {}", x);
}
#[allow(dead_code)]
fn p061_function() {
    fn p061_proc_inline(x: String)
    {
        println!("value is {}", x);
    }
    p061_proc(10);
    p061_proc_inline("hello".to_string());
}

fn main() {
    p055();
    p056();
    p057();
    p058();
    // p059();
    // p060();
    // p061();
    // p061_function();
}
