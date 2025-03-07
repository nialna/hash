use std::fmt;

use arrow::{
    datatypes::{DataType, Field as ArrowField},
    error::ArrowError,
};
use thiserror::Error as ThisError;

use super::prelude::*;
use crate::{
    datastore::schema::{FieldKey, FieldType, RootFieldSpec, ShortJsonError},
    hash_types,
    hash_types::state::AgentStateField,
};

#[derive(Debug)]
pub enum SupportedType {
    NullableField,
    DataType(String),
    ArrowDataType(DataType),
    VariableSizeColumn,
}

impl fmt::Display for SupportedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullableField => write!(f, "NullableField"),
            Self::DataType(s) => write!(f, "DataType: {}", s),
            Self::ArrowDataType(a) => write!(f, "ArrowDataType: {:?}", a),
            Self::VariableSizeColumn => write!(f, "VariableSizeColumn"),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Unique(String),

    #[error("Couldn't acquire shared lock on object")]
    ProxySharedLock,

    #[error("Couldn't acquire exclusive lock on object")]
    ProxyExclusiveLock,

    #[error("Special field missing in schema: {0}")]
    SpecialKeyMissing(String),

    #[error("Arrow Error: {0}")]
    Arrow(#[from] ArrowError),

    #[error("Field not found in Schema: {0}")]
    FieldNotFound(ArrowField),

    #[error("Invalid Arrow object downcast. Field name: {name}")]
    InvalidArrowDowncast { name: String },

    #[error("Memory Error: {0}")]
    Memory(String),

    #[error("Not implemented: {0}")]
    NotImplemented(SupportedType),

    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Parse Int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Tried to convert a slice to an array: {0}")]
    TryFromSlice(#[from] std::array::TryFromSliceError),

    #[error("Failed to interpret a sequence of u8's (bytes) as a string: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Simulation error: {0}")]
    Simulation(#[from] hash_types::error::Error),

    #[error("Shared memory error: {0}")]
    SharedMemory(#[from] shared_memory::ShmemError),

    #[error("Failed to acquire lock ({0})")]
    Lock(&'static str),

    #[error("Agent id ({0}) is not a valid uuid")]
    InvalidAgentId(String),

    #[error("Invalid index {ind} (len: {len})")]
    InvalidIndex { ind: usize, len: usize },

    #[error("Arrow RecordBatch message error: {0}")]
    ArrowBatch(String),

    #[error("Failed to read Arrow Schema from buffer")]
    ArrowSchemaRead,

    #[error("No column found in batch with name: {0}")]
    ColumnNotFound(String),

    #[error("Array data was expected to contain child data")]
    ChildDataExpected,

    #[error("Array data was expected to contain buffers")]
    BufferExpected,

    #[error("Join Handle error: {0}")]
    JoinHandle(String),

    #[error("Unexpected vector length: was {len} but expected {expected}")]
    UnexpectedVectorLength { len: usize, expected: usize },

    #[error("Invalid system agent message: to: {to:?}, type: {message_type}, data: {data}")]
    InvalidSystemMessage {
        to: Vec<String>,
        message_type: String,
        data: String,
    },

    #[error("Unexpected Arrow datatype. Expected: {expected:?}, got: {got:?}")]
    UnexpectedArrowDataType {
        expected: ArrowDataType,
        got: ArrowDataType,
    },

    #[error("Unsupported Arrow datatype: {d_type:?}")]
    UnsupportedArrowDataType { d_type: ArrowDataType },

    #[error("Expected Arrow Node to contain a null buffer")]
    NullBufferExpected,

    #[error("Did not expect to resize Shared Memory")]
    UnexpectedAgentBatchMemoryResize,

    #[error("did not expect to resize Message Batch shared memory")]
    UnexpectedMessageBatchMemoryResize,

    #[error("Uuid error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Shmem max size reached: Size: {0}, Allowed: {1}")]
    SharedMemoryMaxSize(u64, u64),

    #[error("Expected Buffer actions to contain at least one action")]
    EmptyBufferActionsList,

    #[error("Expected Node Metadata")]
    NodeMetadataExpected,

    #[error("Expected Buffer Metadata")]
    BufferMetadataExpected,

    #[error("Expected Shift Action Vector to be non-empty")]
    EmptyShiftActionVector,

    #[error("Schema size mismatch. Current size: {current}, proposed size: {proposed}")]
    SchemaBufferSizeMismatch { current: usize, proposed: usize },

    #[error("Invalid fixed size list. Required size of list: {required}, actual: {actual}.")]
    FixedSizeListInvalidValue { required: i32, actual: usize },

    #[error("Object is missing field with name: {0}")]
    MissingFieldInObject(String),

    #[error("Batch expected in pool with length of {length} at index: {index}")]
    BatchExpected { length: usize, index: usize },

    #[error("Expected boolean value in `serde_json::Value`")]
    BooleanSerdeValueExpected,

    #[error("Unable to read IPC message as record batch")]
    InvalidRecordBatchIpcMessage,

    #[error("Expected new data buffer length to be smaller than current. Id: {0}")]
    ExpectedSmallerNewDataSize(String),

    #[error("Cannot create empty shared memory objects")]
    EmptySharedMemory,

    #[error("Cannot unwrap Arc")]
    ArcUnwrap,

    #[error("Cannot get inner value from RwLock for agent batch")]
    RwLockInnerAgentBatch(#[from] std::sync::PoisonError<AgentBatch>),

    #[error("Cannot get inner value from RwLock for message batch: {0:?}")]
    RwLockInnerMessageBatch(#[from] std::sync::PoisonError<MessageBatch>),

    #[error("Built-in column missing: {0:?}")]
    BuiltInColumnMissing(AgentStateField),

    #[error("Behavior keys are missing for behavior: {0}")]
    BehaviorKeysMissing(String),

    #[error("Key missing in KeySet: {0}")]
    CannotFindKeyInKeySet(String),

    #[error("IO: {0:?}")]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    RwLock(String),

    #[error("Invalid utf-8: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("Invalid behavior name: \"{0}\"")]
    InvalidBehaviorName(String),

    #[error("Behavior names should be valid utf-8 bytes")]
    InvalidBehaviorNameUtf8,

    #[error("Unexpected undefined command")]
    UnexpectedUndefinedCommand,

    #[error("Field Spec Short JSON repr error: {0}")]
    ShortJsonError(#[from] ShortJsonError),

    #[error(
        "Key clash when attempting to insert a new agent-scoped field with key: {0:?}. The new \
         field has a differing type: {1:?} to the existing field: {2:?}"
    )]
    AgentScopedFieldKeyClash(FieldKey, FieldType, FieldType),

    #[error(
        "Attempting to insert a new field under key:{0:?} which clashes. New field: {1:?} \
         Existing field: {2:?}"
    )]
    FieldKeyClash(FieldKey, RootFieldSpec, RootFieldSpec),

    #[error(
        "Can't take multiple write access to shared state, e.g. by cloning writable task shared \
         store"
    )]
    MultipleWriteSharedState,
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unique(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unique(s)
    }
}

impl<'a, T> From<std::sync::TryLockError<std::sync::RwLockReadGuard<'a, T>>> for Error {
    fn from(_: std::sync::TryLockError<std::sync::RwLockReadGuard<'a, T>>) -> Self {
        Error::RwLock("RwLock read error for Datastore".into())
    }
}

impl<'a, T> From<std::sync::TryLockError<std::sync::RwLockWriteGuard<'a, T>>> for Error {
    fn from(_: std::sync::TryLockError<std::sync::RwLockWriteGuard<'a, T>>) -> Self {
        Error::RwLock("RwLock write error for Datastore".into())
    }
}
