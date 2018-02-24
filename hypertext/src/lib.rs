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

//#[derive(Debug)]
//struct Element {
//    pub name: String,
//    pub leading_text: Option<String>,
//    pub text: Option<String>,
//    pub children: Vec<Element>,
//    pub trailing_text: Option<String>,
//}

#[derive(Debug)]
enum Element {
    Element {
        name: String,
        children: Vec<Element>,
    },

    Text(String),
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
                Ok((p, elements)) => {
                    children.extend(elements);
                    parsee = p
                }
                Err(()) => break,
            }
        }

        (parsee, children)
    }

    fn element(self) -> Result<(Self, Vec<Element>), ()> {
        let parsee = self;

        let mut elements = vec![];

        let (parsee, leading_text) = parsee.text();
        if let Some(text) = leading_text {
            elements.push(Element::Text(text.into()));
        }

        let (parsee, name) = parsee.open_tag()?;

        let (parsee, text) = parsee.text();
        let (parsee, mut children) = parsee.elements();
        if let Some(text) = text {
            children.insert(0, Element::Text(text.into()));
        }

        let (parsee, close) = parsee.close_tag()?;

        let (parsee, trailing_text) = parsee.text();

        assert_eq!(name, close); // TODO: return Err()

        elements.push(Element::Element {
            name: name.into(),
            children,
        });

        if let Some(text) = trailing_text {
            elements.push(Element::Text(text.into()));
        }

        Ok((parsee, elements))
    }

    fn parse(self) -> Result<(Self, Vec<Element>), ()> {
        self.element()
    }
}

fn parse(input: &str) -> Result<Element, ()> {
    let (parsee, mut elements) = Parsee(input).parse()?;

    if !parsee.0.is_empty() {
        return Err(()); // only one root element allowed! (must parse all input)
    }

    if elements.len() != 1 {
        return Err(()); // only one root element allowed!
    }

    let element = elements.remove(0);
    println!("{:#?}", element);

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
            parse("<div> text <div>Hello!</div> more text <div>Test!</div> more </div>").is_ok()
        );
    }
}
