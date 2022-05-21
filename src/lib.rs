#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

pub fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

pub fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

pub fn divide(x: i32, y: i32) -> i32 {
    return x / y;
}
