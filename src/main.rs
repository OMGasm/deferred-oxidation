struct Defer<F: FnMut()>(F);
impl<F: FnMut()> core::ops::Drop for Defer<F> {
    fn drop(&mut self) {
        self.0()
    }
}

macro_rules! defer {
    ($($t:tt)*) => {
        let _foo = Defer(|| { $($t)* });
    };
}

fn foo() {
    defer! {
        println!("a");
        println!("block");
        defer!(println!("why?"));
        println!("of");
        println!("code?");
    };

    println!("done foo")
}

fn main() {
    defer!(println!("deferred!"));
    foo();
    defer!(println!("deferred again!"));
    println!("Hello, world!");
}
