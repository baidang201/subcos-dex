#![cfg_attr(not(feature = "std"), no_std, no_main)]
use openbrush::contracts::psp37::Id;
use openbrush::traits::{AccountId, Timestamp};
use scale::{Decode, Encode};

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
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Order {
    counter: u32, //order index
    address: AccountId,
    pair: (Id, Id), //AssetId_1 is base,  AssetId_2 is quote token
    timestamp: Timestamp,
    order_type: OrderType,
    amount_offered: u128,
    amout_requested: u128,
}

#[ink::contract]
mod subcosdex {
    use crate::Id;
    use crate::{DexError, Order, OrderType};
    use ink::prelude::vec;
    use ink::prelude::vec::Vec;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Subcosdex {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        asset_id: Id,
        amount: u128,
    }

    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        asset_id: Id,
        amount: u128,
    }

    #[ink(event)]
    pub struct OrderCreated {
        #[ink(topic)]
        asset_id_1: Id,
        #[ink(topic)]
        asset_id_2: Id,
        offered_amount: u128,
        requested_amount: u128,
        order_type: OrderType,
    }

    #[ink(event)]
    pub struct OrderCanceled {
        #[ink(topic)]
        counter: u128,
    }

    #[ink(event)]
    pub struct OrderTaken {
        #[ink(topic)]
        counter: u128,
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
        pub fn deposit(&mut self, _asset_id: Id, _amount: u128) -> Result<(), DexError> {
            Self::env().emit_event(Deposit {
                asset_id: _asset_id,
                amount: _amount,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, _owner: AccountId, _id: Id) -> Balance {
            Balance::MAX
        }

        #[ink(message)]
        pub fn withdraw(&mut self, _asset_id: Id, _amount: u128) -> Result<(), DexError> {
            Self::env().emit_event(Withdraw {
                asset_id: _asset_id,
                amount: _amount,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn tokens(&self) -> Vec<Id> {
            vec![Id::U64(1), Id::U128(2), Id::U64(3), Id::U16(9), Id::U64(11)]
        }

        #[ink(message)]
        pub fn owners_tokens(&self, _owner: AccountId) -> Vec<Id> {
            vec![Id::U64(1), Id::U64(3)]
        }

        #[ink(message)]
        pub fn order_for(&self, _order_index: u128) -> Option<Order> {
            Some(Order {
                counter: 0,
                address: AccountId::from([0x01; 32]),
                pair: (Id::U64(1), Id::U64(3)),
                timestamp: self.env().block_timestamp(),
                order_type: OrderType::BUY,
                amount_offered: 2,
                amout_requested: 1,
            })
        }

        #[ink(message)]
        pub fn pair_orders(&self, _asset_id_1: Id, _asset_id_2: Id) -> Vec<Order> {
            vec![
                Order {
                    counter: 0,
                    address: AccountId::from([0x01; 32]),
                    pair: (Id::U64(1), Id::U64(3)),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::BUY,
                    amount_offered: 2,
                    amout_requested: 1,
                },
                Order {
                    counter: 1,
                    address: AccountId::from([0x01; 32]),
                    pair: (Id::U64(1), Id::U64(3)),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::SELL,
                    amount_offered: 1,
                    amout_requested: 33,
                },
            ]
        }

        #[ink(message)]
        pub fn user_orders(&self, _owner: AccountId) -> Vec<Order> {
            vec![
                Order {
                    counter: 0,
                    address: AccountId::from([0x01; 32]),
                    pair: (Id::U64(1), Id::U64(3)),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::BUY,
                    amount_offered: 2,
                    amout_requested: 1,
                },
                Order {
                    counter: 1,
                    address: AccountId::from([0x01; 32]),
                    pair: (Id::U64(1), Id::U64(3)),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::SELL,
                    amount_offered: 1,
                    amout_requested: 33,
                },
                Order {
                    counter: 2,
                    address: AccountId::from([0x01; 32]),
                    pair: (Id::U128(2), Id::U64(3)),
                    timestamp: self.env().block_timestamp(),
                    order_type: OrderType::BUY,
                    amount_offered: 2,
                    amout_requested: 1,
                },
                Order {
                    counter: 3,
                    address: AccountId::from([0x01; 32]),
                    pair: (Id::U128(2), Id::U64(3)),
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
            _asset_id_1: Id,
            _asset_id_2: Id,
            _offered_amount: u128,
            _requested_amount: u128,
            _order_type: OrderType,
        ) -> Result<(), DexError> {
            Self::env().emit_event(OrderCreated {
                asset_id_1: _asset_id_1,
                asset_id_2: _asset_id_2,
                offered_amount: _offered_amount,
                requested_amount: _requested_amount,
                order_type: _order_type,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn cancel_order(&self, _order_index: u128) -> Result<(), DexError> {
            Self::env().emit_event(OrderCanceled {
                counter: _order_index,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn take_order(&self, _order_index: u128) -> Result<(), DexError> {
            Self::env().emit_event(OrderTaken {
                counter: _order_index,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn owner_token_by_index(&self, index: u32) -> Id {
            // will use pallet map iter later
            Id::U64(index.into())
        }

        #[ink(message)]
        pub fn pair_order_by_index(&self, _index: u32) -> Order {
            // will use pallet map iter later
            Order {
                counter: 0,
                address: AccountId::from([0x01; 32]),
                pair: (Id::U64(1), Id::U64(3)),
                timestamp: self.env().block_timestamp(),
                order_type: OrderType::BUY,
                amount_offered: 2,
                amout_requested: 1,
            }
        }

        #[ink(message)]
        pub fn user_order_by_index(&self, _index: u32) -> Order {
            // will use pallet map iter later
            Order {
                counter: 0,
                address: AccountId::from([0x01; 32]),
                pair: (Id::U64(1), Id::U64(3)),
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
