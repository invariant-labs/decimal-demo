#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod types;

#[ink::contract]
mod amm {
    use crate::types::{Liquidity, Percentage, Price, Ratio, TokenAmount};
    use decimal::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct AMM {
        x: TokenAmount,
        y: TokenAmount,
        l: Liquidity,
        fee: Percentage,
    }

    impl AMM {
        #[ink(constructor)]
        pub fn new(amm_fee: Percentage) -> Self {
            Self {
                fee: amm_fee,
                ..Default::default()
            }
        }

        #[ink(message)]
        pub fn add_liquidity(&mut self, delta_x: TokenAmount) -> Liquidity {
            let ratio = if self.l.is_zero() {
                self.x = delta_x;
                self.y = delta_x;
                self.l = Liquidity::from_decimal(self.x.big_mul(self.y));
                return Liquidity::from_decimal(self.x.big_mul(self.y));
            } else {
                Ratio::from_decimal(delta_x).big_div_up(self.x)
            };

            self.x = (self.x) * (Ratio::from_integer(1) + ratio);
            self.y = (self.y) * (Ratio::from_integer(1) + ratio);
            self.l = self.l * (Ratio::from_integer(1) + ratio);

            Liquidity::from_decimal(ratio * self.l)
        }

        #[ink(message)]
        pub fn remove_liquidity(&mut self, delta_x: TokenAmount) -> Liquidity {
            let ratio = Ratio::from_decimal(delta_x).big_div(self.x);

            let alpha = Ratio::from_integer(1) - ratio;
            self.x = self.x * alpha;
            self.y = self.y * alpha;
            self.l = self.l * (alpha * alpha);

            Liquidity::from_decimal(ratio * self.l)
        }

        #[ink(message)]
        pub fn swap(&mut self, amount: TokenAmount, in_x: bool) {
            if in_x {
                self.x += amount;
                let delta_x = TokenAmount::from_decimal_up(
                    (Percentage::from_integer(1) - self.fee).big_mul(amount),
                );
                let withdraw_amount = (amount * self.y).big_div_up(delta_x);
                self.y -= withdraw_amount;
            } else {
                self.y += amount;
                let delta_y = TokenAmount::from_decimal_up(
                    (TokenAmount::from_integer(1) - self.x).big_mul(amount),
                );
                let withdraw_amount = (amount * self.x).big_div_up(delta_y);
                self.x -= withdraw_amount;
            };
        }

        #[ink(message)]
        pub fn get_price(&self) -> Price {
            Price::from_decimal(self.y.big_div(self.x))
        }

        #[ink(message)]
        pub fn get_x(&self) -> TokenAmount {
            self.x
        }

        #[ink(message)]
        pub fn get_y(&self) -> TokenAmount {
            self.y
        }

        #[ink(message)]
        pub fn get_l(&self) -> Liquidity {
            self.l
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn initialize_works() {
            let fee = Percentage::from_scale(1, 2);
            let _ = AMM::new(fee);
        }

        #[ink::test]
        fn add_liquidity() {
            let fee = Percentage::from_scale(1, 2);
            let mut amm = AMM::new(fee);

            {
                let amm_x = amm.get_x();
                let amm_y = amm.get_y();
                let amm_l = amm.get_l();
                println!("x: {:?}, y: {:?}, l: {:?}", amm_x, amm_y, amm_l);
            }

            let delta_x = TokenAmount(5);
            amm.add_liquidity(delta_x);

            {
                let amm_x = amm.get_x();
                let amm_y = amm.get_y();
                let amm_l = amm.get_l();
                println!("x: {:?}, y: {:?}, l: {:?}", amm_x, amm_y, amm_l);
            }

            let current_price = amm.get_price();
            assert_eq!(current_price, Price::from_integer(1));
        }

        #[ink::test]
        fn remove_liquidity() {
            let fee = Percentage::from_scale(1, 2);
            let mut amm = AMM::new(fee);

            {
                let amm_x = amm.get_x();
                let amm_y = amm.get_y();
                let amm_l = amm.get_l();
                println!("x: {:?}, y: {:?}, l: {:?}", amm_x, amm_y, amm_l);
            }

            let delta_x = TokenAmount(100);
            amm.add_liquidity(delta_x);

            {
                let amm_x = amm.get_x();
                let amm_y = amm.get_y();
                let amm_l = amm.get_l();
                println!("x: {:?}, y: {:?}, l: {:?}", amm_x, amm_y, amm_l);
            }

            let current_price = amm.get_price();
            assert_eq!(current_price, Price::from_integer(1));

            let delta_x = TokenAmount(10);
            amm.remove_liquidity(delta_x);

            {
                let amm_x = amm.get_x();
                let amm_y = amm.get_y();
                let amm_l = amm.get_l();
                println!("x: {:?}, y: {:?}, l: {:?}", amm_x, amm_y, amm_l);
            }

            let current_price = amm.get_price();
            assert_eq!(current_price, Price::from_integer(1));
        }
    }
}
