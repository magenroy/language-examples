fn mutate(x: &mut i32) {
    *x += 1;
}

#[derive(Debug, PartialEq, Eq)]
enum XYZ { X,Y,Z }

impl XYZ {
    fn mutate(&mut self) {
        *self = Self::X;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate_test() {
        let mut x = 3;
        mutate(&mut x);
        assert_eq!(x,4);

        let mut a = XYZ::Z;
        a.mutate();
        assert_eq!(a, XYZ::X);
    }

}
