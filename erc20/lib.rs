#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    use scale::{Decode, Encode};
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }
    type Result<T> = core::result::Result<T, Error>;
    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            balances.insert(Self::env().caller(), &total_supply);
            Self {
                total_supply,
                balances,
                ..Default::default()
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }
        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.get_balance(&who)
        }
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.get_allowance(&owner, &spender)
        }
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();

            self.allowances.insert(&(caller, spender), &value);
            self.env().emit_event(Approval {
                owner: caller,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            self.transfer_impl(&caller, &to, value)
        }
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.get_allowance(&from, &caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.allowances
                .insert(&(from, caller), &(allowance - value));

            self.transfer_impl(&from, &to, value)
        }
        #[inline]
        fn get_balance(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }
        #[inline]
        fn get_allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        fn transfer_impl(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);
            if value > balance_from {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));
            self.env().emit_event(Transfer {
                from: *from,
                to: *to,
                value,
            });
            Ok(())
        }
    }
}
