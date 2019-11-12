
use support::{dispatch::Result, StorageValue, StorageMap, decl_storage, decl_module, decl_event, ensure};
use system::ensure_signed;
extern crate integer_sqrt;


// the module configuration trait
pub trait Trait: system::Trait {
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Token {
    pub TotalSupply get(total_supply): u64 = 50000000;
    pub BalanceOf get(balance_of): map T::AccountId => u64;
    Init get(is_init): bool;
  }
}

//module declaration
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
	// this is needed only if you are using events in your module
      fn deposit_event<T>() = default;
      

      // initialize the token
      // transfers the total_supply amout to the caller
      fn init(origin) -> Result {
          let sender = ensure_signed(origin)?;
          ensure!(Self::is_init() == false, "Already initialized.");
          <BalanceOf<T>>::insert(sender.clone(), Self::total_supply());
          <Init<T>>::put(true);
          Ok(())
      }



      // transfer tokens from one account to another
      fn transfer(_origin, to: T::AccountId, value: u64) -> Result {
        let sender = ensure_signed(_origin)?;
        let sender_balance = Self::balance_of(sender.clone());
       	let updated_from_balance = sender_balance.checked_sub(value).ok_or("")?;

        let receiver_balance = Self::balance_of(to.clone());
        let updated_to_balance = receiver_balance.checked_add(value).ok_or("")?;

        <BalanceOf<T>>::insert(sender.clone(), updated_from_balance);
        <BalanceOf<T>>::insert(to.clone(), updated_to_balance);

        Self::deposit_event(RawEvent::Transfer(sender, to, value));
        
        Ok(())
      }
  /*    
    //  transfer tokens from one account to another with condition
      fn simple_condition(_origin, to: T::AccountId, value: u64) -> Result {
        let sender = ensure_signed(_origin)?;        
        let sender_balance = Self::balance_of(sender.clone());
		let receiver_balance = Self::balance_of(to.clone());

        if sender_balance <1000
          {
            //transfer tokens only if is less than 1000
            let updated_from_balance = sender_balance.checked_sub(value).ok_or("")?;
            let receiver_balance = Self::balance_of(to.clone());
            let updated_to_balance = receiver_balance.checked_add(value).ok_or("")?;
         
            <BalanceOf<T>>::insert(sender.clone(), updated_from_balance);
            <BalanceOf<T>>::insert(to.clone(), updated_to_balance);
            Self::deposit_event(RawEvent::Transfer(sender, to, value));
        
		  } else if sender_balance >= 1000 {
            //transfer tokens to sender if it is more than 1000
            let updated_from_balance = sender_balance.checked_sub(value-(value-1000)).ok_or("")?;
            //let sender_balance = Self::balance_of(sender.clone());
		    //let receiver_balance = Self::balance_of(to.clone());
            let updated_to_balance = receiver_balance.checked_add(value-(value-1000)).ok_or("")?;
         
            <BalanceOf<T>>::insert(sender.clone(), updated_from_balance);
            <BalanceOf<T>>::insert(to.clone(), updated_to_balance);
            Self::deposit_event(RawEvent::Transfer(sender, to, value));     
        }
        Ok(())
      }
*/


	      //  transfer tokens from one account to another with condition
      fn simple_condition(_origin, to: T::AccountId, value: u64) -> Result {
        let sender = ensure_signed(_origin)?;        
        let sender_balance = Self::balance_of(sender.clone());
		// let receiver_balance = Self::balance_of(to.clone());
		// let escrow_balance = Self::balance_of(to.clone());

        if sender_balance % 1000 == 0
          {
            //transfer tokens only if is less than 1000
            let updated_from_balance = sender_balance.checked_sub(value).ok_or("")?;
            let receiver_balance = Self::balance_of(to.clone());
            let updated_to_balance = receiver_balance.checked_add(value).ok_or("")?;
			//let updated_to_escrow_balance = escrow_balance.checked_add(value).ok_or("")?;
            <BalanceOf<T>>::insert(sender.clone(), updated_from_balance);
            <BalanceOf<T>>::insert(to.clone(), updated_to_balance);
			//<BalanceOf<T>>::insert(to.clone(), updated_to_escrow_balance);
            Self::deposit_event(RawEvent::Transfer(sender, to, value));
        
		  } else if sender_balance % 1000 != 0 {
            //transfer tokens to sender if it is more than 1000
            let updated_from_balance = sender_balance.checked_sub(value).ok_or("")?;
            //let sender_balance = Self::balance_of(sender.clone());
		    let receiver_balance = Self::balance_of(to.clone());
            let updated_to_balance = receiver_balance.checked_add(value-(value % 1000)).ok_or("")?;

			//let updated_to_escrow_balance = escrow_balance.checked_add(value % 1000).ok_or("")?;
         
            <BalanceOf<T>>::insert(sender.clone(), updated_from_balance);
            <BalanceOf<T>>::insert(to.clone(), updated_to_balance);
			//<BalanceOf<T>>::insert(to.clone(), updated_to_escrow_balance);
            Self::deposit_event(RawEvent::Transfer(sender, to, value));     
        }
        Ok(())
      }

  }
}


// events
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        Transfer(AccountId, AccountId, u64),
    }
);


