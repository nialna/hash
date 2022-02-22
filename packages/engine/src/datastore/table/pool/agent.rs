use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    datastore::{
        batch::{AgentBatch, DynamicBatch},
        table::pool::proxy::PoolWriteProxy,
        Result,
    },
    simulation::package::state::StateColumn,
};
use crate::datastore::batch::{Batch, Metaversion};

/// An ordered collection of similar [`AgentBatch`]es for each group within a simulation run.
#[derive(Clone)]
pub struct AgentPool {
    batches: Vec<Arc<RwLock<AgentBatch>>>,
}

impl super::Pool<AgentBatch> for AgentPool {
    fn new(batches: Vec<Arc<RwLock<AgentBatch>>>) -> Self {
        Self { batches }
    }

    fn get_batches(&self) -> &[Arc<RwLock<AgentBatch>>] {
        &self.batches
    }

    fn get_batches_mut(&mut self) -> &mut Vec<Arc<RwLock<AgentBatch>>> {
        &mut self.batches
    }
}

impl Extend<AgentBatch> for AgentPool {
    fn extend<T: IntoIterator<Item = AgentBatch>>(&mut self, iter: T) {
        self.batches
            .extend(iter.into_iter().map(|batch| Arc::new(RwLock::new(batch))))
    }
}

impl PoolWriteProxy<AgentBatch> {
    pub fn modify_loaded_column(
        &mut self,
        column: StateColumn
    ) -> Result<()> {
        let mut group_start = 0;
        for batch in self.batches_iter_mut() {
            let num_agents = batch.num_agents();
            let next_start = group_start + num_agents;
            let change = column.get_arrow_change(group_start..next_start)?;

            // The data we write into the agent pool is based on taking
            // the data currently loaded in the record batches in the pool
            // and then modifying it, so the batch version should become
            // newer by one than the loaded version.
            let change_version = {
                let mut m = *batch.loaded();
                m.increment_batch();
                m
            };

            batch.push_change(change, &change_version)?;
            group_start = next_start;
        }
        Ok(())
    }

    pub fn flush_pending_columns(&mut self) -> Result<()> {
        for batch in self.batches_iter_mut() {
            batch.flush_changes()?;
        }
        Ok(())
    }
}
