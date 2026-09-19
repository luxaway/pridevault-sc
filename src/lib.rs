#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait PrideVault {
    #[init]
    fn init(&self, collection_id: TokenIdentifier) {
        require!(collection_id.is_valid_esdt_identifier(), "invalid collection id");
        self.collection_id().set(collection_id);
        self.is_paused().set(false);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(setCollection)]
    fn set_collection(&self, collection_id: TokenIdentifier) {
        require!(collection_id.is_valid_esdt_identifier(), "invalid collection id");
        self.collection_id().set(collection_id);
    }

    #[only_owner]
    #[endpoint(pause)]
    fn pause(&self) {
        self.is_paused().set(true);
    }

    #[only_owner]
    #[endpoint(unpause)]
    fn unpause(&self) {
        self.is_paused().set(false);
    }

    /// Stake one NFT / SFT from the configured collection.
    /// Caller must send exactly one payment matching collection_id.
    #[payable]
    #[endpoint(stake)]
    fn stake(&self) {
        self.require_not_paused();

        let payment = self.call_value().single_esdt();
        let collection = self.collection_id().get();
        require!(payment.token_identifier == collection, "wrong collection");
        require!(payment.amount > 0, "zero amount");

        let caller = self.blockchain().get_caller();
        let nonce = payment.token_nonce;

        let already = self.staked_amount(&caller, nonce).get();
        self.staked_amount(&caller, nonce)
            .set(&already + &payment.amount);

        let _ = self.staked_nonces(&caller).insert(nonce);
        self.total_staked().update(|t| *t += &payment.amount);

        self.stake_event(&caller, &payment.token_identifier, nonce, &payment.amount);
    }

    /// Unstake a given nonce / amount back to the caller.
    #[endpoint(unstake)]
    fn unstake(&self, nonce: u64, amount: BigUint) {
        self.require_not_paused();
        require!(amount > 0, "zero amount");

        let caller = self.blockchain().get_caller();
        let staked = self.staked_amount(&caller, nonce).get();
        require!(staked >= amount, "not enough staked");

        let remaining = &staked - &amount;
        if remaining == 0 {
            self.staked_amount(&caller, nonce).clear();
            let _ = self.staked_nonces(&caller).swap_remove(&nonce);
        } else {
            self.staked_amount(&caller, nonce).set(&remaining);
        }

        self.total_staked().update(|t| *t -= &amount);

        let collection = self.collection_id().get();
        self.tx()
            .to(&caller)
            .single_esdt(&collection, nonce, &amount)
            .transfer();

        self.unstake_event(&caller, &collection, nonce, &amount);
    }

    #[view(getCollectionId)]
    #[storage_mapper("collectionId")]
    fn collection_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(isPaused)]
    #[storage_mapper("isPaused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

    #[view(getTotalStaked)]
    #[storage_mapper("totalStaked")]
    fn total_staked(&self) -> SingleValueMapper<BigUint>;

    #[view(getStakedAmount)]
    #[storage_mapper("stakedAmount")]
    fn staked_amount(&self, user: &ManagedAddress, nonce: u64) -> SingleValueMapper<BigUint>;

    #[view(getStakedNonces)]
    #[storage_mapper("stakedNonces")]
    fn staked_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[event("stake")]
    fn stake_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] token_id: &TokenIdentifier,
        #[indexed] nonce: u64,
        amount: &BigUint,
    );

    #[event("unstake")]
    fn unstake_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] token_id: &TokenIdentifier,
        #[indexed] nonce: u64,
        amount: &BigUint,
    );

    fn require_not_paused(&self) {
        require!(!self.is_paused().get(), "paused");
    }
}
