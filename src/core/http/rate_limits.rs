use std::collections::HashMap;

use crate::utils::time;

pub struct RateLimits {
    rps: i32,
    request_history: HashMap<i32, i32>,
}

impl RateLimits {
    pub fn new(rps: i32) -> Self {
        return Self {
            rps,
            request_history: HashMap::new(),
        };
    }

    pub fn is_limited(&self) -> bool {
        let time: i32 = time::get_epoch_time() as i32;
        let rps_amount: Option<&i32> = self.request_history.get(&time);
        match rps_amount {
            Some(rps_amount) => return *rps_amount > self.rps,
            None => return false,
        }
    }

    pub fn new_request(&mut self) -> () {
        let time: i32 = time::get_epoch_time() as i32;
        let rps_amount: Option<&i32> = self.request_history.get(&time);
        match rps_amount {
            Some(rps_amount) => self.request_history.insert(time, rps_amount + 1),
            None => self.request_history.insert(time, 1),
        };
    }

    pub fn amount(&self) -> i32 {
        let time: i32 = time::get_epoch_time() as i32;
        let rps_amount: Option<&i32> = self.request_history.get(&time);
        match rps_amount {
            Some(rps_amount) => return *rps_amount,
            None => return 0,
        };
    }

    pub fn history(&self) -> HashMap<i32, i32> {
        return self.request_history.clone();
    }
}
