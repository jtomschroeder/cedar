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
}

struct Parsee<'s>(&'s str);

impl<'s> Parsee<'s> {
    fn spaces(self) -> Parsee<'s> {
        Parsee(self.0.trim_left())
    }

    fn tag(self, text: &str) -> Result<Parsee<'s>, ()> {
        if self.0.starts_with(text) {
            Ok(Parsee(&self.0[text.len()..]))
        } else {
            Err(())
        }
    }

    fn identifier(self) -> Result<(Parsee<'s>, &'s str), ()> {
        match self.0.chars().take_while(|c| c.is_alphanumeric()).count() {
            0 => Err(()),
            count => Ok((Parsee(&self.0[count..]), &self.0[..count])),
        }
    }

    fn text(self) -> Result<(Parsee<'s>, &'s str), ()> {
        let count = self.0.chars().take_while(|&c| c != '<').count();
        Ok((Parsee(&self.0[count..]), &self.0[..count]))
    }

    fn parse(self) -> Result<(Element, Self), ()> {
        let parsee = self;

        let (parsee, name) = parsee.spaces().tag("<")?.identifier()?;
        let (parsee, text) = parsee.tag(">")?.text()?;

        // try! parser-combinator pattern
        let p = parsee.0;
        let (child, parsee) = match parsee.parse() {
            Ok((element, parsee)) => (Some(element), parsee),
            Err(()) => (None, Parsee(p)),
        };

        let (parsee, closing) = parsee.spaces().tag("</")?.identifier()?;
        let parsee = parsee.tag(">")?;

        assert_eq!(name, closing); // TODO: return Err()

        println!("{:?}", (name, text, child, closing));

        Ok((Element { name: name.into() }, parsee))
    }
}

fn parse(input: &str) -> Result<Element, ()> {
    let (element, _) = Parsee(input).parse()?;
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

        println!("{:?}", parse("<div></div>"));
    }
}
