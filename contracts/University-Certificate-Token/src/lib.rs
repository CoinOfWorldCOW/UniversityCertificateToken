#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, Map, symbol_short};

#[contract]
pub struct CertificateContract;

const CERTS: Symbol = symbol_short!("CERTS");

#[contractimpl]
impl CertificateContract {
    // Issue certificate (only issuer can call)
    pub fn issue(env: Env, issuer: Address, student: Address, cert_hash: Symbol) {
        issuer.require_auth(); // 🔐 only issuer allowed

        let mut certs: Map<Address, Symbol> =
            env.storage()
                .instance()
                .get(&CERTS)
                .unwrap_or(Map::new(&env));

        certs.set(student, cert_hash);

        env.storage().instance().set(&CERTS, &certs);
    }

    // Verify certificate
    pub fn verify(env: Env, student: Address) -> Option<Symbol> {
        let certs: Map<Address, Symbol> =
            env.storage()
                .instance()
                .get(&CERTS)
                .unwrap_or(Map::new(&env));

        certs.get(student)
    }
}

mod test;
