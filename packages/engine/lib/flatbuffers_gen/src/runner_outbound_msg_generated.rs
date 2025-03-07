#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::cast_sign_loss,
    clippy::empty_enum,
    clippy::used_underscore_binding,
    clippy::redundant_static_lifetimes,
    clippy::redundant_field_names,
    unused_imports
)]
// automatically generated by the FlatBuffers compiler, do not modify

use std::{cmp::Ordering, mem};

use super::{
    batch_generated::*, metaversion_generated::*, package_error_generated::*,
    runner_error_generated::*, runner_errors_generated::*, runner_warning_generated::*,
    runner_warnings_generated::*, serialized_generated::*, sync_state_interim_generated::*,
    target_generated::*, user_error_generated::*, user_errors_generated::*,
    user_warning_generated::*, user_warnings_generated::*,
};

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_RUNNER_OUTBOUND_MSG_PAYLOAD: u8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_RUNNER_OUTBOUND_MSG_PAYLOAD: u8 = 9;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_RUNNER_OUTBOUND_MSG_PAYLOAD: [RunnerOutboundMsgPayload; 10] = [
    RunnerOutboundMsgPayload::NONE,
    RunnerOutboundMsgPayload::TaskMsg,
    RunnerOutboundMsgPayload::TaskCancelled,
    RunnerOutboundMsgPayload::RunnerError,
    RunnerOutboundMsgPayload::RunnerErrors,
    RunnerOutboundMsgPayload::RunnerWarning,
    RunnerOutboundMsgPayload::RunnerWarnings,
    RunnerOutboundMsgPayload::PackageError,
    RunnerOutboundMsgPayload::UserErrors,
    RunnerOutboundMsgPayload::UserWarnings,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RunnerOutboundMsgPayload(pub u8);
#[allow(non_upper_case_globals)]
impl RunnerOutboundMsgPayload {
    pub const ENUM_MAX: u8 = 9;
    pub const ENUM_MIN: u8 = 0;
    pub const ENUM_VALUES: &'static [Self] = &[
        Self::NONE,
        Self::TaskMsg,
        Self::TaskCancelled,
        Self::RunnerError,
        Self::RunnerErrors,
        Self::RunnerWarning,
        Self::RunnerWarnings,
        Self::PackageError,
        Self::UserErrors,
        Self::UserWarnings,
    ];
    pub const NONE: Self = Self(0);
    pub const PackageError: Self = Self(7);
    pub const RunnerError: Self = Self(3);
    pub const RunnerErrors: Self = Self(4);
    pub const RunnerWarning: Self = Self(5);
    pub const RunnerWarnings: Self = Self(6);
    pub const TaskCancelled: Self = Self(2);
    pub const TaskMsg: Self = Self(1);
    pub const UserErrors: Self = Self(8);
    pub const UserWarnings: Self = Self(9);

    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::NONE => Some("NONE"),
            Self::TaskMsg => Some("TaskMsg"),
            Self::TaskCancelled => Some("TaskCancelled"),
            Self::RunnerError => Some("RunnerError"),
            Self::RunnerErrors => Some("RunnerErrors"),
            Self::RunnerWarning => Some("RunnerWarning"),
            Self::RunnerWarnings => Some("RunnerWarnings"),
            Self::PackageError => Some("PackageError"),
            Self::UserErrors => Some("UserErrors"),
            Self::UserWarnings => Some("UserWarnings"),
            _ => None,
        }
    }
}
impl std::fmt::Debug for RunnerOutboundMsgPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl<'a> flatbuffers::Follow<'a> for RunnerOutboundMsgPayload {
    type Inner = Self;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { flatbuffers::read_scalar_at::<u8>(buf, loc) };
        Self(b)
    }
}

impl flatbuffers::Push for RunnerOutboundMsgPayload {
    type Output = RunnerOutboundMsgPayload;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe {
            flatbuffers::emplace_scalar::<u8>(dst, self.0);
        }
    }
}

