use crate::{
    datastore::prelude::*,
    proto::{ExperimentId, SharedDataset},
};

pub struct Batch {
    pub(crate) memory: Memory,
    loaded: Metaversion,
}

impl super::Batch for Batch {
    fn memory(&self) -> &Memory {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    fn loaded(&self) -> &Metaversion {
        &self.loaded
    }

    fn latest_known(&self) -> &Metaversion {
        &self.loaded // Since the latest known version can't be updated for datasets
    }

    fn update_latest_known(&mut self, _new_version: &Metaversion) {
        panic!("`update_latest_known` is not implemented for context batch")
    }

    fn load_latest_known(&mut self) -> Result<()> {
        Err(Error::from("`load_latest_known` is not implemented for context batch"))
    }
}

impl Batch {
    pub fn new_from_dataset(
        dataset: &SharedDataset,
        experiment_id: &ExperimentId,
    ) -> Result<Batch> {
        let dataset_name = dataset.shortname.clone();
        let dataset_size = dataset
            .data
            .as_ref()
            .map(|data| data.len())
            .unwrap_or_default();
        let mut memory =
            Memory::from_sizes(experiment_id, 0, dataset_name.len(), 0, dataset_size, false)?;
        let reload_state = Metaversion::default();
        memory.set_header(&dataset_name)?;
        let buffer = memory.get_mut_data_buffer()?;
        buffer.copy_from_slice(
            dataset
                .data
                .as_ref()
                .map(|data| data.as_bytes())
                .unwrap_or_default(),
        );
        Ok(Batch {
            memory,
            reload_state,
        })
    }
}
