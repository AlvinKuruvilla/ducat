use ark_crypto_primitives::crh::{
    sha256::constraints::{Sha256Gadget, UnitVar},
    CRHSchemeGadget,
};
use ark_ff::PrimeField;
use ark_r1cs_std::{fields::fp::FpVar, ToBytesGadget, ToConstraintFieldGadget};

#[derive(Clone, Debug)]
pub struct Address {
    public_key: String,
    secret_key: String,
}
/// According to the Zcash paper, this is the address generation procedure. Hash the secret key to generate the pairs (pk, sk)
impl Address {
    pub fn new(secret_key: String) -> Self {
        Self {
            public_key: secret_key.clone(),
            secret_key,
        }
        // let mut holder = vec![];
        // let unit_var: UnitVar<F> = UnitVar::default();
        // holder.extend_from_slice(&secret_key.to_bytes().unwrap());

        // let sn = Sha256Gadget::evaluate(&unit_var, &holder)
        //     .unwrap()
        //     .0
        //     .to_constraint_field()
        //     .unwrap()[0]
        //     .to_owned();
        // Self {
        //     public_key: sn,
        //     secret_key,
        // }
    }
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }
    pub fn secret_key(&self) -> String {
        self.secret_key.clone()
    }
}
