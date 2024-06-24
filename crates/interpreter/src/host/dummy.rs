use derive_where::derive_where;

use crate::{
    primitives::{
        hash_map::Entry, Address, Bytes, ChainSpec, Env, HashMap, Log, B256, KECCAK_EMPTY, U256,
    },
    Host, SStoreResult, SelfDestructResult,
};
use std::vec::Vec;

use super::LoadAccountResult;

/// A dummy [Host] implementation.
#[derive_where(Clone, Debug, Default; ChainSpecT::Block, ChainSpecT::Transaction)]
pub struct DummyHost<ChainSpecT>
where
    ChainSpecT: ChainSpec,
{
    pub env: Env<ChainSpecT>,
    pub storage: HashMap<U256, U256>,
    pub transient_storage: HashMap<U256, U256>,
    pub log: Vec<Log>,
}

impl<ChainSpecT> DummyHost<ChainSpecT>
where
    ChainSpecT: ChainSpec,
{
    /// Create a new dummy host with the given [`Env`].
    #[inline]
    pub fn new(env: Env<ChainSpecT>) -> Self {
        Self {
            env,
            storage: HashMap::new(),
            transient_storage: HashMap::new(),
            log: Vec::new(),
        }
    }

    /// Clears the storage and logs of the dummy host.
    #[inline]
    pub fn clear(&mut self) {
        self.storage.clear();
        self.log.clear();
    }
}

impl<ChainSpecT> Host for DummyHost<ChainSpecT>
where
    ChainSpecT: ChainSpec,
{
    type ChainSpecT = ChainSpecT;

    #[inline]
    fn env(&self) -> &Env<ChainSpecT> {
        &self.env
    }

    #[inline]
    fn env_mut(&mut self) -> &mut Env<ChainSpecT> {
        &mut self.env
    }

    #[inline]
    fn load_account(&mut self, _address: Address) -> Option<LoadAccountResult> {
        Some(LoadAccountResult::default())
    }

    #[inline]
    fn block_hash(&mut self, _number: u64) -> Option<B256> {
        Some(B256::ZERO)
    }

    #[inline]
    fn balance(&mut self, _address: Address) -> Option<(U256, bool)> {
        Some((U256::ZERO, false))
    }

    #[inline]
    fn code(&mut self, _address: Address) -> Option<(Bytes, bool)> {
        Some((Bytes::default(), false))
    }

    #[inline]
    fn code_hash(&mut self, __address: Address) -> Option<(B256, bool)> {
        Some((KECCAK_EMPTY, false))
    }

    #[inline]
    fn sload(&mut self, __address: Address, index: U256) -> Option<(U256, bool)> {
        match self.storage.entry(index) {
            Entry::Occupied(entry) => Some((*entry.get(), false)),
            Entry::Vacant(entry) => {
                entry.insert(U256::ZERO);
                Some((U256::ZERO, true))
            }
        }
    }

    #[inline]
    fn sstore(&mut self, _address: Address, index: U256, value: U256) -> Option<SStoreResult> {
        let (present, is_cold) = match self.storage.entry(index) {
            Entry::Occupied(mut entry) => (entry.insert(value), false),
            Entry::Vacant(entry) => {
                entry.insert(value);
                (U256::ZERO, true)
            }
        };

        Some(SStoreResult {
            original_value: U256::ZERO,
            present_value: present,
            new_value: value,
            is_cold,
        })
    }

    #[inline]
    fn tload(&mut self, _address: Address, index: U256) -> U256 {
        self.transient_storage
            .get(&index)
            .copied()
            .unwrap_or_default()
    }

    #[inline]
    fn tstore(&mut self, _address: Address, index: U256, value: U256) {
        self.transient_storage.insert(index, value);
    }

    #[inline]
    fn log(&mut self, log: Log) {
        self.log.push(log)
    }

    #[inline]
    fn selfdestruct(&mut self, _address: Address, _target: Address) -> Option<SelfDestructResult> {
        panic!("Selfdestruct is not supported for this host")
    }
}
