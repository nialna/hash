// automatically generated by the FlatBuffers compiler, do not modify

use std::{cmp::Ordering, mem};

use crate::{batch_generated::*, metaversion_generated::*};

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum StateSnapshotSyncOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct StateSnapshotSync<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StateSnapshotSync<'a> {
    type Inner = StateSnapshotSync<'a>;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> StateSnapshotSync<'a> {
    pub const VT_AGENT_POOL: flatbuffers::VOffsetT = 4;
    pub const VT_CURRENT_STEP: flatbuffers::VOffsetT = 8;
    pub const VT_MESSAGE_POOL: flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        StateSnapshotSync { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args StateSnapshotSyncArgs<'args>,
    ) -> flatbuffers::WIPOffset<StateSnapshotSync<'bldr>> {
        let mut builder = StateSnapshotSyncBuilder::new(_fbb);
        builder.add_current_step(args.current_step);
        if let Some(x) = args.message_pool {
            builder.add_message_pool(x);
        }
        if let Some(x) = args.agent_pool {
            builder.add_agent_pool(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn agent_pool(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch>>,
            >>(StateSnapshotSync::VT_AGENT_POOL, None)
            .unwrap()
    }

    #[inline]
    pub fn message_pool(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch>>,
            >>(StateSnapshotSync::VT_MESSAGE_POOL, None)
            .unwrap()
    }

    #[inline]
    pub fn current_step(&self) -> i64 {
        self._tab
            .get::<i64>(StateSnapshotSync::VT_CURRENT_STEP, Some(0))
            .unwrap()
    }
}

impl flatbuffers::Verifiable for StateSnapshotSync<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Batch>>,
            >>(&"agent_pool", Self::VT_AGENT_POOL, true)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Batch>>,
            >>(&"message_pool", Self::VT_MESSAGE_POOL, true)?
            .visit_field::<i64>(&"current_step", Self::VT_CURRENT_STEP, false)?
            .finish();
        Ok(())
    }
}
pub struct StateSnapshotSyncArgs<'a> {
    pub agent_pool: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>>>,
    >,
    pub message_pool: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>>>,
    >,
    pub current_step: i64,
}
impl<'a> Default for StateSnapshotSyncArgs<'a> {
    #[inline]
    fn default() -> Self {
        StateSnapshotSyncArgs {
            agent_pool: None,   // required field
            message_pool: None, // required field
            current_step: 0,
        }
    }
}
pub struct StateSnapshotSyncBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> StateSnapshotSyncBuilder<'a, 'b> {
    #[inline]
    pub fn add_agent_pool(
        &mut self,
        agent_pool: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Batch<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            StateSnapshotSync::VT_AGENT_POOL,
            agent_pool,
        );
    }

    #[inline]
    pub fn add_message_pool(
        &mut self,
        message_pool: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Batch<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            StateSnapshotSync::VT_MESSAGE_POOL,
            message_pool,
        );
    }

    #[inline]
    pub fn add_current_step(&mut self, current_step: i64) {
        self.fbb_
            .push_slot::<i64>(StateSnapshotSync::VT_CURRENT_STEP, current_step, 0);
    }

    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> StateSnapshotSyncBuilder<'a, 'b> {
        let start = _fbb.start_table();
        StateSnapshotSyncBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<StateSnapshotSync<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, StateSnapshotSync::VT_AGENT_POOL, "agent_pool");
        self.fbb_
            .required(o, StateSnapshotSync::VT_MESSAGE_POOL, "message_pool");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for StateSnapshotSync<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("StateSnapshotSync");
        ds.field("agent_pool", &self.agent_pool());
        ds.field("message_pool", &self.message_pool());
        ds.field("current_step", &self.current_step());
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_state_snapshot_sync<'a>(buf: &'a [u8]) -> StateSnapshotSync<'a> {
    unsafe { flatbuffers::root_unchecked::<StateSnapshotSync<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_state_snapshot_sync<'a>(buf: &'a [u8]) -> StateSnapshotSync<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<StateSnapshotSync<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `StateSnapshotSync`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_snapshot_sync_unchecked`.
pub fn root_as_state_snapshot_sync(
    buf: &[u8],
) -> Result<StateSnapshotSync, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<StateSnapshotSync>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `StateSnapshotSync` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_state_snapshot_sync_unchecked`.
pub fn size_prefixed_root_as_state_snapshot_sync(
    buf: &[u8],
) -> Result<StateSnapshotSync, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<StateSnapshotSync>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `StateSnapshotSync` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_snapshot_sync_unchecked`.
pub fn root_as_state_snapshot_sync_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<StateSnapshotSync<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<StateSnapshotSync<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `StateSnapshotSync` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_snapshot_sync_unchecked`.
pub fn size_prefixed_root_as_state_snapshot_sync_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<StateSnapshotSync<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<StateSnapshotSync<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a StateSnapshotSync and returns
/// it. # Safety
/// Callers must trust the given bytes do indeed contain a valid `StateSnapshotSync`.
pub unsafe fn root_as_state_snapshot_sync_unchecked(buf: &[u8]) -> StateSnapshotSync {
    flatbuffers::root_unchecked::<StateSnapshotSync>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed StateSnapshotSync
/// and returns it. # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `StateSnapshotSync`.
pub unsafe fn size_prefixed_root_as_state_snapshot_sync_unchecked(buf: &[u8]) -> StateSnapshotSync {
    flatbuffers::size_prefixed_root_unchecked::<StateSnapshotSync>(buf)
}
#[inline]
pub fn finish_state_snapshot_sync_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<StateSnapshotSync<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_state_snapshot_sync_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<StateSnapshotSync<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
