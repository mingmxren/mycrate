use std::ops::Add;

/// Add left and right together.
///
/// # Examples
///
/// ```
/// assert_eq!(5, ming_test_crate::add(2, 3));
/// ```
pub fn add<T>(left: T, right: T) -> T
where
    T: Add<Output = T>,
{
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
