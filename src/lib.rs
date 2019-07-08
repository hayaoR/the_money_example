use std::ops::Add;
use std::ops::Mul;
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
        Money {
            amount: self.amount / rate,
            currency: currency,
        }
    }

    fn times(&self, multiplier: u32) -> Box<Sum> {
        Box::new (Sum { 
            augend: Box::new( 
            Money { 
                amount: self.amount * multiplier,
                currency: self.currency,
            }),
            addend: Box::new(
                Money {
                    amount: 0,
                    currency: self.currency,
                }
            ),
        }
        )
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount 
    }
}

impl Add for Money {
    type Output = Sum;
    fn add(self, other: Self) -> Sum {
        Sum {
            augend: Box::new(self),
            addend: Box::new(other),
        }
    }
}

impl Mul<u32> for Money {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Money {
            amount: self.amount * rhs,
            currency: self.currency,
        }
    }
}

trait Expression {
    fn reduced(&self, bank: &Bank, currency: Currency) -> Money;
    fn times(&self, multiplier: u32) -> Box<Sum>;
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

    fn add_rate (&mut self, from: Currency, to: Currency, rate: u32) {
        self.map.insert((from, to), rate);
    }

    fn get_rate (&self, from: Currency, to: Currency) -> u32 {
        if from == to {
            return 1
        }
        *self.map.get(&(from, to)).unwrap_or(&1)
    }
}

struct Sum 
{
    augend: Box<Expression>,
    addend: Box<Expression>,
}

impl Sum
{
    fn new(augend: Box<Expression>, addend: Box<Expression>) -> Sum {
       Sum {
            augend: augend,
            addend: addend,
        }
    }
}

impl Expression for Sum {
    fn reduced(&self, bank: &Bank, currency: Currency) -> Money {
        Money {
            amount: self.augend.reduced(bank, currency).amount + self.addend.reduced(bank, currency).amount,
            currency: currency,
        }
    }

    fn times(&self, multiplier: u32) -> Box<Sum> {
        Box::new( Sum {
            augend: self.augend.times(multiplier),
            addend: self.addend.times(multiplier),
        })
    }
}

impl Add <Box<Expression>> for Sum {
    type Output = Sum;
    fn add(self, other: Box<Expression>) -> Sum {
        Sum {
            augend: Box::new(self),
            addend: other,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_test() {
        let five = Money::dollar(5);
        let five2 = five.clone();
        assert_eq!(Money::dollar(10), five*2);
        assert_eq!(Money::dollar(15), five2*3);
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
        let sum = Sum::new(Box::new(Money::dollar(3)), Box::new(Money::dollar(4)));
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
        let mut bank = Bank::new();
        bank.add_rate(Currency::CHE, Currency::USD, 2);
        let result = bank.reduced(Money::franc(2), Currency::USD);
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    fn rate_test() {
        let bank = Bank::new();
        assert_eq!(1, bank.get_rate(Currency::USD, Currency::USD));
    }

    #[test]
    fn mixed_add_test() {
        let five_backs = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();

        bank.add_rate(Currency::CHE, Currency::USD, 2);
        let result = bank.reduced(five_backs + ten_francs, Currency::USD);
        assert_eq!(Money::dollar(10), result);
    }

    #[test]
    fn sum_plus_money_test() {
        let five_backs = Money::dollar(5);
        let five_backs2 = five_backs.clone();
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();

        bank.add_rate(Currency::CHE, Currency::USD, 2);
        let sum = Sum::new(Box::new(five_backs), Box::new(ten_francs)) + Box::new(five_backs2);
        let result = bank.reduced(sum, Currency::USD);
        assert_eq!(Money::dollar(15), result);
    }

    #[test]
    fn sum_times_test() {
        let five_backs = Money::dollar(5);
        let ten_francs = Money::franc(10);

        let mut bank = Bank::new();
        bank.add_rate(Currency::CHE, Currency::USD, 2);

        let sum = Sum::new(Box::new(five_backs), Box::new(ten_francs)).times(2);
        let result = bank.reduced(*sum, Currency::USD);
        assert_eq!(Money::dollar(20), result);
    }
}