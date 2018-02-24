#![allow(dead_code)]
#![feature(proc_macro)]

extern crate proc_macro as pm;

#[proc_macro]
pub fn hypertext(input: pm::TokenStream) -> pm::TokenStream {
    let input = input.to_string();
    format!("::browser::log(\"Hello, world: '{}'\")", input)
        .parse()
        .unwrap()
}

#[derive(Debug)]
struct Element {
    pub name: String,
    pub leading_text: Option<String>,
    pub text: Option<String>,
    pub children: Vec<Element>,
    pub trailing_text: Option<String>,
}

#[derive(Clone, Debug)]
struct Parsee<'s>(&'s str);

impl<'s> Parsee<'s> {
    fn spaces(self) -> Self {
        Parsee(self.0.trim_left())
    }

    fn tag(self, text: &str) -> Result<Self, ()> {
        if self.0.starts_with(text) {
            Ok(Parsee(&self.0[text.len()..]))
        } else {
            Err(())
        }
    }

    fn identifier(self) -> Result<(Self, &'s str), ()> {
        match self.0.chars().take_while(|c| c.is_alphanumeric()).count() {
            0 => Err(()),
            count => Ok((Parsee(&self.0[count..]), &self.0[..count])),
        }
    }

    fn text(self) -> (Self, Option<&'s str>) {
        let count = self.0.chars().take_while(|&c| c != '<').count();

        let text = self.0[..count].trim();
        let text = if text.is_empty() { None } else { Some(text) };

        (Parsee(&self.0[count..]), text)
    }

    fn open_tag(self) -> Result<(Self, &'s str), ()> {
        let (parsee, name) = self.spaces().tag("<")?.identifier()?;
        let parsee = parsee.tag(">")?;
        Ok((parsee, name))
    }

    fn close_tag(self) -> Result<(Self, &'s str), ()> {
        let (parsee, name) = self.spaces().tag("</")?.identifier()?;
        let parsee = parsee.tag(">")?.spaces();
        Ok((parsee, name))
    }

    fn elements(self) -> (Self, Vec<Element>) {
        let mut children = vec![];
        let mut parsee = self;
        loop {
            let p = parsee.clone();
            match p.element() {
                Ok((p, element)) => {
                    children.push(element);
                    parsee = p
                }
                Err(()) => break,
            }
        }

        (parsee, children)
    }

    fn element(self) -> Result<(Self, Element), ()> {
        let parsee = self;

        let (parsee, leading_text) = parsee.text();

        let (parsee, name) = parsee.open_tag()?;

        let (parsee, text) = parsee.text();

        // try! parser-combinator pattern
        // let p = parsee.0;
        // let (parsee, _child) = match parsee.element() {
        //     Ok((parsee, element)) => (parsee, Some(element)),
        //     Err(()) => (Parsee(p), None),
        // };

        let (parsee, children) = parsee.elements();

        let (parsee, close) = parsee.close_tag()?;

        let (parsee, trailing_text) = parsee.text();

        assert_eq!(name, close); // TODO: return Err()

        Ok((
            parsee,
            Element {
                name: name.into(),
                leading_text: leading_text.map(String::from),
                text: text.map(String::from),
                children,
                trailing_text: trailing_text.map(String::from),
            },
        ))
    }

    fn parse(self) -> Result<(Self, Element), ()> {
        self.element()
    }
}

fn parse(input: &str) -> Result<Element, ()> {
    let (parsee, element) = Parsee(input).parse()?;

    println!("{:#?}", element);

    if !parsee.0.is_empty() {
        return Err(()); // only one root element allowed!
    }

    Ok(element)
}

#[cfg(test)]
mod tests {
    use parse;

    #[test]
    fn parser() {
        assert!(parse("--").is_err());
        assert!(parse("<div></div>").is_ok());
        assert!(parse("<div>Hello, world!</div>").is_ok());
        assert!(parse("<div>Hello, world! <div></div> </div>").is_ok());

        assert!(
            parse(
                "<div></div>
                 <div></div>"
            ).is_err()
        );
    }

    #[test]
    fn nested() {
        assert!(
            parse(
                "<div> text
                   <div>Hello!</div>
                   <div>Test</div>
                 </div>"
            ).is_ok()
        );
    }

    #[test]
    fn text_around_child() {
        assert!(parse("<div> text <div>Hello!</div> more text </div>").is_ok());
        assert!(
            parse("<div> text <div>Hello!</div> more text <div>Hello!</div> more </div>").is_ok()
        );
    }
}
