pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add() {
        let res = add_one(5);
        assert_eq!(res, 6);
    }
}
