use crate::rates::Rate;
use crate::currency::{CurrCode, Currency};
use std::fmt;
use std::error::Error;
use std::ops::Deref;


#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Value {
    pub currency_code: CurrCode,
    pub amount: f64,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:.2}", self.currency_code.currency().symbol, self.amount)
    }
}

// todo - build a macro to ensure currency parity for equations (checks fn to ensure CurrCode is eq)
impl Value {
    pub fn new(amount: f64, currency_code: CurrCode) -> Value {
        Value{currency_code, amount }
    }

    pub fn new_vec(vector_of_num: Vec<f64>, currency_code: CurrCode) -> Vec<Value> {
        vector_of_num.iter().map(|f| Value::new(*f, currency_code)).collect()
    }

    pub fn convert(&self, start_cur_amount: Value, conversion_rate: Rate) -> Value {
        Value::new(start_cur_amount.amount * conversion_rate.rate,
                   start_cur_amount.currency_code)
    }

    // method that takes a string and parses it into a value
    // need to include a CurrCode b/c symbols are used with diff curr (e.g. $ dollar is common)
//    pub fn value_parse_from_str(string: &str, exp_curr_code: CurrCode) -> Result<Value, Box<dyn Error>> {
//        Ok()
//    } // todo return a result


    pub fn value_to_string(&self) -> String {
        self.to_string()
    }

    // involves leaking the memory of the String...use with caution
    pub fn value_to_str(&self) -> &'static str {
        Box::leak(self.to_string().into_boxed_str())
    }

}