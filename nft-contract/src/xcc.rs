use near_sdk::{
    env, AccountId, Promise, Gas, 
};

use crate::Hello;
use crate::external::{hello_near, TGAS};




impl hello Contract{

    pub fn hello(hello_account: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
          hello_account,
        }
      }
    

    
    pub fn query_greeting(&self) -> Promise {
            // Create a promise to call HelloNEAR.get_greeting()
            let promise = hello_near::ext(self.hello_account.clone())
              .with_static_gas(Gas(5*TGAS))
              .get_greeting();
            
            return promise
            // .then( // Create a promise to callback query_greeting_callback
            //   Self::ext(env::current_account_id())
            //   .with_static_gas(Gas(5*TGAS))
            //   .query_greeting_callback()
            // )
          }
        
}
    