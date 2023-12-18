# Day.rs: Overview

Throughout our lives we constantly face the challenge of working towards long term goals while trying to balance our time-constrained everyday lives. Day is a tool that allows to specify what your time should look like declaratively and deliberately allowing it to schedules your time accordingly.

As part of this, day augments or replaces your traditional calendaring and todo systems.

## What can it do
Automatically managing and scheduling tasks and sleep around your everyday commitments. This can be anything from a standard list of To-Dos with or without deadlines to a complex schedule of constrained events, on-going projects and recurring tasks, with special considerations for optimally partioning sleep, exercise, meals, work, socialising and rest in a way that's both productive and sustainable long-term.

## Crates
There are three crates in this repo:
- `day_core` contains all the types and scheduling logic and is partitioned into a separate crate to allow for non-cli interfaces in the future.
- `day_cli` contains all the cli logic, including the storage of state and configuration files

## Installation
`cargo install day_cli`


# Inbuilt Modules
Day is configured through the use of modules, each of which interacts with the scheduler in a (mostly) configurable order in order to prioritise more important tasks.m

The only inbuilt module currently is the sleep module, which automatically takes priority over everything else (unless compromising on sleep is unavoidable due to a commitment / deadline).

# Custom Modules
The true power of day comes with it's custom module system, allowing you to systematically model a graph like structure defining how you might want to spend your time. There are several types of custom modules, each of which can define daily/weekly requirements/goals for time spent or sessions scheduled:

- Commitments: These are events at specific times outside of your control, usually imported from an external calendar.
- Slice: This is the type of module that contain the actual tasks with variable scheduling
- Parent: Define a module with child modules with descendent slices having a time/session weighting to decide how much of each are scheduled in the parent's time.

### Options
`id`: A string identifier for the module
#### `dependencies`: Modules that are only scheduled when this one is scheduled

`when`:
- Every occurence
- Any occurence every (duration)
- Lack of occurence

`scheduling`:
- Just (before | after)
- At (duration) (before | after)
- At any point (before | after) in (calendar unit) with (closer | further) being preferred
- Anytime in (calendar unit)

`required`: If there isn't enough time to schedule the child, setting required to false will still mean that the parent can be scheduled

## Commitments
This is a module made up of things that you (ideally) have to go to.

### Options


## Todo
A todo module is a specific list of tasks.

### Options
`repetition`:
- None
- Circular (when the list is complete, every event in the list becomes available for completion)
- Scattered Random (every task is always available for completion, however each task is randomly chosen based on their weightings)
- Scattered Historical (every task is always available for completion, however each task is roughly chosen as many times as their weighting would imply by looking at historical completion)



# Example
```
sleep {
    dependencies [
        {
            id: brush_teeth
            scheduling: just before
            required: true
        },
        {
            id: brush_teeth
            scheduling: just after
            required: true
        }
    ]
}
brush_teeth {
    tasks: [
        Brush Your Teeth {
            duration: 3
        }
    ]
    repetition: circular
    min: 2
    max: 2
    per: day
}
shower {
    tasks {
        Take A Shower {
            duration: 3
        }
    }
    min: 1
    max: 2
    per: day
}
exercise {
    // ICAL
    minimum_attendance: 80%
    dependencies {
        {
            id: shower
            scheduling: At any point after during day with closer preferred,
            required: true,
        }
    }
}
studying {
    children [
        Subject 1 {
            todos [...]
        }
        Subject 2 {
            todos [...]
        }
        Subject 3 {
            todos [...]
        }
        Subject 4 {
            todos [...]
        }
    ]
    max: 12 Hours
    per: Day 
}
```

