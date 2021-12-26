# README

This is a very trivial preprocessor for mdbook which simple does a search and replace.
On its own it takes as stdin markdown text with some additional and converts certain substrings to HTML.

I am using it since I want to support features not easily available in markdown:

- text color {blue}blue font{/blue}
- small text {small}subscript font{/small}
- collapsable question/answer blocks where the answer is hidden until you click the dropdown arrow.
```
?Q Your question goes
   here until we get to the answer
?A You answer goes here until we get to the end
   but it is hidden so that the reader has a chance
   to think before they reveal the answer
?E
```

## mdbook-plantuml

The <img> element that is generated does not allow for embedded hyperlinks to be clickable. This needs to be changed to a <object> tag. This postprocessor will change the generated html.

## Examples

After running through the preprocessor these will be converted to appropriate HTML.

- {question}This is a question?{answer}Here is the hidden answer in a clickable dropdown{/question}
using the html details/summary tags.
- {small}smaller font{/small}
- {red}red text{/red}

## Install

`cargo install --path . --bin mdbook-plus`

## Limitations

- Since these special markers in the input markdown files are not legitimate for normal markdown, intellisense in VS Code (for example) may flag lines with warnings.
- The preprocessing is just dumb string replacement, so if you ever need the actual marker strings and not the replacement values in your output you may be hamstrung since this does not any escaping capabillities.
- Note: `code in backticks` does not render correctly inside color blocks.
