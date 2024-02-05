// Copyright 2015 Google Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! HTML renderer that takes an iterator of events as input.

use pulldown_cmark::{
    escape::{escape_href, escape_html, StrWrite},
    Alignment, CodeBlockKind, CowStr,
    Event::{self, *},
    HeadingLevel, LinkType, Options, Parser, Tag,
};

use std::collections::HashMap;
use std::io;

enum TableState {
    Head,
    Body,
}

struct HtmlWriter<'a, I, W> {
    /// Iterator supplying events.
    iter: I,

    /// Writer to write to.
    writer: W,

    /// Whether or not the last write wrote a newline.
    end_newline: bool,

    /// In a comment block.
    in_comment: bool,

    /// Header level
    header_level: u8,

    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    numbers: HashMap<CowStr<'a>, usize>,
}

impl<'a, I, W> HtmlWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite,
{
    fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            in_comment: false,
            header_level: 0,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
        }
    }

    /// Writes a new line.
    fn write_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    /// Writes a buffer, and tracks whether or not a newline was written.
    #[inline]
    fn write(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_str(s)?;

        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }
        Ok(())
    }

    fn run(mut self) -> io::Result<u8> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    escape_html(&mut self.writer, &text)?;
                    self.end_newline = text.ends_with('\n');
                }
                Code(text) => {
                    self.write("<code>")?;
                    escape_html(&mut self.writer, &text)?;
                    self.write("</code>")?;
                }
                Html(html) => {
                    self.write(&html)?;
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("<br />\n")?;
                }
                Rule => {
                    if !self.end_newline {
                        self.write("\n")?;
                    }
                    if self.in_comment {
                        self.write("</section>\n")?;
                    } else {
                        self.write("<section class='comment'/>\n")?;
                    }
                    self.in_comment = !self.in_comment;
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    self.write("<sup class='footnote-reference'><a href='#")?;
                    escape_html(&mut self.writer, &name)?;
                    self.write("'>")?;
                    let number = *self.numbers.entry(name).or_insert(len);
                    write!(&mut self.writer, "{}", number)?;
                    self.write("</a></sup>")?;
                }
                TaskListMarker(true) => {
                    self.write("<input disabled='' type='checkbox' checked=''/>\n")?;
                }
                TaskListMarker(false) => {
                    self.write("<input disabled='' type='checkbox'/>\n")?;
                }
            }
        }
        Ok(self.header_level)
    }

    /// Writes the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                if self.end_newline {
                    self.write("<p>")
                } else {
                    self.write("\n<p>")
                }
            }
            Tag::Heading(level, id, classes) => {
                if self.end_newline {
                    self.end_newline = false;
                } else {
                    self.write("\n")?;
                }

                if !self.in_comment {
                    if self.header_level >= h_to_num(level) {
                        let close_sections = (0..self.header_level - h_to_num(level))
                            .fold("</div>\n".to_string(), |res, _| res + "</div>\n");
                        self.write(&close_sections)?;
                    }
                    self.write("<div>")?;
                    self.header_level = h_to_num(level);
                }

                write!(&mut self.writer, "<{}", level)?;
                if let Some(id) = id {
                    self.write(" id='")?;
                    escape_html(&mut self.writer, id)?;
                    self.write("'")?;
                }
                let mut classes = classes.iter();
                if let Some(class) = classes.next() {
                    self.write(" class='")?;
                    escape_html(&mut self.writer, class)?;
                    for class in classes {
                        self.write(" ")?;
                        escape_html(&mut self.writer, class)?;
                    }
                    self.write("'")?;
                }
                self.write(">")
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                self.write("<table>")
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                self.write("<thead><tr>")
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.write("<tr>")
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("<th")?;
                    }
                    TableState::Body => {
                        self.write("<td")?;
                    }
                }
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => self.write(" style='text-align: left'>"),
                    Some(&Alignment::Center) => self.write(" style='text-align: center'>"),
                    Some(&Alignment::Right) => self.write(" style='text-align: right'>"),
                    _ => self.write(">"),
                }
            }
            Tag::BlockQuote => {
                if self.end_newline {
                    self.write("<blockquote>\n")
                } else {
                    self.write("\n<blockquote>\n")
                }
            }
            Tag::CodeBlock(info) => {
                if !self.end_newline {
                    self.write_newline()?;
                }
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        if lang.is_empty() {
                            self.write("<pre><code>")
                        } else {
                            self.write("<pre><code class='language-")?;
                            escape_html(&mut self.writer, lang)?;
                            self.write("'>")
                        }
                    }
                    CodeBlockKind::Indented => self.write("<pre><code>"),
                }
            }
            Tag::List(Some(1)) => {
                if self.end_newline {
                    self.write("<ol>\n")
                } else {
                    self.write("\n<ol>\n")
                }
            }
            Tag::List(Some(start)) => {
                if self.end_newline {
                    self.write("<ol start='")?;
                } else {
                    self.write("\n<ol start='")?;
                }
                write!(&mut self.writer, "{}", start)?;
                self.write("'>\n")
            }
            Tag::List(None) => {
                if self.end_newline {
                    self.write("<ul>\n")
                } else {
                    self.write("\n<ul>\n")
                }
            }
            Tag::Item => {
                if self.end_newline {
                    self.write("<li>")
                } else {
                    self.write("\n<li>")
                }
            }
            Tag::Emphasis => self.write("<em>"),
            Tag::Strong => self.write("<strong>"),
            Tag::Strikethrough => self.write("<del>"),
            Tag::Link(LinkType::Email, dest, title) => {
                self.write("<a href='mailto:")?;
                escape_href(&mut self.writer, &dest)?;
                if !title.is_empty() {
                    self.write("' title='")?;
                    escape_html(&mut self.writer, &title)?;
                }
                self.write("'>")
            }
            Tag::Link(_link_type, dest, title) => {
                self.write("<a href='")?;
                escape_href(&mut self.writer, &dest)?;
                if !title.is_empty() {
                    self.write("' title='")?;
                    escape_html(&mut self.writer, &title)?;
                }
                self.write("'>")
            }
            Tag::Image(_link_type, dest, title) => {
                self.write("<img src='")?;
                escape_href(&mut self.writer, &dest)?;
                self.write("' alt='")?;
                self.raw_text()?;
                if !title.is_empty() {
                    self.write("' title='")?;
                    escape_html(&mut self.writer, &title)?;
                }
                self.write("' />")
            }
            Tag::FootnoteDefinition(name) => {
                if self.end_newline {
                    self.write("<div class='footnote-definition' id='")?;
                } else {
                    self.write("\n<div class='footnote-definition' id='")?;
                }
                escape_html(&mut self.writer, &*name)?;
                self.write("'><sup class='footnote-definition-label'>")?;
                let len = self.numbers.len() + 1;
                let number = *self.numbers.entry(name).or_insert(len);
                write!(&mut self.writer, "{}", number)?;
                self.write("</sup>")
            }
        }
    }

    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                self.write("</p>\n")?;
            }
            Tag::Heading(level, _id, _classes) => {
                self.write("</")?;
                write!(&mut self.writer, "{}", level)?;
                self.write(">\n")?;
            }
            Tag::Table(_) => {
                self.write("</tbody></table>\n")?;
            }
            Tag::TableHead => {
                self.write("</tr></thead><tbody>\n")?;
                self.table_state = TableState::Body;
            }
            Tag::TableRow => {
                self.write("</tr>\n")?;
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("</th>")?;
                    }
                    TableState::Body => {
                        self.write("</td>")?;
                    }
                }
                self.table_cell_index += 1;
            }
            Tag::BlockQuote => {
                self.write("</blockquote>\n")?;
            }
            Tag::CodeBlock(_) => {
                self.write("</code></pre>\n")?;
            }
            Tag::List(Some(_)) => {
                self.write("</ol>\n")?;
            }
            Tag::List(None) => {
                self.write("</ul>\n")?;
            }
            Tag::Item => {
                self.write("</li>\n")?;
            }
            Tag::Emphasis => {
                self.write("</em>")?;
            }
            Tag::Strong => {
                self.write("</strong>")?;
            }
            Tag::Strikethrough => {
                self.write("</del>")?;
            }
            Tag::Link(_, _, _) => {
                self.write("</a>")?;
            }
            Tag::Image(_, _, _) => (), // shouldn't happen, handled in start
            Tag::FootnoteDefinition(_) => {
                self.write("</div>\n")?;
            }
        }
        Ok(())
    }

    // run raw text, consuming end tag
    fn raw_text(&mut self) -> io::Result<()> {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(text) | Code(text) | Text(text) => {
                    escape_html(&mut self.writer, &text)?;
                    self.end_newline = text.ends_with('\n');
                }
                SoftBreak | HardBreak | Rule => {
                    self.write(" ")?;
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name).or_insert(len);
                    write!(&mut self.writer, "[{}]", number)?;
                }
                TaskListMarker(true) => self.write("[x]")?,
                TaskListMarker(false) => self.write("[ ]")?,
            }
        }
        Ok(())
    }
}

