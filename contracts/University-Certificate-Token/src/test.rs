#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, Symbol};

#[test]
fn test_issue_and_verify_certificate() {
    let env = Env::default();
    env.mock_all_auths();

    // Create contract
    let contract_id = env.register_contract(None, CertificateContract);

    // Create test addresses
    let issuer = Address::generate(&env);
    let student = Address::generate(&env);

    // Certificate hash
    let cert_hash = Symbol::short("CERT123");

    // Issue certificate
    CertificateContract::client(&env)
        .issue(&issuer, &student, &cert_hash);

    // Verify certificate
    let result = CertificateContract::client(&env)
        .verify(&student);

    // Assert
    assert_eq!(result, Some(cert_hash));
}