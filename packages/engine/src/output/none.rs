use serde_json::Value;

use super::{OutputPersistenceCreatorRepr, SimulationOutputPersistenceRepr};
use crate::{
    config::PersistenceConfig,
    output::{error::Result, OutputPersistenceResultRepr},
    proto::SimulationShortId,
    simulation::step_output::SimulationStepOutput,
    SimRunConfig,
};

#[derive(Default)]
pub struct NoOutputPersistence {}

impl NoOutputPersistence {
    pub fn new() -> NoOutputPersistence {
        Self::default()
    }
}

impl OutputPersistenceCreatorRepr for NoOutputPersistence {
    type SimulationOutputPersistence = NoSimulationOutputPersistence;

    fn new_simulation(
        &self,
        _sim_id: SimulationShortId,
        _persistence_config: &PersistenceConfig,
    ) -> Result<Self::SimulationOutputPersistence> {
        Ok(NoSimulationOutputPersistence {})
    }
}

pub struct NoSimulationOutputPersistence {}

#[async_trait::async_trait]
impl SimulationOutputPersistenceRepr for NoSimulationOutputPersistence {
    type OutputPersistenceResult = ();

    async fn add_step_output(&mut self, _output: SimulationStepOutput) -> Result<()> {
        Ok(())
    }

    async fn finalize(self, _config: &SimRunConfig) -> Result<Self::OutputPersistenceResult> {
        Ok(())
    }
}

impl OutputPersistenceResultRepr for () {
    fn into_value(self) -> Result<(&'static str, Value)> {
        Ok(("none", Value::Null))
    }
}