impl flatbuffers::EndianScalar for RunnerOutboundMsgPayload {
    #[inline]
    fn to_little_endian(self) -> Self {
        let b = u8::to_le(self.0);
        Self(b)
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(self) -> Self {
        let b = u8::from_le(self.0);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for RunnerOutboundMsgPayload {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        u8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for RunnerOutboundMsgPayload {}
pub struct RunnerOutboundMsgPayloadUnionTableOffset {}

// struct TaskId, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct TaskId(pub [u8; 16]);
impl Default for TaskId {
    fn default() -> Self {
        Self([0; 16])
    }
}
impl std::fmt::Debug for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaskId")
            .field("inner", &self.inner())
            .finish()
    }
}

impl flatbuffers::SimpleToVerifyInSlice for TaskId {}
impl flatbuffers::SafeSliceAccess for TaskId {}
impl<'a> flatbuffers::Follow<'a> for TaskId {
    type Inner = &'a TaskId;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a TaskId>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a TaskId {
    type Inner = &'a TaskId;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<TaskId>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for TaskId {
    type Output = TaskId;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const TaskId as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b TaskId {
    type Output = TaskId;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const TaskId as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for TaskId {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.in_buffer::<Self>(pos)
    }
}
impl<'a> TaskId {
    #[allow(clippy::too_many_arguments)]
    pub fn new(inner: &[i8; 16]) -> Self {
        let mut s = Self([0; 16]);
        s.set_inner(&inner);
        s
    }

    pub fn inner(&'a self) -> flatbuffers::Array<'a, i8, 16> {
        flatbuffers::Array::follow(&self.0, 0)
    }

    pub fn set_inner(&mut self, items: &[i8; 16]) {
        flatbuffers::emplace_scalar_array(&mut self.0, 0, items);
    }
}

pub enum TaskMsgOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TaskMsg<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TaskMsg<'a> {
    type Inner = TaskMsg<'a>;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> TaskMsg<'a> {
    pub const VT_GROUP_INDEX: flatbuffers::VOffsetT = 10;
    pub const VT_METAVERSIONING: flatbuffers::VOffsetT = 12;
    pub const VT_PACKAGE_SID: flatbuffers::VOffsetT = 4;
    pub const VT_PAYLOAD: flatbuffers::VOffsetT = 14;
    pub const VT_TARGET: flatbuffers::VOffsetT = 8;
    pub const VT_TASK_ID: flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TaskMsg { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TaskMsgArgs<'args>,
    ) -> flatbuffers::WIPOffset<TaskMsg<'bldr>> {
        let mut builder = TaskMsgBuilder::new(_fbb);
        builder.add_group_index(args.group_index);
        builder.add_package_sid(args.package_sid);
        if let Some(x) = args.payload {
            builder.add_payload(x);
        }
        if let Some(x) = args.metaversioning {
            builder.add_metaversioning(x);
        }
        if let Some(x) = args.task_id {
            builder.add_task_id(x);
        }
        builder.add_target(args.target);
        builder.finish()
    }

    #[inline]
    pub fn package_sid(&self) -> u64 {
        self._tab
            .get::<u64>(TaskMsg::VT_PACKAGE_SID, Some(0))
            .unwrap()
    }

    #[inline]
    pub fn task_id(&self) -> Option<&'a TaskId> {
        self._tab.get::<TaskId>(TaskMsg::VT_TASK_ID, None)
    }

    #[inline]
    pub fn target(&self) -> Target {
        self._tab
            .get::<Target>(TaskMsg::VT_TARGET, Some(Target::Python))
            .unwrap()
    }

    #[inline]
    pub fn group_index(&self) -> u64 {
        self._tab
            .get::<u64>(TaskMsg::VT_GROUP_INDEX, Some(0))
            .unwrap()
    }

    #[inline]
    pub fn metaversioning(&self) -> Option<StateInterimSync<'a>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<StateInterimSync>>(TaskMsg::VT_METAVERSIONING, None)
    }

    #[inline]
    pub fn payload(&self) -> Serialized<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<Serialized>>(TaskMsg::VT_PAYLOAD, None)
            .unwrap()
    }
}

