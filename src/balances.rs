use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};


pub trait Config : crate::system::Config{
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T : Config> {
    balances: BTreeMap<T::AccountId,T::Balance>,
}


pub enum Call<T: Config> {
    RemoveMe(core::marker::PhantomData<T>),
}

impl<T:Config> Pallet<T> 
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance){
        self.balances.insert(who.clone(),amount);
    }

 
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
        let mut callerBalance = self.balance(&caller);
        let mut toBalance = self.balance(&to);
        let mut new_caller_balance = callerBalance.checked_sub(&amount).ok_or("Not enough funds")?;
        let mut new_to_balance = toBalance.checked_add(&amount).ok_or("Overflow Occured")?;

        self.set_balance(&caller,new_caller_balance);
        self.set_balance(&to,new_to_balance);

        Ok(())
    }
}


impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller : Self::Caller, call: Self::Call) -> crate::support::DispatchResult   {
        Ok(())
    }

}


#[cfg(test)]
mod tests {

    
    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u64;
    }
    #[test]
    fn init_balances() {

        let mut balances = super::Pallet::<TestConfig>::new();
        assert_eq!(balances.balance(&"alice".to_string()),0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()),0);
    }


    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();
        balances.set_balance(&"alice".to_string(), 0);
        balances.set_balance(&"bob".to_string(), 0);
        assert_eq!(balances.transfer("alice".to_string(),"bob".to_string(), 10), Err("Not enough funds"));

        balances.set_balance(&"alice".to_string(), 10);
        balances.transfer("alice".to_string(),"bob".to_string(), 10);
        assert_eq!(balances.balance(&"bob".to_string()), 10);
        assert_eq!(balances.balance(&"alice".to_string()), 0)
    }
}