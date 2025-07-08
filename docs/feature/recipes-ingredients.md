# Recipies and Ingrediants

In a coding task, there are definitively inputs and outputs. I would like to create a system for this in the MCP but for an LLM, and use the MCP as a way to assit in serving this functionality.

Much like a in a cookbook making ingredients into a meal by following a recipe, I am imagining an LLM following a set of instructions to make a particular result happen given a particular set of inputs. But, due to the imperfect nature of cooking, differing ingredients in type and quality, as well as the particular nuance in how a recipe is followed, creates a different outcome in the final dish, everytime.

This is why I would like to describe this feature as such. I would like to be able to assign resources, files, or what have you (I expect just files) to ingredient concepts, and apply a recipe to them.

## Notes

- May in the future, detect the nature of the ingredients automatically, as well as pull a recipe for an objective automatically as well.
- This effort may also need a supplementary effort of detecting whether a file or resource fits an ingredient description well.
- In terms of personalization, I would like to support people being able to write their own recipes and currate them for their own project or general needs.

## Concepts

### Ingredient

A some file or resource which is to be leveraged in the recipe.

### Recipe

A ToDo list, fundamentally. One of the things that I would like here is the ability to have these lists available as a RAG type resource. Instead of an LLM coming up with a list of things to do and to break down a task constantly, it should just see if there is a `recipe` available that already covers it's needs.
