#[derive(Debug)]
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

impl PartialEq for Dollar {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_test() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn eq_test() {
        assert_eq!(Dollar::new(5), Dollar::new(5));
        assert!(Dollar::new(5) != Dollar::new(6));
    }
}