struct Dollar {
    amount: u32,
}

impl Dollar {
    fn new(x: u32) -> Dollar {
        Dollar {
            amount: x,
        }
    }
    fn times(&self, multiplier: u32) -> Dollar {
        Dollar { 
            amount: self.amount * multiplier,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_test() {
        let five = Dollar::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }
}