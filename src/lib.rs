use mdbook::book::{Book, BookItem, Chapter};
//use mdbook::errors::Error;
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
//use pulldown_cmark::Tag::*;
//use pulldown_cmark::{Event, Options, Parser};
//use pulldown_cmark_to_cmark::{cmark_with_options, Options as COptions};

//#[macro_use]
//extern crate log;

pub struct MDBookPlus;

impl Preprocessor for MDBookPlus {
    fn name(&self) -> &str {
        "plus"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(MDBookPlus::search_and_replace(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }
}

/*
            s,{timestamp},$(ts),g;
            s,{/small},</sub>,g; s,{small},<sub>,g;
            s,{question},<details><summary>Q: ,g; s,{/question},</details>,g; s,{small},<sub>,g;
            s,{answer},</summary>,g; s,{/question},</details>,g; s,{small},<sub>,g;
            s,{red},<span style='color:red'>,g; s,{/red},</span>,g;
            s,{blue},<span style='color:blue'>,g; s,{/blue},</span>,g;
            s,{green},<span style='color:green'>,g; s,{/green},</span>,g;
            s,{yellow},<span style='color:yellow'>,g; s,{/yellow},</span>,g;
            s,{gray},<span style='color:gray'>,g; s,{/gray},</span>,g;
            s,{code},<code>,g; s,{/code},</code>,g;

*/
/*
fn build_toc<'a>(toc: &[(u32, String)]) -> String {
    log::trace!("MDBookPlus from {:?}", toc);
    let mut result = String::new();

    // "Normalize" header levels.
    // If headers skip a level, we need to normalize them to avoid the skip.
    // Otherwise the markdown render will escape nested levels.
    //
    // This is a rough approximation only.
    let mut last_lower = 0;
    let toc = toc.iter().map(|(lvl, name)| {
        let lvl = *lvl;
        let lvl = if last_lower + 1 == lvl {
            last_lower = lvl;
            lvl
        } else if last_lower + 1 < lvl {
            last_lower + 1
        } else {
            last_lower = lvl;
            lvl
        };
        (lvl, name)
    });

    for (level, name) in toc {
        let width = 2 * (level - 1) as usize;
        let slug = mdbook::utils::normalize_id(&name);
        let entry = format!("{1:0$}* [{2}](#{3})\n", width, "", name, slug);
        result.push_str(&entry);
    }

    result
}
*/

//fn replace_question_block(s: &str) -> String {
//    let mut o = String::from(s);

/// Replace {question}...{answer}...{/question} blocks with
/// HTML <details> tag. This is a summary with collapsed  
/// details block which serves as a question with a hidden
/// answer.
fn replace_question_block(s: String) -> String {
    if !s.contains("{question}") {
        return s;
    }
    let mut o = String::from(s);
    while o.contains("{question}") {
        o = o
            .replace("{question}", "<details><summary>Q: ")
            .replace("{answer}", "</summary>A: ")
            .replace("\n?Q", "<details><summary>Q: ")
            .replace("\n?A", "</summary>A: ")
            .replace("\n?E", "</details>")
            .replace("{/question}", "</details>");
    }
    o
}

/// Replace `...` with <code>...</code>
// fn replace_code_block(s: String) -> String {
//     if !s.contains("`") {
//         return s;
//     }
//     let mut o = String::from(s);
//     o = o.replace("```", "{three_backticks}");
//     while o.contains("`") {
//         o = o.replacen("`", "<code>", 1);
//         if !o.contains("`") {
//             warn!("Missing backtick");
//         }
//         o = o.replacen("`", "</code>", 1);
//     }
//     o = o.replace("{three_backticks}", "```");
//     o
// }

fn replace_colors(s: String) -> String {
    s.replace("{red}", "  <span style='color:red'>")
        .replace("{/red}", "</span>")
        .replace("{blue}", "<span style='color:lightblue'>")
        .replace("{/blue}", "</span>")
        .replace("{green}", "<span style='color:green'>")
        .replace("{/green}", "</span>")
        .replace("{yellow}", "<span style='color:yellow'>")
        .replace("{/yellow}", "</span>")
        .replace("{grey}", "<span style='color:gray'>")
        .replace("{/grey}", "</span>")
        .replace("{gray}", "<span style='color:gray'>")
        .replace("{/gray}", "</span>")
}

fn search_and_replace(content: &str) -> Result<String> {
    let s = content
        .replace("{code}", "<code>")
        .replace("{/code}", "</code>")
        .replace("{small}", "<sub>")
        .replace("{/small}", "</sub>")
        .replace("{small}", "<sub>")
        .replace("{/small}", "</sub>");

    let s = replace_colors(s);
    // let s = replace_code_block(s);
    let s = replace_question_block(s);

    let s = s
        .replace("{question}", "<details><summary>Q: ")
        .replace("{answer}", "</summary>A: ")
        .replace("\n?Q", "<details><summary>Q: ")
        .replace("\n?A", "</summary>A: ")
        .replace("\n?E", "</details>")
        .replace("{/question}", "</details>");

    let s = s
        .replace("{pseudo}", "")
        .replace("{pseudo-text}", "<details><summary>Pseudo Code</summary>")
        .replace(
            "{pseudo-code}",
            "</details><details><summary>Solution</summary>",
        )
        .replace("{/pseudo}", "</details>");

    let s = s
        .replace("[#dash]", "\\-")
        .replace("[#open_bracket]", "{")
        .replace("[#close_bracket]", "}")
        .replace("[#backtick]", "`")
        .replace("[#question_mark]", "?");

    // <img src="mdbook-plantuml-img/xxx.svg" alt="" /></p>


    let s = s
        .replace("<img", "\\-")
        .replace("[#open_bracket]", "{")
        .replace("[#close_bracket]", "}")
        .replace("[#backtick]", "`")
        .replace("[#question_mark]", "?");

    // Uncomment if you want to print updates if content changed
    // if s != content {
    //     eprintln!("mdbook-plus updated <<{}>>", s)
    // }

    // Convert <img.../> to <object.../> for plantuml images so that hyperlinks work
    let mut s = s;
    //let re = Regex::new(r#"<img src="(?P<src>mdbook-plantuml-img/.*)" alt="" />"#).unwrap();
    let re = Regex::new(r#"....(?P<src>mdbook-plantuml-img/.*.svg)."#).unwrap();
    if re.is_match(&s) {
        s = re.replace_all(&s, r#"<object data="$src" type="image/svg+xml" >$src</object>"#).to_string();
        //eprintln!("mdbook-plus updated <<{}>>", s)
    } else {
        //eprintln!("mdbook-plus not updated <<{}>>", s)
    }

    return Ok(s);
}

/*
fn search_and_replace2(content: &str) -> Result<String> {
    let mut buf = String::with_capacity(content.len());
    let mut toc_found = false;

    let mut toc_content = vec![];
    let mut current_header = vec![];
    let mut current_header_level: Option<u32> = None;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    for e in Parser::new_ext(&content, opts.clone()) {
        log::trace!("Event: {:?}", e);

        if let Event::Html(html) = e {
            if &*html == "<!-- toc -->\n" {
                toc_found = true;
            }
            continue;
        }
        if !toc_found {
            continue;
        }

        if let Event::Start(Heading(lvl)) = e {
            if lvl < 5 {
                current_header_level = Some(lvl);
            }
            continue;
        }
        if let Event::End(Heading(_)) = e {
            // Skip if this header is nested too deeply.
            if let Some(level) = current_header_level.take() {
                let header = current_header.join("");

                current_header.clear();
                toc_content.push((level, header));
            }
            continue;
        }
        if current_header_level.is_none() {
            continue;
        }

        match e {
            Event::Text(header) => current_header.push(header),
            Event::Code(code) => {
                let text = format!("`{}`", code);
                current_header.push(text.into());
            }
            _ => {} // Rest is unhandled
        }
    }

    let toc = build_toc(&toc_content);
    let toc_events = Parser::new(&toc).collect::<Vec<_>>();

    let events = Parser::new_ext(&content, opts)
        .map(|e| {
            if let Event::Html(html) = e.clone() {
                if &*html == "<!-- toc -->\n" {
                    return toc_events.clone();
                }
            }
            vec![e]
        })
        .flat_map(|e| e);

    let mut opts = COptions::default();
    opts.newlines_after_codeblock = 1;
    cmark_with_options(events, &mut buf, None, opts)
        .map(|_| buf)
        .map_err(|err| Error::from(format!("Markdown serialization failed: {}", err)))
}
*/

impl MDBookPlus {
    fn search_and_replace(chapter: &mut Chapter) -> Result<String> {
        search_and_replace(&chapter.content)
    }
}

/*
            s,{timestamp},$(ts),g;
*/
#[cfg(test)]
mod test {
    use super::search_and_replace;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_global_replace() {
        let content = r#"
{code}...{/code} {code}...{/code}
{small}...{/small} {small}...{/small}
{red}...{/red}
{blue}...{/blue}
{green}...{/green}
{yellow}...{/yellow}
{gray}...{/gray}
{grey}...{/grey}
"#;

        let expected = r#"
<code>...</code> <code>...</code>
<sub>...</sub> <sub>...</sub>
<span style='color:red'>...</span>
<span style='color:blue'>...</span>
<span style='color:green'>...</span>
<span style='color:yellow'>...</span>
<span style='color:gray'>...</span>
"#;

        assert_eq!(expected, search_and_replace(content).unwrap());
    }
}
