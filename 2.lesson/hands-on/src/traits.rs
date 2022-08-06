// inheritance class equivalent in rust is trait

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-bytes {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-bytes {}", self)
    }
}

pub fn traits() {
    let answer: i32 = 42;
    let maybe_pi: f64 = 3.14159;
    let s1 = answer.show();
    let s2 = maybe_pi.show();

    println!("show {}", s1);
    println!("show {}", s2);
}
