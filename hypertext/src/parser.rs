#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub block: String,
}

#[derive(Debug)]
pub enum Element {
    Element {
        name: String,
        attributes: Vec<Attribute>,
        children: Vec<Element>,
    },

    Text(String),
    Block(String),
}

#[derive(Clone, Debug)]
struct Parsee<'s>(&'s str);

impl<'s> Parsee<'s> {
    fn peek(&self) -> Option<char> {
        self.0.chars().next()
    }

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
        let count = self.0.chars().take_while(|&c| c != '<' && c != '{').count();

        let text = self.0[..count].trim();
        let text = if text.is_empty() { None } else { Some(text) };

        (Parsee(&self.0[count..]), text)
    }

    fn block(self) -> Result<(Self, &'s str), ()> {
        let parsee = self.spaces().tag("{")?;

        let mut stack = 1;
        let count = parsee
            .0
            .chars()
            .take_while(|&c| {
                match c {
                    '{' => stack += 1,
                    '}' => stack -= 1,
                    _ => {}
                }
                stack > 0
            })
            .count();

        let block = parsee.0[..count].trim();

        let count = count + 1; // count trailing '}'

        let parsee = Parsee(&parsee.0[count..]);
        let parsee = parsee.spaces();

        Ok((parsee, block))
    }

    fn attribute(self) -> Result<(Self, Attribute), ()> {
        let (parsee, attr) = self.spaces().identifier()?;
        let (parsee, block) = parsee.spaces().tag("=")?.block()?;

        Ok((
            parsee,
            Attribute {
                name: attr.into(),
                block: block.into(),
            },
        ))
    }

    fn attributes(self) -> (Self, Vec<Attribute>) {
        let mut attrs = vec![];
        let mut parsee = self;
        loop {
            let p = parsee.clone();
            match p.attribute() {
                Ok((p, attr)) => {
                    attrs.push(attr);
                    parsee = p
                }
                Err(()) => break,
            }
        }

        (parsee, attrs)
    }

    fn open_tag(self) -> Result<(Self, &'s str, Vec<Attribute>), ()> {
        let (parsee, name) = self.spaces().tag("<")?.spaces().identifier()?;

        let (parsee, attrs) = parsee.attributes();

        let parsee = parsee.spaces().tag(">")?;
        Ok((parsee, name, attrs))
    }

    fn close_tag(self) -> Result<(Self, &'s str), ()> {
        let (parsee, name) = self.spaces()
            .tag("<")?
            .spaces()
            .tag("/")?
            .spaces()
            .identifier()?;
        let parsee = parsee.spaces().tag(">")?.spaces();
        Ok((parsee, name))
    }

    fn elements(self) -> (Self, Vec<Element>) {
        let mut elements = vec![];
        let mut parsee = self;

        loop {
            let p = parsee.clone();

            // Parse a block, element, or text node

            let (p, element) = match p.peek() {
                Some('{') => {
                    let (p, block) = p.block().unwrap();
                    (p, Element::Block(block.into()))
                }

                Some('<') => match p.element() {
                    Ok((p, element)) => (p, element),
                    Err(()) => break,
                },

                Some(_) => {
                    let (p, text) = p.text();
                    (p, Element::Text(text.unwrap().into()))
                }

                None => break,
            };

            elements.push(element);
            parsee = p
        }

        (parsee, elements)
    }

    fn element(self) -> Result<(Self, Element), ()> {
        let parsee = self;

        let (parsee, name, attrs) = parsee.open_tag()?;

        let (parsee, children) = parsee.spaces().elements();

        let (parsee, close) = parsee.close_tag()?;

        assert_eq!(name, close); // TODO: return Err()

        Ok((
            parsee,
            Element::Element {
                name: name.into(),
                attributes: attrs,
                children,
            },
        ))
    }

    fn parse(self) -> Result<(Self, Element), ()> {
        self.element()
    }
}

pub fn parse(input: &str) -> Result<Element, ()> {
    let (parsee, element) = Parsee(input).parse()?;

    if !parsee.0.is_empty() {
        return Err(()); // only one root element allowed! (must parse all input)
    }

    // println!("{:#?}", element);

    Ok(element)
}

#[cfg(test)]
mod tests {
    use parser::parse;

    #[test]
    fn basic_parse() {
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

    #[test]
    fn attributes() {
        assert!(parse(r#"<div attr1={"test"} attr2={|| 42}></div>"#).is_ok());
        assert!(parse(r#"<div attr1={{ 42 }}></div>"#).is_ok());
    }

    //    #[test]
    //    fn self_closing_tag() {
    //        assert!(parse("<div />").is_ok());
    //    }

    #[test]
    fn buttons() {
        assert!(
            parse(
                "<div>
                   <button click={Message::Increment}>+</button>
                   <div>{model}</div>
                   <button click={Message::Decrement}>-</button>
                 </div>"
            ).is_ok()
        );

        assert!(parse("< div > < / div >").is_ok());
        assert!(parse("< div > < button click = { Message :: Increment } > + < / button > < div > { model } < / div > < button click = { Message :: Decrement } > - < / button > < / div >").is_ok());
    }

    #[test]
    fn embedded_block() {
        assert!(parse("<div>{model}</div>").is_ok());
        assert!(parse("<div>{model} HEY {test}</div>").is_ok());
    }
}
