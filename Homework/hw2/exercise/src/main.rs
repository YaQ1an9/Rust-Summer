use std::ops::Add;

// Exercise 1
// 实现一个Buffer<T>,Buffer只有一个成员Vec<T>
struct Buffer<T: std::ops::Add<Output = T> + Copy> {
    buffer: Vec<T>,
}
// 实现sum方法,返回buffer中所有元素的和
impl<T: Add<Output = T> + Copy> Buffer<T> {
    fn sum(&self) -> T {
        let mut sum = self.buffer[0];
        for i in 1..self.buffer.len() {
            sum = sum + self.buffer[i];
        }
        sum
    }
}

// Exercise 2
// compareString返回true表示x比y在字典序上更大
fn compareString(x: &str, y: &str) -> bool {
    let mut x = x.chars();
    let mut y = y.chars();
    loop {
        let a = x.next();
        let b = y.next();
        if a == None && b == None {
            return false;
        }
        if a == None {
            return false;
        }
        if b == None {
            return true;
        }
        if a.unwrap() > b.unwrap() {
            return true;
        }
        if a.unwrap() < b.unwrap() {
            return false;
        }
    }
}
fn main() {
    let buffer = Buffer {
        buffer: vec![1, 2, 3, 4, 5],
    };
    println!("sum: {}", buffer.sum());
    let nude = vec!['a', 'b', 'c', 'd', 'e'];
    let mut iter = nude.iter();
    // 通过闭包+迭代器生成新的Vec<char>,内容是nude中所有元素加 1
    let new_nude: Vec<char> = iter.map(|x| (*x as u8 + 1) as char).collect();
    println!("new_nude: {:?}", new_nude);


}
// 测试compareString
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compare_string() {
        assert_eq!(compareString("abc", "abd"), false);
        assert_eq!(compareString("abc", "abc"), false);
        assert_eq!(compareString("abd", "abc"), true);
    }

}