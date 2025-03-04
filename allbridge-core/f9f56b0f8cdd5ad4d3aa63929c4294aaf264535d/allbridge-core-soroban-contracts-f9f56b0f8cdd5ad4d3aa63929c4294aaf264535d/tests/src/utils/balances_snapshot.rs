use crate::{contracts::pool::Pool as PoolInfo, utils::format_diff};
use core::panic;
use ethnum::U256;
use shared::utils::num::sqrt;
use std::ops::Index;

use super::BridgeEnv;

impl PoolInfo {
    pub fn get_y(&self, native_x: u128) -> u128 {
        let a4 = self.a << 2;
        let ddd = U256::new(self.d * self.d) * self.d;
        // 4A(D - x) - D
        let part1 = a4 as i128 * (self.d as i128 - native_x as i128) - self.d as i128;
        // x * (4AD³ + x(part1²))
        let part2 = (ddd * a4 + (U256::new((part1 * part1) as u128) * native_x)) * native_x;
        // (sqrt(part2) + x(part1)) / 8Ax)
        (sqrt(&part2).as_u128() as i128 + (native_x as i128 * part1)) as u128
            / ((self.a << 3) * native_x)
    }
}

#[derive(Debug, Clone)]
pub struct BalancesSnapshot {
    pub yaro_pool_info: PoolInfo,
    pub yusd_pool_info: PoolInfo,

    pub alice_yaro_balance: u128,
    pub alice_yusd_balance: u128,
    pub alice_native_balance: u128,

    pub bob_yaro_balance: u128,
    pub bob_yusd_balance: u128,
    pub bob_native_balance: u128,

    pub bridge_yaro_balance: u128,
    pub bridge_yusd_balance: u128,
    pub bridge_native_balance: u128,

    pub pool_yaro_balance: u128,
    pub pool_yusd_balance: u128,

    pub messenger_native_balance: u128,
}

impl Index<String> for BalancesSnapshot {
    type Output = u128;

    fn index(&self, string: String) -> &Self::Output {
        self.index(string.as_str())
    }
}

impl Index<&str> for BalancesSnapshot {
    type Output = u128;

    fn index(&self, string: &str) -> &Self::Output {
        match string {
            "alice_yaro_balance" => &self.alice_yaro_balance,
            "alice_yusd_balance" => &self.alice_yusd_balance,
            "alice_native_balance" => &self.alice_native_balance,
            "bob_yaro_balance" => &self.bob_yaro_balance,
            "bob_yusd_balance" => &self.bob_yusd_balance,
            "bob_native_balance" => &self.bob_native_balance,
            "bridge_yaro_balance" => &self.bridge_yaro_balance,
            "bridge_yusd_balance" => &self.bridge_yusd_balance,
            "bridge_native_balance" => &self.bridge_native_balance,
            "pool_yaro_balance" => &self.pool_yaro_balance,
            "pool_yusd_balance" => &self.pool_yusd_balance,
            "messenger_native_balance" => &self.messenger_native_balance,
            _ => panic!("BalancesSnapshot: unknown field: {}", string),
        }
    }
}

impl BalancesSnapshot {
    pub fn get_pool_info_by_tag(&self, tag: &str) -> PoolInfo {
        match tag {
            "yaro" => self.yaro_pool_info.clone(),
            "yusd" => self.yusd_pool_info.clone(),
            _ => panic!("Unexpected tag"),
        }
    }

    pub fn take(bridge_env: &BridgeEnv) -> BalancesSnapshot {
        let alice_address = bridge_env.alice.as_address();
        let bob_address = bridge_env.bob.as_address();

        let alice_yaro_balance = bridge_env.yaro_token.balance_of(&alice_address);
        let alice_yusd_balance = bridge_env.yusd_token.balance_of(&alice_address);
        let alice_native_balance = bridge_env.native_token.balance_of(&alice_address);

        let bob_yaro_balance = bridge_env.yaro_token.balance_of(&bob_address);
        let bob_yusd_balance = bridge_env.yusd_token.balance_of(&bob_address);
        let bob_native_balance = bridge_env.native_token.balance_of(&bob_address);

        let bridge_yaro_balance = bridge_env.yaro_token.balance_of(&bridge_env.bridge.id);
        let bridge_yusd_balance = bridge_env.yusd_token.balance_of(&bridge_env.bridge.id);
        let bridge_native_balance = bridge_env.native_token.balance_of(&bridge_env.bridge.id);

        let pool_yaro_balance = bridge_env.yaro_token.balance_of(&bridge_env.yaro_pool.id);
        let pool_yusd_balance = bridge_env.yusd_token.balance_of(&bridge_env.yusd_pool.id);

        let messenger_native_balance = bridge_env.native_token.balance_of(&bridge_env.messenger.id);

        BalancesSnapshot {
            yaro_pool_info: bridge_env.yaro_pool.client.get_pool(),
            yusd_pool_info: bridge_env.yusd_pool.client.get_pool(),
            alice_yaro_balance,
            bridge_yaro_balance,
            alice_native_balance,
            pool_yaro_balance,
            bridge_native_balance,
            messenger_native_balance,
            alice_yusd_balance,
            pool_yusd_balance,
            bridge_yusd_balance,
            bob_yaro_balance,
            bob_yusd_balance,
            bob_native_balance,
        }
    }

