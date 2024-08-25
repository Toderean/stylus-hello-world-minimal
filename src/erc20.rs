extern crate alloc;
use alloy_primitives::{Address, U256};
use core::marker::PhantomData;
use stylus_sdk::{
    alloy_primitives, msg, prelude::* 
};

/// Using Stylus and ERC-20
pub trait ERC20Params{
    /// Immutable token name
    const NAME: &'static str;

    /// Immutable token  symbol
    const SYMBOL: &'static str;

    /// Immutable token decimals
    const DECIMALS: u8;
}


sol_storage!{
    ///ERC20 implements all ERC-20 methods
    pub struct Erc20<T: ERC20Params> {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
        PhantomData<T> phantom;
    }
}

impl<T: ERC20Params> Erc20<T> {
    /// Immutable token name
    pub fn name() -> String {
        T::NAME.into()
    }

    /// Immutable token symbol
    pub fn symbol() -> String {
        T::SYMBOL.into()
    }

    /// Immutable token decimals
    pub fn decimals() -> u8 {
        T::DECIMALS
    }

    /// Total supply of tokens
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    /// Balance of `address`
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    pub fn _transfer(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), &'static str> {
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        
        if old_sender_balance < amount {
            return Err("Insufficient balance");
        }
        
        sender_balance.set(old_sender_balance - amount);

        let mut receiver_balance = self.balances.setter(to);
        let new_receiver_balance = receiver_balance.get() + amount;
        receiver_balance.set(new_receiver_balance);

        self.log_transfer_event(from, to, amount);
        Ok(())
    }
    
    pub fn mint(&mut self, address: Address, amount: U256) -> Result<(), &'static str> {
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + amount;
        balance.set(new_balance);

        self.total_supply.set(self.total_supply.get() + amount);

        self.log_transfer_event(Address::ZERO, address, amount);

        Ok(())
    }
    
    pub fn burn(&mut self, address: Address, amount: U256) -> Result<(), &'static str> {
        let mut balance = self.balances.setter(address);
        let old_balance = balance.get();
        if old_balance < amount {
            return Err("Insufficient balance");
        }
        balance.set(old_balance - amount);

        self.total_supply.set(self.total_supply.get() - amount);

        self.log_transfer_event(address, Address::ZERO, amount);

        Ok(())
    }
    
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, &'static str> {
        self._transfer(msg::sender(), to, value)?;
        Ok(true)
    }

    fn log_transfer_event(&self, from: Address, to: Address, amount: U256) {
        println!("Transfer Event: from: {:?}, to: {:?}, amount: {:?}", from, to, amount);
    }
}


