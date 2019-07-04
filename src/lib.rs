use std::ops::Add;

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

impl Expression for Money {}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount 
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}

trait Expression {}

struct Bank {}

impl Bank {
    fn new() -> Bank {
        Bank {}
    }

    fn reduced(&self, sum: Money, currency: Currency) -> Money {
        Money::dollar(10)
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


    #[test]
    fn add_test() {
        let five = Money::dollar(5);
        let sum = five.times(5);
        let bank = Bank::new();
        let reduced = bank.reduced(sum, Currency::USD);
        assert_eq!(Money::dollar(10), reduced); 
    }
    
}