impl flatbuffers::Verifiable for TaskMsg<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u64>(&"package_sid", Self::VT_PACKAGE_SID, false)?
            .visit_field::<TaskId>(&"task_id", Self::VT_TASK_ID, false)?
            .visit_field::<Target>(&"target", Self::VT_TARGET, false)?
            .visit_field::<u64>(&"group_index", Self::VT_GROUP_INDEX, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<StateInterimSync>>(
                &"metaversioning",
                Self::VT_METAVERSIONING,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<Serialized>>(
                &"payload",
                Self::VT_PAYLOAD,
                true,
            )?
            .finish();
        Ok(())
    }
}
pub struct TaskMsgArgs<'a> {
    pub package_sid: u64,
    pub task_id: Option<&'a TaskId>,
    pub target: Target,
    pub group_index: u64,
    pub metaversioning: Option<flatbuffers::WIPOffset<StateInterimSync<'a>>>,
    pub payload: Option<flatbuffers::WIPOffset<Serialized<'a>>>,
}
impl<'a> Default for TaskMsgArgs<'a> {
    #[inline]
    fn default() -> Self {
        TaskMsgArgs {
            package_sid: 0,
            task_id: None,
            target: Target::Python,
            group_index: 0,
            metaversioning: None,
            payload: None, // required field
        }
    }
}
pub struct TaskMsgBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TaskMsgBuilder<'a, 'b> {
    #[inline]
    pub fn add_package_sid(&mut self, package_sid: u64) {
        self.fbb_
            .push_slot::<u64>(TaskMsg::VT_PACKAGE_SID, package_sid, 0);
    }

    #[inline]
    pub fn add_task_id(&mut self, task_id: &TaskId) {
        self.fbb_
            .push_slot_always::<&TaskId>(TaskMsg::VT_TASK_ID, task_id);
    }

    #[inline]
    pub fn add_target(&mut self, target: Target) {
        self.fbb_
            .push_slot::<Target>(TaskMsg::VT_TARGET, target, Target::Python);
    }

    #[inline]
    pub fn add_group_index(&mut self, group_index: u64) {
        self.fbb_
            .push_slot::<u64>(TaskMsg::VT_GROUP_INDEX, group_index, 0);
    }

