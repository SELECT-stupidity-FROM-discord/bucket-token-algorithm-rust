use std::time::{Duration, Instant};
use std::{thread};
use rand::Rng;

struct Cooldown {
    tokens: u64,
    per: u64,
    bucket: u64,
    last_check: Instant,
}

impl Cooldown {
    fn forward(&self, request: &str) -> String {
        return format!("Hello {}", request);
    }
}

impl Cooldown {
    fn drop_pack(&self, remaining: f64) -> String {
        return format!("You are being rated limited, please try again in {} seconds", remaining);
    }
}

impl Cooldown {
    fn check_ratelimit(&mut self, request: &str) -> String {
        let current = Instant::now();
        let elapsed = current - self.last_check;
        self.bucket += (elapsed.as_secs() as f32 * (self.tokens as f32 /self.per as f32)) as u64;
        if self.bucket > self.tokens {
            self.bucket = self.tokens;
        }
        if self.bucket < 1 {
            let rate_limit_time_remaining = (self.per as f64/self.tokens as f64) - elapsed.as_secs() as f64;
            return String::from(self.drop_pack(rate_limit_time_remaining));
        }
        else {
            self.last_check = current;
            self.bucket = self.bucket - 1;
            return self.forward(&request);
        }
    }
}

fn main () {
    let mut var = Cooldown {
        tokens: 700,
        per: 60,
        bucket: 700,
        last_check: Instant::now()
    };
    loop {
        let randr: u64 = rand::thread_rng().gen_range(1..401);
        let sleep_dur = Duration::from_millis(randr);
        let var: String = var.check_ratelimit("World");
        println!("{}", var);
        thread::sleep(sleep_dur);
    }
}