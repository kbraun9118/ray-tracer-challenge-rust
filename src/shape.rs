pub trait Shape {}

#[derive(Debug)]
struct Circle {
    name: String,
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}

impl Shape for Circle {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circles_compare_memory() {
        let a = Circle {
            name: String::from("hello"),
        };
        let b = Circle {
            name: String::from("hello"),
        };
        let c = &a;

        assert_ne!(a, b);
        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_eq!(&a, c)
    }
}