fn h_to_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// push it to a `String`.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let parser = Parser::new(markdown_str);
///
/// let mut html_buf = String::new();
/// html::push_html(&mut html_buf, parser);
///
/// assert_eq!(html_buf, r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
fn push_html<'a, I>(s: &mut String, iter: I) -> u8
where
    I: Iterator<Item = Event<'a>>,
{
    HtmlWriter::new(iter, s).run().unwrap()
}

/// Convert the content of a markdown string to an HTML string.
fn md_content_into_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();

    let div_to_close = push_html(&mut html_output, parser);
    for _ in 0..div_to_close {
        html_output.push_str("</div>\n");
    }

    html_output
}

const HEAD: &str = "<!DOCTYPE html><html lang='it'>
<head>
	<meta charset='utf-8' />
	<meta name='viewport' content='width=device-width, initial-scale=1' />
	<title>{{ TITLE }}</title>
	<link rel='stylesheet' href='assets/main.css' />
	<script type='module' src='assets/main.js'></script>
    <link rel='stylesheet' href='https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css' integrity='sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV' crossorigin='anonymous'>
<script defer src='https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js' integrity='sha384-XjKyOOlGwcjNTAIQHIpgOno0Hl1YQqzUOEleOLALmuqehneUG+vnGctmUb0ZY0l8' crossorigin='anonymous'></script>
 <script defer src='https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js' integrity='sha384-+VBxd3r6XgURycqtZ117nYw44OOcIax56Z4dCRWbxyPt0Koah1uHoK0o4+/RRE05' crossorigin='anonymous'></script>
    <script>
        document.addEventListener('DOMContentLoaded', function() {
            renderMathInElement(document.body, {
              // customised options
              delimiters: [
                  {left: '$$', right: '$$', display: true},
                  {left: '$', right: '$', display: false},
                  {left: '\\\\(', right: '\\\\)', display: false},
                  {left: '\\\\[', right: '\\\\]', display: true},
                  {left: '\\\\begin{equation}', right: '\\\\end{equation}', display: true},
                  {left: '\\\\begin{align}', right: '\\\\end{align}', display: true},
                  {left: '\\\\begin{alignat}', right: '\\\\end{alignat}', display: true},
                  {left: '\\\\begin{gather}', right: '\\\\end{gather}', display: true},
                  {left: '\\\\begin{CD}', right: '\\\\end{CD}', display: true},
                  {left: '\\\\[', right: '\\\\]', display: true}
              ],
              throwOnError : false
            });
        });
    </script>
</head>

<body>
";

const FOOT: &str = "
	<a class='clickable-area' id='top' />
	<a class='clickable-area' id='right' />
	<a class='clickable-area' id='left' />
	<a class='clickable-area' id='bottom' />
</body>
</html>";

pub fn to_html(md: &mut str) -> String {
    let mut html = md_content_into_html(md);
    html = HEAD.to_string() + &html + FOOT;
    html
}
