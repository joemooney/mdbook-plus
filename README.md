# README

This is a very trivial preprocessor for mdbook which simple does a search and replace.
On its own it takes as stdin (pseudo) markdown text and converts certain substrings to HTML.

I am using it since I want to support features not easily available in markdown:

- text color {small}smaller font{/small}
- small text
- collapsable question/answer blocks

## Examples

After running through the preprocessor these will be converted to appropriate HTML.

- {question}This is a question?{answer}Here is the hidden answer in a clickable dropdown{/question}
- {small}smaller font{/small}
- {red}red text{/red}

## Install

`cargo install --path . --bin mdbook-plus`

## Limitations

- Technically, input "markdown" files are a **hacked** markdown with my own special markers. So intellisense in VS Code (for example) for markdown may flag lines with warnings.
- The preprocessing is not smart, I will get to that once I have the need.
- If you want to keep these matching substrings in your text there is no way to escape them.
- `code in backticks` does not render correctly inside color blocks.
