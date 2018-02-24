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

#[derive(Debug)]
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

    fn text(self) -> Result<(Self, &'s str), ()> {
        let count = self.0.chars().take_while(|&c| c != '<').count();
        Ok((Parsee(&self.0[count..]), &self.0[..count]))
    }

    fn element(self) -> Result<(Self, Element), ()> {
        let (parsee, name) = self.spaces().tag("<")?.identifier()?;
        let (parsee, text) = parsee.tag(">")?.text()?;

        let text = text.trim();

        // try! parser-combinator pattern
        let p = parsee.0;
        let (parsee, child) = match parsee.parse() {
            Ok((parsee, element)) => (parsee, Some(element)),
            Err(()) => (Parsee(p), None),
        };

        let (parsee, closing) = parsee.spaces().tag("</")?.identifier()?;
        let parsee = parsee.tag(">")?.spaces();

        assert_eq!(name, closing); // TODO: return Err()

        println!("{:?}", (name, text, child, closing));

        Ok((parsee, Element { name: name.into() }))
    }

    fn parse(self) -> Result<(Self, Element), ()> {
        self.element()
    }
}

fn parse(input: &str) -> Result<Element, ()> {
    let (parsee, element) = Parsee(input).parse()?;

    println!("{:?} {:?}", element, parsee);

    assert!(parsee.0.is_empty());

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

        //        assert!(
        //            parse(
        //                "<div></div>
        //                 <div></div>"
        //            ).is_ok()
        //        );

        //        assert!(
        //            parse(
        //                "<div></div>
        //                   <div></div>
        //                   <div></div>
        //                 <div></div>"
        //            ).is_ok()
        //        );

        println!("{:?}", parse("<div></div>"));
    }
}
