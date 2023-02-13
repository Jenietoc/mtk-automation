// Module to obtain the identifier of an account by ID
pub mod identifier {
    use soroban_auth::Identifier;
    use soroban_sdk::AccountId;

    pub fn get_account_identifier(account_id: AccountId) -> Identifier {
        Identifier::Account(account_id)
    }
}