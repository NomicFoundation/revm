use crate::{db::Database, Block, SpecId, Transaction};
use core::{fmt::Debug, hash::Hash};

/// The type that enumerates the chain's hardforks.
pub trait HardforkTrait: Clone + Copy + Default + PartialEq + Eq + Into<SpecId> {}

impl<HardforkT> HardforkTrait for HardforkT where
    HardforkT: Clone + Copy + Default + PartialEq + Eq + Into<SpecId>
{
}

pub trait HaltReasonTrait: Clone + Debug + PartialEq + Eq + From<crate::HaltReason> {}

impl<HaltReasonT> HaltReasonTrait for HaltReasonT where
    HaltReasonT: Clone + Debug + PartialEq + Eq + From<crate::HaltReason>
{
}

pub trait TransactionValidation {
    /// An error that occurs when validating a transaction.
    type ValidationError: Debug + core::error::Error;
}

pub trait ChainSpec: Sized {
    /// Chain context type.
    type ChainContext: Sized + Default + Debug;

    /// The type that contains all block information.
    type Block: Block;

    /// The type that contains all transaction information.
    type Transaction: Transaction + TransactionValidation;

    /// The type that enumerates the chain's hardforks.
    type Hardfork: HardforkTrait;

    /// Halt reason type.
    type HaltReason: HaltReasonTrait;
}

pub trait EvmWiring: ChainSpec + Sized {
    /// Chain specification type.
    type ChainSpec: ChainSpec;

    /// External context type
    type ExternalContext: Sized;

    /// Database type.
    type Database: Database;
}

impl<EvmWiringT: EvmWiring> ChainSpec for EvmWiringT {
    type ChainContext = <EvmWiringT::ChainSpec as ChainSpec>::ChainContext;
    type Block = <EvmWiringT::ChainSpec as ChainSpec>::Block;
    type Transaction = <EvmWiringT::ChainSpec as ChainSpec>::Transaction;
    type Hardfork = <EvmWiringT::ChainSpec as ChainSpec>::Hardfork;
    type HaltReason = <EvmWiringT::ChainSpec as ChainSpec>::HaltReason;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EthereumChainSpec;

impl ChainSpec for EthereumChainSpec {
    type ChainContext = ();
    type Block = crate::BlockEnv;
    type Transaction = crate::TxEnv;
    type Hardfork = SpecId;
    type HaltReason = crate::HaltReason;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EthereumWiring<DB: Database, EXT> {
    phantom: core::marker::PhantomData<(DB, EXT)>,
}

impl<DB: Database, EXT: Debug> EvmWiring for EthereumWiring<DB, EXT> {
    type ChainSpec = EthereumChainSpec;
    type Database = DB;
    type ExternalContext = EXT;
}

pub type DefaultEthereumWiring = EthereumWiring<crate::db::EmptyDB, ()>;
