#[derive(Debug, PartialEq, Clone, Copy)]
enum Currency {
    USD,
    CHE,
}

#[derive(Debug)]
struct Money {
    amount: u32,
    currency: Currency,
}

impl Money {
    fn new(x: u32, currecy: Currency) -> Money {
        Money {
            amount: x,
            currency: currecy,
        }
    }
    fn times(&self, multiplier: u32) -> Money {
        Money { 
            amount: self.amount * multiplier,
            currency: self.currency,
        }
    }
    fn dollar(x: u32) -> Money {
        Money {
            amount: x,
            currency: Currency::USD,
        }
    }
    fn franc(x: u32) -> Money {
        Money {
            amount: x,
            currency: Currency::CHE,
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_test() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn eq_test() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert!(Money::dollar(5) != Money::dollar(6));
        assert!(Money::dollar(5) != Money::franc(6));
    }
    
}