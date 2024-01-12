#![cfg_attr(not(feature = "std"), no_std, no_main)]
use scale::{Decode, Encode};

use openbrush::traits::{AccountId, Timestamp};

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DexError {
    FailScaleCode,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OrderType {
    BUY,
    SELL,
}

#[derive(Decode, Encode)]
pub struct Order {
    counter: u32, //order index
    address: AccountId,
    pair: (u128, u128), //AssetId_1 is base,  AssetId_2 is quote token
    timestamp: Timestamp,
    order_type: OrderType,
    amount_offered: u128,
    amout_requested: u128,
}

#[cfg(feature = "psp37")]
#[ink::contract]
mod subcosdex {
    use crate::DexError;
    use crate::OrderType;
    use openbrush::contracts::psp37::Id;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Subcosdex {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Subcosdex {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn deposit(&mut self, asset_id: Id, amount: u128) -> Result<(), DexError> {
            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId, id: Id) -> Balance {
            Balance::new(111)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, asset_id: Id, amount: u128) -> Result<(), DexError> {
            Ok(())
        }

        #[ink(message)]
        pub fn tokens(&self) -> Vec<Id> {
            vec![1, 3, 9, 11]
        }

        #[ink(message)]
        pub fn owners_tokens(owner: AccountId) -> Vec<Id> {
            vec![1, 3]
        }

        #[ink(message)]
        pub fn order_for(&self, order_index: u128) -> Option<Order> {
            Order {
                counter: 0,
                address: AccountId::default(),
                pair: (1, 3),
                timestamp: self.env().block_timestamp(),
                order_type: OrderType::BUY,
                amount_offered: 2,
                amout_requested: 1,
            }
        }

        #[ink(message)]
        pub fn pair_orders(&self, asset_id_1: Id, asset_id_2: Id) -> Vec<Order> {
            vec![
                Order {
                    counter: 0,
                    address: AccountId::default(),
                    pair: (1, 3),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::BUY,
                    amount_offered: 2,
                    amout_requested: 1,
                },
                Order {
                    counter: 1,
                    address: AccountId::default(),
                    pair: (1, 3),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::SELL,
                    amount_offered: 1,
                    amout_requested: 33,
                },
            ]
        }

        #[ink(message)]
        pub fn get_user_orders(&self, owner: AccountId) -> Vec<Order> {
            vec![
                Order {
                    counter: 0,
                    address: AccountId::default(),
                    pair: (1, 3),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::BUY,
                    amount_offered: 2,
                    amout_requested: 1,
                },
                Order {
                    counter: 1,
                    address: AccountId::default(),
                    pair: (1, 3),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::SELL,
                    amount_offered: 1,
                    amout_requested: 33,
                },
                Order {
                    counter: 2,
                    address: AccountId::default(),
                    pair: (2, 3),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::BUY,
                    amount_offered: 2,
                    amout_requested: 1,
                },
                Order {
                    counter: 3,
                    address: AccountId::default(),
                    pair: (2, 3),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::SELL,
                    amount_offered: 1,
                    amout_requested: 33,
                },
            ]
        }

        #[ink(message)]
        pub fn make_order(
            &self,
            asset_id_1: Id,
            asset_id_2: Id,
            offered_amount: u128,
            requested_amount: u128,
            order_type: OrderType,
        ) -> Result<(), DexError> {
            Ok(())
        }

        #[ink(message)]
        pub fn cancel_order(&self, order_index: u128) -> Result<(), DexError> {
            Ok(())
        }

        #[ink(message)]
        pub fn take_order(&self, order_index: u128) -> Result<(), DexError> {
            Ok(())
        }

        #[ink(message)]
        pub fn owner_token_by_index(&self, index: u32) -> Id {
            // will use pallet map iter later
            index
        }

        #[ink(message)]
        pub fn pair_order_by_Index(&self, index: u32) -> Order {
            // will use pallet map iter later
            Order {
                counter: 0,
                address: AccountId::default(),
                pair: (1, 3),
                timestamp: self.env().block_timestamp(),
                order_type: OrderType::BUY,
                amount_offered: 2,
                amout_requested: 1,
            }
        }

        #[ink(message)]
        pub fn user_order_by_Index(&self, index: u32) -> Order {
            // will use pallet map iter later
            Order {
                counter: 0,
                address: AccountId::default(),
                pair: (1, 3),
                timestamp: self.env().block_timestamp(),
                order_type: OrderType::BUY,
                amount_offered: 2,
                amout_requested: 1,
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
    }
}
