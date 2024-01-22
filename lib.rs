#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::env::{chain_extension::FromStatusCode, DefaultEnvironment, Environment};
use ink::prelude::{string::String, vec::Vec};
use openbrush::{
    contracts::psp37::Id,
    traits::{AccountId, Timestamp},
};
use scale::{Decode, Encode};

pub type Result<T> = core::result::Result<T, DexError>;
type DefaultAccountId = <DefaultEnvironment as Environment>::AccountId;
type DefaultBalance = <DefaultEnvironment as Environment>::Balance;

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DexError {
    FailIBCCall,
    FailScaleCode,
    UnSupportIdType,
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
}

impl FromStatusCode for DexError {
    fn from_status_code(status_code: u32) -> core::result::Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailIBCCall),
            _ => panic!("encountered unknown status code"),
        }
    }
}

// todo need parse scale error, this is for test
impl From<scale::Error> for DexError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl From<DexError> for String {
    fn from(e: DexError) -> Self {
        match e {
            DexError::FailIBCCall => "FailIBCCall".into(),
            DexError::FailScaleCode => "FailScaleCode".into(),
            DexError::UnSupportIdType => "UnSupportIdType".into(),
            DexError::Custom(info) => info,
        }
    }
}

#[derive(Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OrderType {
    BUY,
    SELL,
}

#[derive(Decode, Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Order {
    counter: u64, //order index
    address: AccountId,
    pair: (Id, Id), //AssetId_1 is base,  AssetId_2 is quote token
    timestamp: u64,
    order_type: OrderType,
    amount_offered: u128,
    amout_requested: u128,
}

#[derive(Decode, Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct WrapperOrder {
    counter: u64, //order index
    address: DefaultAccountId,
    pair: (u32, u32), //AssetId_1 is base,  AssetId_2 is quote token
    timestamp: u64,
    order_type: OrderType,
    amount_offered: u128,
    amout_requested: u128,
}

#[ink::chain_extension]
pub trait DexExtension {
    type ErrorCode = DexError;

    #[ink(extension = 0x50001)]
    fn deposit(asset_id: u32, amount: u128) -> Result<()>;

    #[ink(extension = 0x50002)]
    fn balance_of(owner: DefaultAccountId, asset_id: u32) -> Result<DefaultBalance>;

    #[ink(extension = 0x50003)]
    fn withdraw(asset_id: u32, amount: u128) -> Result<()>;

    #[ink(extension = 0x50004)]
    fn tokens() -> Result<Vec<u32>>;

    #[ink(extension = 0x50005)]
    fn owners_tokens(owner: DefaultAccountId) -> Result<Vec<u32>>;

    #[ink(extension = 0x50006)]
    fn order_for(order_index: u64) -> Result<WrapperOrder>;

    #[ink(extension = 0x50007)]
    fn pair_orders(asset_id_1: u32, sset_id_2: u32) -> Result<Vec<WrapperOrder>>;

    #[ink(extension = 0x50008)]
    fn user_orders(owner: DefaultAccountId) -> Result<Vec<WrapperOrder>>;

    #[ink(extension = 0x50009)]
    fn make_order(
        asset_id_1: u32,
        asset_id_2: u32,
        offered_amount: u128,
        requested_amount: u128,
        order_type: OrderType,
    ) -> Result<()>;

    #[ink(extension = 0x5000a)]
    fn cancel_order(order_index: u64) -> Result<()>;

    #[ink(extension = 0x5000b)]
    fn take_order(order_index: u64) -> Result<()>;

    #[ink(extension = 0x5000c)]
    fn owner_token_by_index(owner: DefaultAccountId, index: u64) -> Result<u32>;

    #[ink(extension = 0x5000d)]
    fn pair_order_by_index(asset_id_1: u32, asset_id_2: u32, index: u64) -> Result<WrapperOrder>;

    #[ink(extension = 0x5000e)]
    fn user_order_by_index(owner: DefaultAccountId, index: u64) -> Result<WrapperOrder>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DexDefaultEnvironment {}

impl Environment for DexDefaultEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = DexExtension;
}

