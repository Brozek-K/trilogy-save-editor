mod bonus_powers;
mod head_morph;
mod link;
mod plot_category;
mod raw_plot;

pub use self::{bonus_powers::*, head_morph::*, link::*, plot_category::*, raw_plot::*};

use indexmap::IndexMap;
use yew::prelude::*;

use super::raw_ui::RawUi;
use crate::gui::RcUi;
use crate::save_data::shared::plot::{BitVec, PlotTable};

#[derive(Clone)]
pub enum IntPlotType {
    Vec(RcUi<Vec<RcUi<i32>>>),
    IndexMap(RcUi<IndexMap<i32, RcUi<i32>>>),
}

impl PartialEq for IntPlotType {
    fn eq(&self, other: &IntPlotType) -> bool {
        match (self, other) {
            (IntPlotType::Vec(vec), IntPlotType::Vec(other)) => vec == other,
            (IntPlotType::IndexMap(index_map), IntPlotType::IndexMap(other)) => index_map == other,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub enum FloatPlotType {
    Vec(RcUi<Vec<RcUi<f32>>>),
    IndexMap(RcUi<IndexMap<i32, RcUi<f32>>>),
}

impl PartialEq for FloatPlotType {
    fn eq(&self, other: &FloatPlotType) -> bool {
        match (self, other) {
            (FloatPlotType::Vec(vec), FloatPlotType::Vec(other)) => vec == other,
            (FloatPlotType::IndexMap(index_map), FloatPlotType::IndexMap(other)) => {
                index_map == other
            }
            _ => false,
        }
    }
}

#[derive(Clone)]
pub enum PlotType {
    Boolean(RcUi<BitVec>),
    Integer(IntPlotType),
    Float(FloatPlotType),
}

impl PartialEq for PlotType {
    fn eq(&self, other: &PlotType) -> bool {
        match (self, other) {
            (PlotType::Boolean(booleans), PlotType::Boolean(other)) => booleans == other,
            (PlotType::Integer(integers), PlotType::Integer(other)) => integers == other,
            (PlotType::Float(floats), PlotType::Float(other)) => floats == other,
            _ => false,
        }
    }
}

impl RawUi for RcUi<PlotTable> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <Link label={label.to_owned()} tab="Raw Plot">{ "Go to Raw Plot tab" }</Link>
        }
    }
}
