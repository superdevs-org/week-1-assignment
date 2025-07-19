Issue 3727

seeds = pda_seed(signer.key) // Not allowed in rust

seeds = [b"prefix", signer.key().as_ref()] // We have to change like this 

OR
we can create our own macros like

macro_rules! my_seeds {
    ($key:expr) => {
        [b"prefix", #key.as_ref()]
    }
}

and then we can use as 
#[account(seed = my_seed(...))]