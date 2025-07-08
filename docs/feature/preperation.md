# Preparation Planning

The Preparation Planning feature enables the LLM to break down and track culinary efforts in a step-by-step manner statefully. In a planning conversation, the LLM should be enabled to come up with and store a break-down of a cooking technique as a preparation, store it, check off each of the techniques as it goes, and complete the preparation.

There is more nuance to this, though. The modification of this preparation should be allowed to be piece-meal. A technique to master should be able to be added anywhere in the preparation. Any of the techniques should be adjustable.

Most importantly, a technique should also be able to be broken down further into another preparation. In this, a preparation is recursively defined, and an LLM should be able to continue expanding techniques into preparations until it knows it can accomplish a technique without further break-down.

This could result in a tree-like structure where a single technique is broken down as needed.

## Concepts

### Technique

A `technique` is a single cooking method to master. It should be accomplishable outright. If it is not, there are grounds to have it broken down further into a `preparation` of `techniques`.

#### Examples

##### Example 1

- Perfect the knife skills for brunoise dicing.

##### Example 2

- Master the emulsification process for hollandaise sauce.

##### Example 3

- Develop consistent temperature control for searing proteins.

### Preparation

A `preparation` is a set of `techniques`. Ultimately, it is derived from a parent `technique`, where completing the `preparation` implicitly results in mastering the parent `technique` without any further action.

#### Examples



