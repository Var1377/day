# Day.rs: Overview
Throughout our lives we constantly face the challenge of working towards long term goals while trying to balance our time-constrained everyday lives. Day is a tool that facilitates productive and sustainable time management by allowing the user to define their goals declaritively and automatically scheduling them around their everyday commitments. 

As part of this, day augments or replaces your traditional calendaring and todo systems.

## What can it do
Automatically managing and scheduling tasks and sleep around your everyday commitments. This can be anything from a standard list of To-Dos with or without deadlines to a complex schedule of constrained events, on-going projects and recurring tasks, with special considerations for optimally partioning sleep, exercise, meals, work, socialising and rest in a way that's both productive and sustainable long-term.

## Crates
There are two crates in this repo:
- `day_core` contains all the types and scheduling logic
- `day_cli` contains all the cli logic, including the storage of state and configuration files

## Installation
`cargo install day_cli`

## Usage
`day --help`

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
| `name` | `string` | The name of the slice | N/A |
| `id` | `string` | A unique identifier | `name` |

#### Slice Length

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `unit` | `minutes \| subslices` | The unit of time to schedule this slice for | `subslices` |
| `min` | `int` | The minimum number of units to schedule this slice for | `1` |
| `preferred` | `int` | The preferred number of units to schedule this slice for | `min` |
| `max` | `int \| null` | The maximum number of units to schedule this slice for | `null` |

#### Default Length

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `defaultSubsliceMinMins` | `int` | Minimum minutes when not delegating to a subslice | `20` for Root Slices<br>Same as parent otherwise |
| `defaultSubslicePreferredMins` | `int` | Preferred minutes when not delegating to a subslice | `40` for Root Slices<br>Same as parent otherwise |
| `defaultSubsliceMaxMins` | `int \| null` | Maximum minutes when not delegating to a subslice | `null` for Root Slices<br>Same as parent otherwise |

#### Self-triggers (Root Slices only)

| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `autoscheduleUnit` | `minutes \| subslices \| slices` | The unit of time for `length` | `subslices` |
| `minPerDay` | `int` | The minimum number of `autoscheduleUnit`s this slice can be autoscheduled in a day | `0` |
| `preferredPerDay` | `int` | The preferred number of `autoscheduleUnit`s this slice can be scheduled in a day | `minPerDay` |
| `maxPerDay` | `int \| null` | The maximum number of `autoscheduleUnit`s this slice can be scheduled in a day (unless manually scheduled) | `null` |
| `minPerWeek` | `int` | The minimum number of `autoscheduleUnit`s this slice can be autoscheduled in a week | `0` |
| `preferredPerWeek` | `int` | The preferred number of `autoscheduleUnit`s this slice can be scheduled in a week | `minPerWeek` |
| `maxPerWeek` | `int \| null` | The maximum number of `autoscheduleUnit`s this slice can be scheduled in a week (unless manually scheduled) | `null` |
| `shareOfFreeTime` | `int` | `shareOfFreeTime / sum(slices shareOfFreeTime for all root slices)` is the proportion of otherwise unoccupied time taken up by this | `0` |

#### Self-triggers (Child Slices only)
| Name | Type | Description | Default |
| ---- | ---- | ----------- | ------- |
| `parentProportion` | `int` | `parentProportion / (parentProportion + sum of sibling parentProportions)` is the proportion of the parent's slices time or sessions (depending on the parent's `subsliceSelectionBehaviour`) dedicated to that slice | `1` |

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
| `errorOnNoEligibleSubslice` | `bool` | Whether to throw an error if there are no eligible subslices to schedule | `false` |
| `subsliceSelectionBehaviour` | `roundRobin \| equalTime` | How to select subslices to schedule | `roundRobin` |


<!-- #### Tasks

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
| `defaultSubtaskTime` | `int` | Default time to spend on each subtask | `totalTime / subtasks.length` | -->

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