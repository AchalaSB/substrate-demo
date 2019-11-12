/*
use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
decl_storage! {
    trait Store for Module<T: Trait> as calculator {
        A get(a): u64=5;
        B get(b): u64=5;
        BalanceOf get(balance_of): map T::AccountId => u64;
    }
}
decl_module! {
    pub struct Module <T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;    //This is default deposit_event definition and it is used only if you are using events in your module i.e. decl_events
        
        fn init(origin) -> Result {
            let a = ensure_signed(origin)?;    //Check for initialization
            <BalanceOf<T>>::insert(a.clone(), Self::a());
            let b = ensure_signed(_origin)?;
            <BalanceOf<T>>::insert(b.clone(), Self::b());
            Ok(())
        }
        fn addition(origin1, to: T::AccountId, _to: T::AccountId, _1to: T::AccountId, value: u64) -> Result {
            let a = ensure_signed(origin1)?;
            let var1 = Self::balance_of(to.clone());
            //let b = ensure_signed(_origin1)?;
            let var2 = Self::balance_of(_to.clone());
            <BalanceOf<T>>::insert(to.clone(), var1); 
            <BalanceOf<T>>::insert(_to.clone(), var2); 
              //Deduct sender's balance
            //<BalanceOf<T>>::insert(to.clone(), updated_to_balance); // Increase receiver's balance
            //<BalanceOf<T>>::insert(_to.clone(), updated_to_escrow_balance);  //Increase escrow balance
            let c = a+b;
            //c = Self::balance_of(_1to.clone());
            let var3= Self::balance_of(_1to.clone(), var3);
            <Balance<T>>::insert(_1to.clone(),var3);
            Ok(())
        }
    }
}
decl_event! (
    pub enum Event<T> where <T as system::Trait>::AccountId {
        Addition(AccountId, AccountId, AccountId, u64),
    }
);
*/
use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;
extern crate integer_sqrt;
use crate::calculator::integer_sqrt::IntegerSquareRoot;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
decl_storage! {
    trait Store for Module<T: Trait> as calculator {
        FinalResult: u64;
    }
}
decl_module! {
    pub struct Module <T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default; //This is default deposit_event definition and it is used only if you are using events in your module i.e. decl_events
        fn add(origin, a: u64, b: u64) -> Result {
            // We use this to make sure that this is a signed message
            // and that a user will be charged a transaction fee.
            let sender = ensure_signed(origin)?;
            let result = a.checked_add(b).ok_or("Numbers are too large to store in a u64")?;
            <FinalResult<T>>::put(result);
            Self::deposit_event(RawEvent::Addition(sender, a, b, result));
            Ok(())
        }
        fn sub(origin, a: u64, b: u64) -> Result {
            // We use this to make sure that this is a signed message
            // and that a user will be charged a transaction fee.
            let sender = ensure_signed(origin)?;
            let result = a.checked_sub(b).ok_or("Numbers are too large to store in a u64")?;
            <FinalResult<T>>::put(result);
            Self::deposit_event(RawEvent::Addition(sender, a, b, result));
            Ok(())
        }
        fn mul(origin, a: u64, b: u64) -> Result {
            // We use this to make sure that this is a signed message
            // and that a user will be charged a transaction fee.
            let sender = ensure_signed(origin)?;
            let result = a.checked_mul(b).ok_or("Numbers are too large to store in a u64")?;
            <FinalResult<T>>::put(result);
            Self::deposit_event(RawEvent::Multiplication(sender, a, b, result));
            Ok(())
        }
        fn div(origin, a: u64, b: u64) -> Result {
            // We use this to make sure that this is a signed message
            // and that a user will be charged a transaction fee.
            let sender = ensure_signed(origin)?;
            let result = a.checked_div(b).ok_or("Numbers are too large to store in a u64")?;
            <FinalResult<T>>::put(result);
            Self::deposit_event(RawEvent::Division(sender, a, b, result));
            Ok(())
        }
        fn sqrt(origin, a: u64) -> Result {
            // We use this to make sure that this is a signed message
            // and that a user will be charged a transaction fee.
            let sender = ensure_signed(origin)?;
            let result = a.integer_sqrt();
            <FinalResult<T>>::put(result);
            Self::deposit_event(RawEvent::Squareroot(sender, a, result));
            Ok(())
        }
    }
}
decl_event! (
    pub enum Event<T> where <T as system::Trait>::AccountId {
        Addition(AccountId, u64, u64, u64),
        Substraction(AccountId, u64, u64, u64),
        Multiplication(AccountId, u64, u64, u64),
        Division(AccountId, u64, u64, u64),
        Squareroot(AccountId, u64, u64),
    }
);