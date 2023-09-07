#[macro_use]
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
mod My_Rc;
use crate::My_Rc::MyRc;
// 实现hash_map宏
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// 简易栈结构
#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }

    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}
fn main() {
    let five = MyRc::new(5);
    let five1 = five.clone();
    println!("five = {}", five);
    println!("five1 = {}", five1);
    println!("{}", five.get_strong());
    drop(five1);    // 释放five1,不会打印dropped!
    println!("{}", five.get_strong());
    drop(five);     // 释放five,会打印dropped!
    // println!("{}", five.inner().strong.get());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_1() {
        let map = hash_map! {
            "one" => 1,
            "two" => 2
        };
        assert_eq!(map["one"], 1);
        assert_eq!(map["two"], 2);
    }
    #[test]
    #[should_panic]
    fn test_marco_2() {
        let map = hash_map! {
            "one" => 1,
            "two" => 2,
            "three" => 3
        };
        assert_eq!(map["one"], 1);
        assert_eq!(map["two"], 3);
    }

    #[test]
    fn test_MyRc() {
        let five = MyRc::new(5);
        let five1 = MyRc::clone(&five);
        let five2 = five.clone();
        let five3 = five.clone();
        assert_eq!(5, *five);
        assert_eq!(5, *five1);
        assert_eq!(5, *five2);
        assert_eq!(5, *five3);
        assert_eq!(4, five.get_strong());
        drop(five3);
        assert_eq!(3, five.get_strong());
        drop(five2);
        // 检查five1和five2的计数是否相同
        assert_eq!(2, five.get_strong());
        assert_eq!(2, five1.get_strong());
    }
}