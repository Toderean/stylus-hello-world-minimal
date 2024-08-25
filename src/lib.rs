#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

mod erc20;

use alloy_primitives::U256;
use alloy_primitives::Address as Address;
use stylus_sdk::alloy_primitives;
use stylus_sdk::{msg, prelude::*, storage::StorageAddress};

/// initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

pub struct EmoryaTokenParams;
impl erc20::ERC20Params for EmoryaTokenParams {
    const NAME: &'static str = "Emorya Finance";
    const SYMBOL: &'static str = "EMR";
    const DECIMALS: u8 = 9;
}

// define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    // #[entrypoint]
    pub struct EmoryaToken {
        // allows erc20 to access StylusToken's storage and make calls
        #[borrow]
        address owner;
        erc20::Erc20<EmoryaTokenParams> erc20;
    }
}

impl EmoryaToken {
    pub fn new() -> Self {
        let owner = msg::sender();
        let slot = U256::from(0);
        let offset = 0;
        let mut owner_address: StorageAddress = unsafe { StorageAddress::new(slot, offset) };
        owner_address.set(owner);
        Self {
            erc20: unsafe { erc20::Erc20::new(slot, offset) },
            owner: owner_address,
        }
    }

    /// Gets the owner address
    pub fn owner(&self) -> Address {
        self.owner.get()
    }

    /// Internal function to check if the caller is the owner
    fn only_owner(&self) -> Result<(), &'static str> {
        if msg::sender() != self.owner.get() {
            return Err("Caller is not the owner");
        }
        Ok(())
    }

    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), &'static str> {
        self.only_owner()?;
        self.owner.set(new_owner);
        Ok(())
    }

    /// mints tokens
    pub fn mint(&mut self, user_address: Address, amount: U256) -> Result<(), &'static str> {
        self.only_owner()?;
        self.erc20.mint(user_address, amount)?;
        Ok(())
    }

    /// burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), &'static str> {
        self.only_owner()?;
        self.erc20.burn(msg::sender(), value)?;
        Ok(())
    }
}
