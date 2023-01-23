#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::wrapper]
pub type BaseRef = dyn Base;

// use ink_lang as ink;
// #[ink::trait_definition]
#[openbrush::trait_definition]
pub trait Base {
    #[ink(message)]
    fn execute_function(&mut self)
        -> Result<(), ()>;
}