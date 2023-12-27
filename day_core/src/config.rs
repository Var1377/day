use std::num::NonZeroU32;

use crate::{state::{self, SliceInner}, maybe_set::MaybeSet};

#[derive(Debug, thiserror::Error)]
pub enum ConfigDeserialisationError {
    #[error("Invalid slice length configuration: {0}")]
    InvalidSliceLengthOptions(&'static str),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub slices: Vec<ConfigSlice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigSlice {
    Struct(SliceBuilder),
    Name(String),
}

pub struct Context<'a> {
    pub parent_result: &'a SliceInner,
    pub parent_config: &'a ConfigSlice,
}

impl ConfigSlice {
    pub fn enabled(&self) -> bool {
        match self {
            ConfigSlice::Struct(slice) => slice.enabled.unwrap_or(true),
            ConfigSlice::Name(_) => true,
        }
    }

    pub fn as_builder(&self) -> Option<&SliceBuilder> {
        match self {
            ConfigSlice::Struct(slice) => Some(slice),
            ConfigSlice::Name(_) => None,
        }
    }

    pub fn to_slice_inner(
        &self,
        context: Option<Context>,
    ) -> Result<state::SliceInner, ConfigDeserialisationError> {
        match self {
            ConfigSlice::Struct(slice) => {
                let result = state::SliceInner {
                    id: slice.id.clone(),
                    name: slice.name.clone().unwrap_or(slice.id.clone()),
                    enabled: slice.enabled.unwrap_or(true),
                    slice_length_options: slice
                        .slice_length_options
                        .clone()
                        .unwrap_or_default()
                        .to_slice_length_options(context)?,
                    children: Vec::new(),
                    tasks: Vec::new(),
                };

                let children = slice
                    .children
                    .iter()
                    .map(|child| {
                        child.to_child_slice(Context {
                            parent_result: &result,
                            parent_config: self,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(state::SliceInner { children, ..result })
            }
            ConfigSlice::Name(name) => Ok(state::SliceInner {
                id: name.clone(),
                name: name.clone(),
                enabled: true,
                slice_length_options: SliceLengthOptionsBuilder::default()
                    .to_slice_length_options(context)?,
                children: Vec::new(),
                tasks: Vec::new(),
            }),
        }
    }

    pub fn to_root_slice(&self) -> Result<state::RootSlice, ConfigDeserialisationError> {
        let inner = self.to_slice_inner(None)?;

        Ok(state::RootSlice {
            inner,
            autoschedule_options: self
                .as_builder()
                .and_then(|slice| slice.autoschedule_options.as_ref())
                .cloned()
                .map(|options| options.try_into())
                .transpose()?
                .unwrap_or_default(),
        })
    }

    pub fn to_child_slice(
        &self,
        parent: Context,
    ) -> Result<state::ChildSlice, ConfigDeserialisationError> {
        let inner = self.to_slice_inner(Some(parent))?;

        let builder = self.as_builder();

        Ok(state::ChildSlice {
            inner,
            parent_can_schedule: builder
                .map(|slice| slice.parent_can_schedule)
                .flatten()
                .unwrap_or(true),
            parent_proportion: builder
                .map(|slice| slice.parent_proportion)
                .flatten()
                .unwrap_or(1),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SliceBuilder {
    id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    slice_length_options: Option<SliceLengthOptionsBuilder>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<ConfigSlice>,
    // root-slice fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    autoschedule_options: Option<AutoscheduleOptionsBuilder>,

    // child-slice fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parent_can_schedule: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parent_proportion: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SliceLengthOptionsBuilder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    unit: Option<state::SliceLengthUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    preferred: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    error_on_no_eligible_subslice: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default_subslice_min_minutes: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default_subslice_preferred_minutes: Option<u32>,
    #[serde(default, skip_serializing_if = "MaybeSet::is_not_set")]
    default_subslice_max_minutes: MaybeSet<u32>,
}

impl SliceLengthOptionsBuilder {
    fn to_slice_length_options(
        self,
        context: Option<Context<'_>>,
    ) -> Result<state::SliceLengthOptions, ConfigDeserialisationError> {
        let min = self
            .min
            .map(|min| {
                min.try_into().map_err(|_| {
                    ConfigDeserialisationError::InvalidSliceLengthOptions("min has to be >= 1")
                })
            })
            .unwrap_or(Ok(NonZeroU32::new(1).unwrap()))?;

        let preferred_offset = if let Some(preferred) = self.preferred {
            if preferred > min.get() {
                preferred - min.get()
            } else {
                return Err(ConfigDeserialisationError::InvalidSliceLengthOptions(
                    "preferred has to be >= min",
                ));
            }
        } else {
            0
        };

        // max has to be >= min + preferred_offset
        let max_offset = if let Some(max) = self.max {
            if max > min.get() + preferred_offset {
                Some(max - min.get() - preferred_offset)
            } else {
                return Err(ConfigDeserialisationError::InvalidSliceLengthOptions(
                    "max has to be >= preferred",
                ));
            }
        } else {
            None
        };

        let default_subslice_min_minutes =
            self.default_subslice_min_minutes
                .unwrap_or(if let Some(parent) = &context {
                    parent
                        .parent_result
                        .slice_length_options
                        .default_subslice_min_minutes
                        .get()
                } else {
                    20
                });

        let nonzero_default_subslice_min_minutes = NonZeroU32::new(default_subslice_min_minutes)
            .ok_or(ConfigDeserialisationError::InvalidSliceLengthOptions(
                "defaultSubsliceMinMinutes has to be > 0",
            ))?;

        let default_subslice_preferred_offset_minutes = self
            .default_subslice_preferred_minutes
            .map(|preferred| {
                preferred.checked_sub(default_subslice_min_minutes).ok_or(
                    ConfigDeserialisationError::InvalidSliceLengthOptions(
                        "defaultSubslicePreferredMinutes has to be >= defaultSubsliceMinMinutes",
                    ),
                )
            })
            .transpose()?
            .unwrap_or(if let Some(ctx) = &context {
                ctx
                    .parent_result
                    .slice_length_options
                    .default_subslice_preferred_offset_minutes
            } else {
                20
            });

        
        let default_subslice_max_offset_minutes = if self.default_subslice_max_minutes.is_not_set() {
            if let Some(ctx) = &context {
                ctx
                    .parent_result
                    .slice_length_options
                    .default_subslice_max_offset_minutes
            } else {
                None
            }
        } else {
            self.default_subslice_max_minutes
                .flatten()
                .map(|max| {
                    max.checked_sub(
                        default_subslice_min_minutes + default_subslice_preferred_offset_minutes,
                    )
                    .ok_or(ConfigDeserialisationError::InvalidSliceLengthOptions(
                        "defaultSubsliceMaxMinutes has to be >= defaultSubslicePreferredMinutes",
                    ))
                })
                .transpose()?
        };

        Ok(state::SliceLengthOptions {
            unit: self.unit.unwrap_or_default(),
            min,
            preferred_offset,
            max_offset,
            error_on_no_eligible_subslice: self.error_on_no_eligible_subslice.unwrap_or_default(),
            default_subslice_min_minutes: nonzero_default_subslice_min_minutes,
            default_subslice_preferred_offset_minutes,
            default_subslice_max_offset_minutes,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AutoscheduleOptionsBuilder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    autoschedule_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    autoschedule_unit: Option<state::AutoscheduleUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min_per_day: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    preferred_per_day: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_per_day: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min_per_week: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    preferred_per_week: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_per_week: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    share_of_free_time: Option<u32>,
}

impl TryInto<state::AutoscheduleOptions> for AutoscheduleOptionsBuilder {
    type Error = ConfigDeserialisationError;

    fn try_into(self) -> Result<state::AutoscheduleOptions, Self::Error> {
        let default = state::AutoscheduleOptions::default();

        let min_per_day = self
            .min_per_day
            .map(|min_per_day| {
                min_per_day.try_into().map_err(|_| {
                    ConfigDeserialisationError::InvalidSliceLengthOptions("min has to be >= 1")
                })
            })
            .unwrap_or(Ok(default.min_per_day))?;

        let preferred_per_day_offset = if let Some(preferred_per_day) = self.preferred_per_day {
            if preferred_per_day > min_per_day {
                preferred_per_day - min_per_day
            } else {
                return Err(ConfigDeserialisationError::InvalidSliceLengthOptions(
                    "preferred has to be >= min",
                ));
            }
        } else {
            default.preferred_per_day_offset
        };

        // max has to be >= min + preferred_offset
        let max_per_day_offset = if let Some(max_per_day) = self.max_per_day {
            if max_per_day > min_per_day + preferred_per_day_offset {
                Some(max_per_day - min_per_day - preferred_per_day_offset)
            } else {
                return Err(ConfigDeserialisationError::InvalidSliceLengthOptions(
                    "max has to be >= preferred",
                ));
            }
        } else {
            default.max_per_day_offset
        };

        let min_per_week = self
            .min_per_week
            .map(|min_per_week| {
                min_per_week.try_into().map_err(|_| {
                    ConfigDeserialisationError::InvalidSliceLengthOptions("min has to be >= 1")
                })
            })
            .unwrap_or(Ok(default.min_per_week))?;

        let preferred_per_week_offset = if let Some(preferred_per_week) = self.preferred_per_week {
            if preferred_per_week > min_per_week {
                preferred_per_week - min_per_week
            } else {
                return Err(ConfigDeserialisationError::InvalidSliceLengthOptions(
                    "preferred has to be >= min",
                ));
            }
        } else {
            default.preferred_per_week_offset
        };

        // max has to be >= min + preferred_offset
        let max_per_week_offset = if let Some(max_per_week) = self.max_per_week {
            if max_per_week > min_per_week + preferred_per_week_offset {
                Some(max_per_week - min_per_week - preferred_per_week_offset)
            } else {
                return Err(ConfigDeserialisationError::InvalidSliceLengthOptions(
                    "max has to be >= preferred",
                ));
            }
        } else {
            default.max_per_week_offset
        };

        Ok(state::AutoscheduleOptions {
            autoschedule_unit: self.autoschedule_unit.unwrap_or(default.autoschedule_unit),
            min_per_day,
            preferred_per_day_offset,
            max_per_day_offset,
            min_per_week,
            preferred_per_week_offset,
            max_per_week_offset,
            share_of_free_time: self
                .share_of_free_time
                .unwrap_or(default.share_of_free_time),
        })
    }
}
