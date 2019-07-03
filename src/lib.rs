struct Dollar {
    amount: u32,
}

impl Dollar {
    fn new(x: u32) -> Dollar {
        Dollar {
            amount: x,
        }
    }
    fn times(&mut self, multiplier: u32) {
        self.amount *= multiplier;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_test() {
        let mut five = Dollar::new(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}