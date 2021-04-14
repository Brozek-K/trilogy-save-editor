use anyhow::*;
use async_trait::async_trait;
use bitvec::prelude::*;

use crate::{
    gui::Gui,
    save_data::{SaveCursor, SaveData},
};

pub type BoolVec = BitVec<Lsb0, u32>;
pub type BoolSlice = BitSlice<Lsb0, u32>;

#[async_trait(?Send)]
impl SaveData for BoolVec {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let len = Self::deserialize_from::<u32>(cursor)?;
        let mut bitfields = Vec::new();

        for _ in 0..len {
            bitfields.push(Self::deserialize_from::<u32>(cursor)?);
        }

        let variables = BoolVec::from_vec(bitfields);
        Ok(variables)
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let bitfields = self.clone().into_vec();

        let len = bitfields.len() as u32;
        Self::serialize_to::<u32>(&len, output)?;

        for bitfield in &bitfields {
            Self::serialize_to::<u32>(&bitfield, output)?;
        }
        Ok(())
    }

    async fn draw_raw_ui(&mut self, ui: &Gui, ident: &str) {
        ui.draw_boolvec(ident, self).await;
    }
}

#[derive(SaveData, Default, Clone)]
pub struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(SaveData, Default, Clone)]
pub struct PlotCodexPage {
    page: i32,
    is_new: bool,
}
