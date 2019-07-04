use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Currency {
    USD,
    CHE,
}

#[derive(Debug, Clone)]
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

impl Expression for Money {
    fn reduced(&self) -> Money {
        self.clone()
    }
}

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

trait Expression {
    fn reduced(&self) -> Money;
}

struct Bank {}

impl Bank {
    fn new() -> Bank {
        Bank {}
    }

    fn reduced<T> (source: T) -> Money 
        where T: Expression
    {
        source.reduced()
    }
}

struct Sum {
    augend: Money,
    addend: Money,
}

impl Sum {
    fn new(audend: Money, addend: Money) -> Sum {
        Sum {
            augend: audend,
            addend: addend,
        }
    }
}

impl Expression for Sum {
    fn reduced(&self) -> Money {
        Money {
            amount: self.augend.amount + self.addend.amount,
            currency: self.augend.currency,
        }
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
    fn reduce_sum_test() {
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let result = Bank::reduced(sum);
        assert_eq!(Money::dollar(7), result);
    }

    #[test]
    fn reduce_money_test() {
       let result = Bank::reduced(Money::dollar(1)); 
       assert_eq!(Money::dollar(1), result);
    }
    
}