use ark_ff::PrimeField;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar, R1CSVar};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef};

use super::transaction::Transaction;

pub struct Organization<F: PrimeField> {
    // TODO: Change the balance counters to u32 and keep a flag for each of those whether or not they are negative
    spent_serial_numbers: Vec<FpVar<F>>, // TODO: When using this we should have sanity checks that panic if repeats exist
    used_address_public_keys: Vec<String>, // TODO: When using this we should have sanity checks that panic if repeats exist
    transaction_root_cache: Vec<FpVar<F>>,
    unique_identifier: String,
    _initial_balance: i32,
    final_balance: i32,
    epoch_balance_delta: i32,
}
impl<F> Organization<F>
where
    F: PrimeField,
{
    pub fn new(
        unique_identifier: String,
        initial_balance: i32,
        known_addresses: Vec<String>,
    ) -> Self {
        Self {
            spent_serial_numbers: Vec::new(),
            used_address_public_keys: known_addresses,
            unique_identifier,
            transaction_root_cache: Vec::new(),
            // NOTE: this field should not be directly accessed or mutated. There is a getter provided to retrieve the value
            _initial_balance: initial_balance,
            final_balance: initial_balance,
            epoch_balance_delta: 0,
        }
    }
    pub fn add_address_public_key(&mut self, address_public_key: String) {
        self.used_address_public_keys.push(address_public_key);
    }
    pub fn add_serial_number(&mut self, sn: FpVar<F>) {
        self.spent_serial_numbers.push(sn);
    }
    pub fn add_root(&mut self, root: FpVar<F>) {
        self.transaction_root_cache.push(root);
    }
    pub fn clear_delta(&mut self) {
        self.epoch_balance_delta = 0;
    }
    pub fn delta(&self) -> i32 {
        self.epoch_balance_delta
    }
    pub fn final_balance(&self) -> i32 {
        self.final_balance
    }
    pub fn initial_balance(&self) -> i32 {
        self._initial_balance
    }
    pub fn update_delta(&mut self, value: i32) {
        self.epoch_balance_delta += value;
    }
    pub fn update_balance(&mut self, epoch_delta: i32) {
        self.final_balance += epoch_delta;
    }
    pub fn identifier(&self) -> String {
        self.unique_identifier.clone()
    }
    pub fn dump_info(&self) {
        println!("Organization: {}", self.unique_identifier);
        println!("========================");
        println!("Balance: {}", self.final_balance());
        println!("Epoch Delta: {}", self.epoch_balance_delta);
        println!("Known Addresses: {:?}", self.used_address_public_keys);
        println!()
    }
    pub fn has_address(&self, address_public_key: String) -> bool {
        self.used_address_public_keys
            .iter()
            .any(|key| *key == address_public_key)
    }

    pub fn is_involved(&self, t: &Transaction<F>) -> bool {
        self.has_address(t.sender_address().public_key())
            || self.has_address(t.receiver_address().public_key())
    }
    pub fn validate_transaction_serial_numbers(&self, blockchain_serial_numbers: Vec<F>) -> bool {
        let cs = ConstraintSystem::<F>::new_ref();
        self.spent_serial_numbers.iter().all(|serial| {
            blockchain_serial_numbers.iter().any(|blockchain_serial| {
                let blockchain_serial_var =
                    FpVar::new_input(cs.clone(), || Ok(blockchain_serial)).unwrap();
                serial
                    .is_eq(&blockchain_serial_var)
                    .unwrap()
                    .value()
                    .unwrap_or(false)
            })
        })
    }
    pub fn validate_transaction_roots(&self, blockchain_transaction_roots: Vec<F>) -> bool {
        let cs = ConstraintSystem::<F>::new_ref();
        self.transaction_root_cache.iter().all(|t_root| {
            blockchain_transaction_roots
                .iter()
                .any(|blockchan_transaction_root| {
                    let blockchain_t_root_var =
                        FpVar::new_input(cs.clone(), || Ok(blockchan_transaction_root)).unwrap();
                    t_root
                        .is_eq(&blockchain_t_root_var)
                        .unwrap()
                        .value()
                        .unwrap_or(false)
                })
        })
    }
    pub fn create_known_addresses(
        cs: ConstraintSystemRef<F>,
        num_addresses: usize,
        offset: usize,
    ) -> Vec<String> {
        let mut addresses = Vec::new();
        for i in 0..num_addresses {
            let address: String = (i + offset).to_string();
            // let address = FpVar::new_input(cs.clone(), || Ok(F::from(i as u64))).unwrap();
            addresses.push(address);
        }
        addresses
    }
}
