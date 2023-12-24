# Day.rs: Overview
Throughout our lives we constantly face the challenge of working towards long term goals while trying to balance our time-constrained everyday lives. Day is a tool that facilitates productive and sustainable time management by allowing the user to define their goals declaritively and automatically scheduling them around their everyday commitments. 

As part of this, day augments or replaces your traditional calendaring and todo systems.

## What can it do
Automatically managing and scheduling tasks and sleep around your everyday commitments. This can be anything from a standard list of To-Dos with or without deadlines to a complex schedule of constrained events, on-going projects and recurring tasks, with special considerations for optimally partioning sleep, exercise, meals, work, socialising and rest in a way that's both productive and sustainable long-term.

## Crates
There are two crates in this repo:
- `day_core` contains all the types and scheduling logic and is partitioned into a separate crate to allow for non-cli interfaces in the future.
- `day_cli` contains all the cli logic, including the storage of state and configuration files

## Installation
`cargo install day_cli`



# Slices
The true power of day comes with it's module system, allowing you to systematically model a graph like structure defining how you might want to spend your time.
These modules are called slices, and represent a slice of your time.

## Inbuilt Slices
Day is configured through the use of modules, each of which interacts with the scheduler in a (mostly) configurable order in order to prioritise more important tasks.

The only inbuilt module currently is the sleep module.

## Custom Slices
Each custom slice is made up of a set of parameters, and crucially, a set of child slices. These child slices are called subslices and are scheduled within the parent slice, and can be configured to be scheduled in a variety of ways. If a slice has no children that can be scheduled, it is considered a leaf slice and is scheduled directly.

### Parameters

#### General

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `name` | `string` | Name | N/A |
| `description` | `string` | Description | N/A |
| `noEligibleSubsliceBehaviour` | `error \| opaque` | What to do if there are no eligible subslices to schedule.<br>`error` stops the slice being scheduled.<br>`opaque` directly schedules the slice without specifying a subslice | `opaque` |

#### Slice Length

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `unit` | `minutes \| subslices` | The unit of time to schedule this slice for | `subslices` |
| `min` | `int` | The minimum number of units to schedule this slice for | `1` |
| `preferred` | `int` | The preferred number of units to schedule this slice for | `min` |
| `max` | `int` | The maximum number of units to schedule this slice for | `max(min, preferred)` |

#### Default Length

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `defaultSubsliceMinMins` | `int` | Minimum minutes when not delegating to a subslice | `10` for Root Slices<br>Same as parent otherwise |
| `defaultSubslicePreferredMins` | `int` | Preferred minutes when not delegating to a subslice | `30` for Root Slices<br>Same as parent otherwise |
| `defaultSubsliceMaxMins` | `int` | Maximum minutes when not delegating to a subslice | `60` for Root Slices<br>Same as parent otherwise |

#### Self-triggers (Root Slices only)

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `autoschedule` | `AutoSchedule | null` | Whether to autoschedule this slice. If `null`, this slice can only be scheduled manually or with a trigger. | `null` |

##### `AutoSchedule`

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `length` | `int` | The number of units you want completed per `timePeriod` | `1` |
| `lengthUnit` | `minutes \| subslices \| slices` | The unit of time for `length` | `slices` |
| `timePeriod` | `int` | The time period in which to complete `length` amount of this slice | `1` |
| `timePeriodUnit` | `day \| proportion` | The unit for `timePeriod` | `proportion` |
| `contextBehaviour` | `contextFree \| average \| onlyUp \| onlyDown` | How to correct for more or less time being spent on a slice in the past  | `contextFree` |
| `maxPerDay` | `int \| null` | The maximum number of times this slice can be autoscheduled in a day | `null` |
| `maxPerWeek` | `int \| null` | The maximum number of times this slice can be autoscheduled in a week | `null` |

#### Self-triggers (Child Slices only)
| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `parentCanSchedule` | `bool` | Whether the parent slice can schedule this slice | `true` |

#### Triggers

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `intersperse` | `string \| null` | What slice to intersperse subslices with | `null` |
| `preferredIntersperseBehaviour` | `fixed \| proportional` | Preferred length of interspersed slices | `fixed` |
| `intersperseRatio` | `float` | Time in this slice / Time in interspersed slice | `4.0` |

#### Subslices

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `children` | `Slice[]` | The subslices of this slice | `[]` |
| `subsliceSelectionBehaviour` | `roundRobin \| equalSlices \| equalTime \| random` | How to select subslices to schedule | `equalSlices` |

#### Tasks

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `tasks` | `Task[]` | The tasks of this slice | `[]` |

##### `Task`

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `name` | `string` | Name | N/A |
| `description` | `string` | Description | N/A |
| `totalTime` | `int` | Total time to spend on this task | Inherited from parent slice |
| `timeSpent` | `int` | Time already spent on this task | `0` |
| `deadline` | `DateTime \| null` | Deadline. | `null` |
| `subtasks` | `Task[]` | Subtasks | `[]` |
| `subtaskSelectionBehaviour` | `inOrder \| firstFit \| random` | How to select subtasks to schedule | `inOrder` |
| `defaultSubtaskTime` | `int` | Default time to spend on each subtask | `totalTime / subtasks.length` |

# Example

```js
[
    {
        name: "Work",
        // don't need to specify anything else as it's scheduled by an external calendar
    },
    {
        name: "Rest",
        autoschedule: {
            length_unit: "proportion",
            length: 1,
        },
        children: [
            {
                name: "Socialising",
                parent_can_schedule: false
            }
        ]
    },
    {
        name: "Study",
        no_eligible_subslice_behaviour: "error",
        autoschedule: {
            length_unit: "minutes",
            length: 1200,
            time_period_unit: "day",
            time_period: 7,
            context_behaviour: "average"
        },
        intersperse: "Rest",
        preferred_intersperse_behaviour: "proportional",
        intersperse_ratio: 5.0,
        children: [
            {
                name: "GroupWork",
                parent_can_schedule: false
            },
            {
                name: "Subject1"
            }
        ]
    },
    {
        name: "Exercise",
        min: 1,
        preferred: 2,
        autoschedule: {
            length: 3,
            length_unit: "slices",
            time_period_unit: "day",
            time_period: 7,
            context_behaviour: "only_down"
        },
        children: [
            {
                name: "Cardio",
                opaque_subslice_min_mins: 20,
                opaque_subslice_max_mins: 40,
                children: [
                    {
                        name: "Running"
                    },
                    {
                        name: "Rowing"
                    }
                ]
            },
            {
                name: "ResistanceTraining"
            }
        ]
    }
]
```