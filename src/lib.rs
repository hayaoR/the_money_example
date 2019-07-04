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
    fn reduced(&self, currency: Currency) -> Money {
        let rate = if self.currency == Currency::CHE && currency == Currency::USD { 2 } else { 1 };
        Money {
            amount: self.amount / rate,
            currency: currency,
        }
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
    fn reduced(&self, currecy: Currency) -> Money;
}

struct Bank {}

impl Bank {
    fn new() -> Bank {
        Bank {}
    }

    fn reduced<T> (source: T, currency: Currency) -> Money 
        where T: Expression
    {
        source.reduced(currency)
    }

    fn add_rate (cur1: Currency, cur2: Currency, rate: u32) {
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
    fn reduced(&self, currency: Currency) -> Money {
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
        let result = Bank::reduced(sum, Currency::USD);
        assert_eq!(Money::dollar(7), result);
    }

    #[test]
    fn reduce_money_test() {
       let result = Bank::reduced(Money::dollar(1), Currency::USD); 
       assert_eq!(Money::dollar(1), result);
    }
    
    #[test]
    fn reduce_money_different_currency() {
        Bank::add_rate(Currency::CHE, Currency::USD, 2);
        let result = Bank::reduced(Money::franc(2), Currency::USD);
        assert_eq!(Money::dollar(1), result);
    }
}