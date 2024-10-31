use std::num::NonZeroU32;

use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct State {
    pub config: Config,
    pub slices: Vec<RootSlice>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceInner {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub slice_length_options: SliceLengthOptions,
    pub children: Vec<ChildSlice>,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildSlice {
    pub inner: SliceInner,
    pub parent_can_schedule: bool,
    pub parent_proportion: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootSlice {
    pub inner: SliceInner,
    pub autoschedule_options: AutoscheduleOptions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceLengthOptions {
    pub unit: SliceLengthUnit,
    pub min: NonZeroU32,
    pub preferred_offset: u32,
    pub max_offset: Option<u32>,
    pub error_on_no_eligible_subslice: bool,
    pub default_subslice_min_minutes: NonZeroU32,
    pub default_subslice_preferred_offset_minutes: u32,
    pub default_subslice_max_offset_minutes: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SliceLengthUnit {
    #[default]
    Subslice,
    Minutes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoscheduleOptions {
    pub autoschedule_unit: AutoscheduleUnit,
    pub min_per_day: u32,
    pub preferred_per_day_offset: u32,
    pub max_per_day_offset: Option<u32>,
    pub min_per_week: u32,
    pub preferred_per_week_offset: u32,
    pub max_per_week_offset: Option<u32>,
    pub share_of_free_time: u32,
}

impl Default for AutoscheduleOptions {
    fn default() -> Self {
        Self {
            autoschedule_unit: AutoscheduleUnit::Subslices,
            min_per_day: 0,
            preferred_per_day_offset: 0,
            max_per_day_offset: None,
            min_per_week: 0,
            preferred_per_week_offset: 0,
            max_per_week_offset: None,
            share_of_free_time: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AutoscheduleUnit {
    Minutes,
    Slices,
    #[default]
    Subslices,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskOptions {
    pub default_task_min_length_minutes: u32,
    pub default_task_preferred_offset_minutes: u32,
    pub default_task_max_offset_minutes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub length_minutes: u32,
    pub preferred_offset_minutes: u32,
    pub max_offset_minutes: Option<u32>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            length_minutes: 0,
            preferred_offset_minutes: 0,
            max_offset_minutes: None,
        }
    }
}