    #[inline]
    pub fn add_metaversioning(
        &mut self,
        metaversioning: flatbuffers::WIPOffset<StateInterimSync<'b>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<StateInterimSync>>(
                TaskMsg::VT_METAVERSIONING,
                metaversioning,
            );
    }

    #[inline]
    pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<Serialized<'b>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<Serialized>>(TaskMsg::VT_PAYLOAD, payload);
    }

    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TaskMsgBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TaskMsgBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TaskMsg<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, TaskMsg::VT_PAYLOAD, "payload");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for TaskMsg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("TaskMsg");
        ds.field("package_sid", &self.package_sid());
        ds.field("task_id", &self.task_id());
        ds.field("target", &self.target());
        ds.field("group_index", &self.group_index());
        ds.field("metaversioning", &self.metaversioning());
        ds.field("payload", &self.payload());
        ds.finish()
    }
}
pub enum TaskCancelledOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TaskCancelled<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TaskCancelled<'a> {
    type Inner = TaskCancelled<'a>;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> TaskCancelled<'a> {
    pub const VT_TASK_ID: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TaskCancelled { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TaskCancelledArgs<'args>,
    ) -> flatbuffers::WIPOffset<TaskCancelled<'bldr>> {
        let mut builder = TaskCancelledBuilder::new(_fbb);
        if let Some(x) = args.task_id {
            builder.add_task_id(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn task_id(&self) -> Option<&'a TaskId> {
        self._tab.get::<TaskId>(TaskCancelled::VT_TASK_ID, None)
    }
}

impl flatbuffers::Verifiable for TaskCancelled<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<TaskId>(&"task_id", Self::VT_TASK_ID, false)?
            .finish();
        Ok(())
    }
}
pub struct TaskCancelledArgs<'a> {
    pub task_id: Option<&'a TaskId>,
}
impl<'a> Default for TaskCancelledArgs<'a> {
    #[inline]
    fn default() -> Self {
        TaskCancelledArgs { task_id: None }
    }
}
pub struct TaskCancelledBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TaskCancelledBuilder<'a, 'b> {
    #[inline]
    pub fn add_task_id(&mut self, task_id: &TaskId) {
        self.fbb_
            .push_slot_always::<&TaskId>(TaskCancelled::VT_TASK_ID, task_id);
    }

    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TaskCancelledBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TaskCancelledBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TaskCancelled<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for TaskCancelled<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("TaskCancelled");
        ds.field("task_id", &self.task_id());
        ds.finish()
    }
}
pub enum RunnerOutboundMsgOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RunnerOutboundMsg<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RunnerOutboundMsg<'a> {
    type Inner = RunnerOutboundMsg<'a>;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> RunnerOutboundMsg<'a> {
    pub const VT_PAYLOAD: flatbuffers::VOffsetT = 8;
    pub const VT_PAYLOAD_TYPE: flatbuffers::VOffsetT = 6;
    pub const VT_SIM_SID: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RunnerOutboundMsg { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RunnerOutboundMsgArgs,
    ) -> flatbuffers::WIPOffset<RunnerOutboundMsg<'bldr>> {
        let mut builder = RunnerOutboundMsgBuilder::new(_fbb);
        if let Some(x) = args.payload {
            builder.add_payload(x);
        }
        builder.add_sim_sid(args.sim_sid);
        builder.add_payload_type(args.payload_type);
        builder.finish()
    }

    #[inline]
    pub fn sim_sid(&self) -> u32 {
        self._tab
            .get::<u32>(RunnerOutboundMsg::VT_SIM_SID, Some(0))
            .unwrap()
    }

    #[inline]
    pub fn payload_type(&self) -> RunnerOutboundMsgPayload {
        self._tab
            .get::<RunnerOutboundMsgPayload>(
                RunnerOutboundMsg::VT_PAYLOAD_TYPE,
                Some(RunnerOutboundMsgPayload::NONE),
            )
            .unwrap()
    }

    #[inline]
    pub fn payload(&self) -> flatbuffers::Table<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(
                RunnerOutboundMsg::VT_PAYLOAD,
                None,
            )
            .unwrap()
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_task_msg(&self) -> Option<TaskMsg<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::TaskMsg {
            let u = self.payload();
            Some(TaskMsg::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_task_cancelled(&self) -> Option<TaskCancelled<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::TaskCancelled {
            let u = self.payload();
            Some(TaskCancelled::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_runner_error(&self) -> Option<RunnerError<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::RunnerError {
            let u = self.payload();
            Some(RunnerError::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_runner_errors(&self) -> Option<RunnerErrors<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::RunnerErrors {
            let u = self.payload();
            Some(RunnerErrors::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_runner_warning(&self) -> Option<RunnerWarning<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::RunnerWarning {
            let u = self.payload();
            Some(RunnerWarning::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_runner_warnings(&self) -> Option<RunnerWarnings<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::RunnerWarnings {
            let u = self.payload();
            Some(RunnerWarnings::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_package_error(&self) -> Option<PackageError<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::PackageError {
            let u = self.payload();
            Some(PackageError::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_user_errors(&self) -> Option<UserErrors<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::UserErrors {
            let u = self.payload();
            Some(UserErrors::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_user_warnings(&self) -> Option<UserWarnings<'a>> {
        if self.payload_type() == RunnerOutboundMsgPayload::UserWarnings {
            let u = self.payload();
            Some(UserWarnings::init_from_table(u))
        } else {
            None
        }
    }
}

impl flatbuffers::Verifiable for RunnerOutboundMsg<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u32>(&"sim_sid", Self::VT_SIM_SID, false)?
            .visit_union::<RunnerOutboundMsgPayload, _>(
                &"payload_type",
                Self::VT_PAYLOAD_TYPE,
                &"payload",
                Self::VT_PAYLOAD,
                true,
                |key, v, pos| match key {
                    RunnerOutboundMsgPayload::TaskMsg => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<TaskMsg>>(
                            "RunnerOutboundMsgPayload::TaskMsg",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::TaskCancelled => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<TaskCancelled>>(
                            "RunnerOutboundMsgPayload::TaskCancelled",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::RunnerError => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<RunnerError>>(
                            "RunnerOutboundMsgPayload::RunnerError",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::RunnerErrors => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<RunnerErrors>>(
                            "RunnerOutboundMsgPayload::RunnerErrors",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::RunnerWarning => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<RunnerWarning>>(
                            "RunnerOutboundMsgPayload::RunnerWarning",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::RunnerWarnings => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<RunnerWarnings>>(
                            "RunnerOutboundMsgPayload::RunnerWarnings",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::PackageError => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<PackageError>>(
                            "RunnerOutboundMsgPayload::PackageError",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::UserErrors => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<UserErrors>>(
                            "RunnerOutboundMsgPayload::UserErrors",
                            pos,
                        ),
                    RunnerOutboundMsgPayload::UserWarnings => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<UserWarnings>>(
                            "RunnerOutboundMsgPayload::UserWarnings",
                            pos,
                        ),
                    _ => Ok(()),
                },
            )?
            .finish();
        Ok(())
    }
}
pub struct RunnerOutboundMsgArgs {
    pub sim_sid: u32,
    pub payload_type: RunnerOutboundMsgPayload,
    pub payload: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RunnerOutboundMsgArgs {
    #[inline]
    fn default() -> Self {
        RunnerOutboundMsgArgs {
            sim_sid: 0,
            payload_type: RunnerOutboundMsgPayload::NONE,
            payload: None, // required field
        }
    }
}
pub struct RunnerOutboundMsgBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RunnerOutboundMsgBuilder<'a, 'b> {
    #[inline]
    pub fn add_sim_sid(&mut self, sim_sid: u32) {
        self.fbb_
            .push_slot::<u32>(RunnerOutboundMsg::VT_SIM_SID, sim_sid, 0);
    }

    #[inline]
    pub fn add_payload_type(&mut self, payload_type: RunnerOutboundMsgPayload) {
        self.fbb_.push_slot::<RunnerOutboundMsgPayload>(
            RunnerOutboundMsg::VT_PAYLOAD_TYPE,
            payload_type,
            RunnerOutboundMsgPayload::NONE,
        );
    }

    #[inline]
    pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(RunnerOutboundMsg::VT_PAYLOAD, payload);
    }

    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> RunnerOutboundMsgBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RunnerOutboundMsgBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RunnerOutboundMsg<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, RunnerOutboundMsg::VT_PAYLOAD, "payload");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for RunnerOutboundMsg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("RunnerOutboundMsg");
        ds.field("sim_sid", &self.sim_sid());
        ds.field("payload_type", &self.payload_type());
        match self.payload_type() {
            RunnerOutboundMsgPayload::TaskMsg => {
                if let Some(x) = self.payload_as_task_msg() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::TaskCancelled => {
                if let Some(x) = self.payload_as_task_cancelled() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::RunnerError => {
                if let Some(x) = self.payload_as_runner_error() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::RunnerErrors => {
                if let Some(x) = self.payload_as_runner_errors() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::RunnerWarning => {
                if let Some(x) = self.payload_as_runner_warning() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::RunnerWarnings => {
                if let Some(x) = self.payload_as_runner_warnings() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::PackageError => {
                if let Some(x) = self.payload_as_package_error() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::UserErrors => {
                if let Some(x) = self.payload_as_user_errors() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerOutboundMsgPayload::UserWarnings => {
                if let Some(x) = self.payload_as_user_warnings() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            _ => {
                let x: Option<()> = None;
                ds.field("payload", &x)
            }
        };
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_runner_outbound_msg<'a>(buf: &'a [u8]) -> RunnerOutboundMsg<'a> {
    unsafe { flatbuffers::root_unchecked::<RunnerOutboundMsg<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_runner_outbound_msg<'a>(buf: &'a [u8]) -> RunnerOutboundMsg<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<RunnerOutboundMsg<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `RunnerOutboundMsg`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_outbound_msg_unchecked`.
pub fn root_as_runner_outbound_msg(
    buf: &[u8],
) -> Result<RunnerOutboundMsg, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<RunnerOutboundMsg>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `RunnerOutboundMsg` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_runner_outbound_msg_unchecked`.
pub fn size_prefixed_root_as_runner_outbound_msg(
    buf: &[u8],
) -> Result<RunnerOutboundMsg, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<RunnerOutboundMsg>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `RunnerOutboundMsg` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_outbound_msg_unchecked`.
pub fn root_as_runner_outbound_msg_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerOutboundMsg<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<RunnerOutboundMsg<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `RunnerOutboundMsg` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_outbound_msg_unchecked`.
pub fn size_prefixed_root_as_runner_outbound_msg_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerOutboundMsg<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<RunnerOutboundMsg<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a RunnerOutboundMsg and returns
/// it. # Safety
/// Callers must trust the given bytes do indeed contain a valid `RunnerOutboundMsg`.
pub unsafe fn root_as_runner_outbound_msg_unchecked(buf: &[u8]) -> RunnerOutboundMsg {
    flatbuffers::root_unchecked::<RunnerOutboundMsg>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed RunnerOutboundMsg
/// and returns it. # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `RunnerOutboundMsg`.
pub unsafe fn size_prefixed_root_as_runner_outbound_msg_unchecked(buf: &[u8]) -> RunnerOutboundMsg {
    flatbuffers::size_prefixed_root_unchecked::<RunnerOutboundMsg>(buf)
}
#[inline]
pub fn finish_runner_outbound_msg_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerOutboundMsg<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_runner_outbound_msg_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerOutboundMsg<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