    #[allow(dead_code)]
    pub fn print_change_with(&self, other: &BalancesSnapshot, title: Option<&str>) {
        let title = title.unwrap_or("Diff");

        println!("----------------------| {title} |----------------------");

        println!(
            "Alice native balance change: {}",
            &format_diff(self.alice_native_balance, other.alice_native_balance)
        );

        println!(
            "Alice yaro balance change: {}",
            &format_diff(self.alice_yaro_balance, other.alice_yaro_balance)
        );

        println!(
            "Alice yusd balance change: {}",
            &format_diff(self.alice_yusd_balance, other.alice_yusd_balance)
        );

        println!(
            "Bob native balance change: {}",
            &format_diff(self.bob_native_balance, other.bob_native_balance)
        );

        println!(
            "Bob yaro balance change: {}",
            &format_diff(self.bob_yaro_balance, other.bob_yaro_balance)
        );

        println!(
            "Bob yusd balance change: {}",
            &format_diff(self.bob_yusd_balance, other.bob_yusd_balance)
        );

        println!(
            "Pool yaro balance change: {}",
            &format_diff(self.pool_yaro_balance, other.pool_yaro_balance)
        );

        println!(
            "Pool yusd balance change: {}",
            &format_diff(self.pool_yusd_balance, other.pool_yusd_balance)
        );

        println!(
            "Bridge yaro balance change: {}",
            &format_diff(self.bridge_yaro_balance, other.bridge_yaro_balance)
        );

        println!(
            "Bridge yaro balance change: {}",
            &format_diff(self.bridge_yusd_balance, other.bridge_yusd_balance)
        );

        println!(
            "Bridge native balance change: {}",
            &format_diff(self.bridge_native_balance, other.bridge_native_balance)
        );

        println!(
            "Messenger native balance change: {}",
            &format_diff(
                self.messenger_native_balance,
                other.messenger_native_balance
            )
        );

        self.print_pool_change_with(other);
    }

    fn print_pool_change_with(&self, other: &BalancesSnapshot) {
        println!("------| Pools |------");

        println!(
            "Yaro pool D change: {}",
            &format_diff(self.yaro_pool_info.d, other.yaro_pool_info.d)
        );
        println!(
            "Yaro pool token balance change: {}",
            &format_diff(
                self.yaro_pool_info.token_balance,
                other.yaro_pool_info.token_balance
            )
        );
        println!(
            "Yaro pool vUsd balance change: {}",
            &format_diff(
                self.yaro_pool_info.v_usd_balance,
                other.yaro_pool_info.v_usd_balance
            )
        );
        println!(
            "Yaro pool reserves change: {}",
            &format_diff(self.yaro_pool_info.reserves, other.yaro_pool_info.reserves)
        );

        println!(
            "Yusd pool D change: {}",
            &format_diff(self.yusd_pool_info.d, other.yusd_pool_info.d)
        );
        println!(
            "Yusd pool token balance change: {}",
            &format_diff(
                self.yusd_pool_info.token_balance,
                other.yusd_pool_info.token_balance
            )
        );
        println!(
            "Yusd pool vUsd balance change: {}",
            &format_diff(
                self.yusd_pool_info.v_usd_balance,
                other.yusd_pool_info.v_usd_balance
            )
        );
        println!(
            "Yusd pool reserves change: {}",
            &format_diff(self.yusd_pool_info.reserves, other.yusd_pool_info.reserves)
        );
    }
}
