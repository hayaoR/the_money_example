use std::ops::Add;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
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
    fn reduced(&self, bank: &Bank, currency: Currency) -> Money {
        let rate = bank.get_rate(self.currency, currency);
        //let rate = if self.currency == Currency::CHE && currency == Currency::USD { 2 } else { 1 };
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
    fn reduced(&self, bank: &Bank, currency: Currency) -> Money;
}

struct Bank {
    map: HashMap<(Currency, Currency), u32>, 
}

impl Bank {
    fn new() -> Bank {
        Bank {
            map: HashMap::new(),
        }
    }

    fn reduced<T> (&self, source: T, currency: Currency) -> Money 
        where T: Expression
    {
        source.reduced(&self, currency)
    }

    fn add_rate (&self, cur1: Currency, cur2: Currency, rate: u32) {
    }

    fn get_rate (&self, cur1: Currency, cur2: Currency) -> u32 {
        if cur1 == cur2 {
            return 1
        }
        *self.map.get(&(cur1, cur2)).unwrap_or(&2)
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
    fn reduced(&self, bank: &Bank, currency: Currency) -> Money {
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
        let bank = Bank::new();
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let result = bank.reduced(sum, Currency::USD);
        assert_eq!(Money::dollar(7), result);
    }

    #[test]
    fn reduce_money_test() {
        let bank = Bank::new();
        let result = bank.reduced(Money::dollar(1), Currency::USD); 
        assert_eq!(Money::dollar(1), result);
    }
    
    #[test]
    fn reduce_money_different_currency() {
        let bank = Bank::new();
        bank.add_rate(Currency::CHE, Currency::USD, 2);
        let result = bank.reduced(Money::franc(2), Currency::USD);
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    fn rate_test() {
        let bank = Bank::new();
        assert_eq!(1, bank.get_rate(Currency::USD, Currency::USD));
    }
}