THIS ENTIRE FEATURE NEEDS TO BE REWRITTEN IN THE LANGUAGE OF COOKING

# ToDo List

The ToDo list feature is meant to supply the LLM with the ability to bread-down and track an effort in a step by step manner statefully. In a planning conv:wqersation, the LLM should be enabled to come up with and store a break-down of a task as a list, store it, check off each of the tasks as it goes, and complete the list.

There is more naunce to this, though. The modification of this list should be allowed to be piece-meal. A task to complete should be able to be added anywhere in the list. An of the tasks should be adjustable.
t
Most implortantly, a task should be also be able to be broken down further into another ToDo list. In this, a ToDo list is recursively defined, and an LLM should be able to continue expanding tasks into ToDo lists until it knows it can accomplish a task without further break-down.

This could result in a tree-like structure where a single task is broken down as needed.

## Concepts

### Objective

An `objective` is a single task to complete. It should be accomplishable outright. If it is not, there are grounds to have it broken down further into a `list` of `objectives`.

#### Examples

##### Example 1

- Produce tests for the new function.

##### Example 2

- Refactor this class to be configurable.

##### Example 3

- Consolodate the routines by leveraging dependency injection.

### List

A `list` is a set of `objectives`. Ultimately, it is derived from a parent `objective`, where completing the `list` implicitely results in completing the parent `objective` without any further action.

#### Examples



