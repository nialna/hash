use std::mem::zeroed;
use crate::datastore::storage::BufferChange;
use crate::datastore::{Error, Result};

// Simple way for every component (language-runner + engine)
// using the datastore to track whether it has to reload memory
// or reload the record batch

#[must_use]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Metaversion {
    memory: u32,
    batch: u32,
}

impl Metaversion {
    pub fn new(memory: u32, batch: u32) -> Result<Metaversion> {
        if !(batch >= memory) {
            Err(Error::from(
                "Batch is updated when memory is updated, so batch version must be at least memory version"
            ))
        } else {
            Ok(Metaversion { memory, batch })
        }
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        let mut bytes = [0; 8];
        let memory_version = self.memory.to_le_bytes();
        let batch_version = self.batch.to_le_bytes();
        bytes[..memory_version.len()].copy_from_slice(&memory_version);
        bytes[memory_version.len()..].copy_from_slice(&batch_version);
        bytes
    }

    #[must_use]
    pub fn memory(&self) -> u32 {
        self.memory
    }

    #[must_use]
    pub fn batch(&self) -> u32 {
        self.batch
    }

    /// Assert invariants, given that `version` is a metaversion of
    /// *the same batch* as `self`.
    fn verify(&self, version: &Metaversion) {
        debug_assert!(self.batch >= self.memory);
        debug_assert!(version.batch >= version.memory);
        // `self` and `version` are metaversions of the same batch,
        // so they can be linearly ordered -- one must have been
        // obtained by modifying the other some number of times
        // (possibly zero). Each modification increments the batch
        // version and sometimes also increments the memory version.
        // Therefore, if the memory version changed, then the batch
        // version must have also changed at least once.
        if self.batch == version.batch {
            debug_assert!(self.memory == version.memory);
        } else if self.batch < version.batch {
            debug_assert!(self.memory <= version.memory);
        } else if self.batch > version.batch {
            debug_assert!(self.memory >= version.memory);
        }
        // This implies:
        // * If the memory is older, then the batch must also be older.
        // * If the memory is newer, then the batch must also be newer.
        // * If memory versions are equal, then batch versions can have any ordering.
    }

    /// Return whether `self` is older than the given `version`.
    pub fn older_than(&self, version: &Metaversion) -> bool {
        self.verify(version);
        self.batch < version.batch // See `verify` for reasoning.
    }

    /// Return whether `self` is newer than the given `version`.
    pub fn newer_than(&self, version: &Metaversion) -> bool {
        self.verify(version);
        self.batch > version.batch // See `verify` for reasoning.
    }

    /// Update this version if the given version is newer.
    pub fn maybe_update(&mut self, version: &Metaversion) {
        self.verify(version);
        if version.batch > self.batch {
            self.batch = version.batch;
            self.memory = version.memory; // See `verify` for reasoning.
        }
    }

    pub fn increment(&mut self) {
        self.memory += 1;
        self.batch += 1;
    }

    pub fn increment_batch(&mut self) {
        self.batch += 1;
    }

    pub fn increment_with(&mut self, change: &BufferChange) {
        if change.resized() {
            self.increment();
        } else if change.shifted() {
            self.increment_batch();
        }
    }
}

impl From<flatbuffers_gen::metaversion_generated::Metaversion<'_>> for Metaversion {
    fn from(state: flatbuffers_gen::metaversion_generated::Metaversion<'_>) -> Metaversion {
        Metaversion {
            memory: state.memory(),
            batch: state.batch(),
        }
    }
}
