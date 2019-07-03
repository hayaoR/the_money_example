#[derive(Debug)]
struct Money {
    amount: u32,
}

impl Money {
    fn new(x: u32) -> Money {
        Money {
            amount: x,
        }
    }
    fn times(&self, multiplier: u32) -> Money {
        Money { 
            amount: self.amount * multiplier,
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount 
    }
}
#[derive(Debug)]
struct Dollar {
    amount: u32,
}

impl Dollar {
    fn new(x: u32) -> Money {
        Money {
            amount: x,
        }
    }
}



#[derive(Debug)]
struct Franc {
    amount: u32,
}

impl Franc {
    fn new(x: u32) -> Money {
        Money {
            amount: x,
        }
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
    
     #[test]
    fn mul_test_franc() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}