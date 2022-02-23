# automatically generated by the FlatBuffers compiler, do not modify

# namespace: 

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class NewSimulationRun(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = NewSimulationRun()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsNewSimulationRun(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # NewSimulationRun
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # NewSimulationRun
    def SimId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # NewSimulationRun
    def Sid(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

    # NewSimulationRun
    def Globals(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # NewSimulationRun
    def PackageConfig(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            x = self._tab.Indirect(o + self._tab.Pos)
            from PackageConfig import PackageConfig
            obj = PackageConfig()
            obj.Init(self._tab.Bytes, x)
            return obj
        return None

    # NewSimulationRun
    def DatastoreInit(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            x = self._tab.Indirect(o + self._tab.Pos)
            from DatastoreInit import DatastoreInit
            obj = DatastoreInit()
            obj.Init(self._tab.Bytes, x)
            return obj
        return None

def Start(builder): builder.StartObject(5)
def NewSimulationRunStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddSimId(builder, simId): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(simId), 0)
def NewSimulationRunAddSimId(builder, simId):
    """This method is deprecated. Please switch to AddSimId."""
    return AddSimId(builder, simId)
def AddSid(builder, sid): builder.PrependUint32Slot(1, sid, 0)
def NewSimulationRunAddSid(builder, sid):
    """This method is deprecated. Please switch to AddSid."""
    return AddSid(builder, sid)
def AddGlobals(builder, globals): builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(globals), 0)
def NewSimulationRunAddGlobals(builder, globals):
    """This method is deprecated. Please switch to AddGlobals."""
    return AddGlobals(builder, globals)
def AddPackageConfig(builder, packageConfig): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(packageConfig), 0)
def NewSimulationRunAddPackageConfig(builder, packageConfig):
    """This method is deprecated. Please switch to AddPackageConfig."""
    return AddPackageConfig(builder, packageConfig)
def AddDatastoreInit(builder, datastoreInit): builder.PrependUOffsetTRelativeSlot(4, flatbuffers.number_types.UOffsetTFlags.py_type(datastoreInit), 0)
def NewSimulationRunAddDatastoreInit(builder, datastoreInit):
    """This method is deprecated. Please switch to AddDatastoreInit."""
    return AddDatastoreInit(builder, datastoreInit)
def End(builder): return builder.EndObject()
def NewSimulationRunEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)