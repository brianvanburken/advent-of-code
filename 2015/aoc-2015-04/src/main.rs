use std::convert::TryInto;

use md5;

fn calculate_coin_number_for_secret(secret: &str, start_len: usize) -> usize {
    let start_hash_match = "0".repeat(start_len);

    for i in 1..i64::MAX {
        let digest = md5::compute(secret.to_owned() + &i.to_string());
        let hash = format!("{:x}", digest);
        if hash.starts_with(&start_hash_match) {
            return i.try_into().unwrap();
        }
    }
    0
}

fn main() {
    println!(
        "Result for secret part 1: {}",
        calculate_coin_number_for_secret("ckczppom", 5)
    );

    println!(
        "Result for secret part 2: {}",
        calculate_coin_number_for_secret("ckczppom", 6)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_609043_for_secret_abcdef() {
        assert_eq!(calculate_coin_number_for_secret("abcdef", 5), 609043);
    }

    #[test]
    fn it_returns_1048970_for_secret_pqrstuv() {
        assert_eq!(calculate_coin_number_for_secret("pqrstuv", 5), 1048970);
    }
}