#[ink::contract(env = crate::DexDefaultEnvironment)]
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
        counter: u64,
    }

    #[ink(event)]
    pub struct OrderTaken {
        #[ink(topic)]
        counter: u64,
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
            let id = match asset_id {
                Id::U32(id) => id,
                _ => return Err(DexError::UnSupportIdType),
            };

            let _ = self
                .env()
                .extension()
                .deposit(id, amount)
                .map_err(|e| DexError::Custom(e.into()))?;

            Self::env().emit_event(Deposit {
                asset_id: asset_id,
                amount: amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId, id: Id) -> Balance {
            let id = match id {
                Id::U32(id) => id,
                _ => return Balance::default(),
            };

            let rt = self
                .env()
                .extension()
                .balance_of(owner, id)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(balance) => balance,
                _ => Balance::default(),
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self, asset_id: Id, amount: u128) -> Result<(), DexError> {
            let id = match asset_id {
                Id::U32(id) => id,
                _ => return Err(DexError::UnSupportIdType),
            };

            let _ = self
                .env()
                .extension()
                .withdraw(id, amount)
                .map_err(|e| DexError::Custom(e.into()))?;

            Self::env().emit_event(Deposit {
                asset_id: asset_id,
                amount: amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn tokens(&self) -> Vec<Id> {
            let rt = self
                .env()
                .extension()
                .tokens()
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(ids) => {
                    let mut new_ids = vec![];
                    for id in ids {
                        new_ids.push(Id::U32(id));
                    }
                    new_ids
                }
                _ => vec![],
            }
        }

        #[ink(message)]
        pub fn owners_tokens(&self, owner: AccountId) -> Vec<Id> {
            let rt = self
                .env()
                .extension()
                .owners_tokens(owner)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(ids) => {
                    let mut new_ids = vec![];
                    for id in ids {
                        new_ids.push(Id::U32(id));
                    }
                    new_ids
                }
                _ => return vec![],
            }
        }

        #[ink(message)]
        pub fn order_for(&self, order_index: u64) -> Option<Order> {
            let default_order = Order {
                counter: 0,
                address: AccountId::from([0x00; 32]),
                pair: (Id::U64(0), Id::U64(0)),
                timestamp: self.env().block_timestamp(),
                order_type: OrderType::BUY,
                amount_offered: 0,
                amout_requested: 0,
            };

            let rt = self
                .env()
                .extension()
                .order_for(order_index)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(order) => Some(Order {
                    counter: order.counter,
                    address: order.address,
                    pair: (Id::U32(order.pair.0), Id::U32(order.pair.1)),
                    timestamp: order.timestamp,
                    order_type: order.order_type,
                    amount_offered: order.amount_offered,
                    amout_requested: order.amout_requested,
                }),
                _ => Some(default_order),
            }
        }

        #[ink(message)]
        pub fn pair_orders(&self, asset_id_1: Id, asset_id_2: Id) -> Vec<Order> {
            let rt_default = vec![];

            let asset_id_1 = match asset_id_1 {
                Id::U32(id) => id,
                _ => return rt_default,
            };

            let asset_id_2 = match asset_id_2 {
                Id::U32(id) => id,
                _ => return rt_default,
            };

            let rt = self
                .env()
                .extension()
                .pair_orders(asset_id_1, asset_id_2)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(orders) => {
                    let mut new_orders = vec![];
                    for order in orders {
                        new_orders.push(Order {
                            counter: order.counter,
                            address: order.address,
                            pair: (Id::U32(order.pair.0), Id::U32(order.pair.1)),
                            timestamp: order.timestamp,
                            order_type: order.order_type,
                            amount_offered: order.amount_offered,
                            amout_requested: order.amout_requested,
                        })
                    }
                    new_orders
                }
                _ => rt_default,
            }
        }

        #[ink(message)]
        pub fn user_orders(&self, owner: AccountId) -> Vec<Order> {
            let rt_default = vec![];

            let rt = self
                .env()
                .extension()
                .user_orders(owner)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(orders) => {
                    let mut new_orders = vec![];
                    for order in orders {
                        new_orders.push(Order {
                            counter: order.counter,
                            address: order.address,
                            pair: (Id::U32(order.pair.0), Id::U32(order.pair.1)),
                            timestamp: order.timestamp,
                            order_type: order.order_type,
                            amount_offered: order.amount_offered,
                            amout_requested: order.amout_requested,
                        })
                    }
                    new_orders
                }
                _ => rt_default,
            }
        }

        #[ink(message)]
        pub fn make_order(
            &mut self,
            asset_id_1: Id,
            asset_id_2: Id,
            offered_amount: u128,
            requested_amount: u128,
            order_type: OrderType,
        ) -> Result<(), DexError> {
            let new_asset_id_1 = match asset_id_1 {
                Id::U32(id) => id,
                _ => return Err(DexError::UnSupportIdType),
            };

            let new_asset_id_2 = match asset_id_2 {
                Id::U32(id) => id,
                _ => return Err(DexError::UnSupportIdType),
            };

            let _ = self
                .env()
                .extension()
                .make_order(
                    new_asset_id_1,
                    new_asset_id_2,
                    offered_amount,
                    requested_amount,
                    order_type.clone(),
                )
                .map_err(|e| DexError::Custom(e.into()))?;

            Self::env().emit_event(OrderCreated {
                asset_id_1: asset_id_1,
                asset_id_2: asset_id_2,
                offered_amount: offered_amount,
                requested_amount: requested_amount,
                order_type: order_type,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn cancel_order(&mut self, order_index: u64) -> Result<(), DexError> {
            let _ = self
                .env()
                .extension()
                .cancel_order(order_index)
                .map_err(|e| DexError::Custom(e.into()))?;

            Self::env().emit_event(OrderCanceled {
                counter: order_index,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn take_order(&mut self, order_index: u64) -> Result<(), DexError> {
            let _ = self
                .env()
                .extension()
                .take_order(order_index)
                .map_err(|e| DexError::Custom(e.into()))?;

            Self::env().emit_event(OrderTaken {
                counter: order_index,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn owner_token_by_index(&self, owner: AccountId, index: u64) -> Id {
            let rt = self
                .env()
                .extension()
                .owner_token_by_index(owner, index)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(id) => Id::U32(id),
                _ => Id::default(),
            }
        }

        #[ink(message)]
        pub fn pair_order_by_index(&self, asset_id_1: Id, asset_id_2: Id, index: u64) -> Order {
            let default_order = Order {
                counter: 0,
                address: AccountId::from([0x00; 32]),
                pair: (Id::U64(0), Id::U64(0)),
                timestamp: self.env().block_timestamp(),
                order_type: OrderType::BUY,
                amount_offered: 0,
                amout_requested: 0,
            };

            let asset_id_1 = match asset_id_1 {
                Id::U32(id) => id,
                _ => return default_order,
            };

            let asset_id_2 = match asset_id_2 {
                Id::U32(id) => id,
                _ => return default_order,
            };

            let rt = self
                .env()
                .extension()
                .pair_order_by_index(asset_id_1, asset_id_2, index)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(order) => Order {
                    counter: order.counter,
                    address: order.address,
                    pair: (Id::U32(order.pair.0), Id::U32(order.pair.1)),
                    timestamp: order.timestamp,
                    order_type: order.order_type,
                    amount_offered: order.amount_offered,
                    amout_requested: order.amout_requested,
                },
                _ => default_order,
            }
        }

        #[ink(message)]
        pub fn user_order_by_index(&self, owner: AccountId, index: u64) -> Order {
            let default_order = Order {
                counter: 0,
                address: AccountId::from([0x00; 32]),
                pair: (Id::U64(0), Id::U64(0)),
                timestamp: 0,
                order_type: OrderType::BUY,
                amount_offered: 0,
                amout_requested: 0,
            };

            let rt = self
                .env()
                .extension()
                .user_order_by_index(owner, index)
                .map_err(|e| DexError::Custom(e.into()));

            match rt {
                Ok(order) => Order {
                    counter: order.counter,
                    address: order.address,
                    pair: (Id::U32(order.pair.0), Id::U32(order.pair.1)),
                    timestamp: order.timestamp,
                    order_type: order.order_type,
                    amount_offered: order.amount_offered,
                    amout_requested: order.amout_requested,
                },
                _ => default_order,
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
