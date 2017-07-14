
#[macro_use]
extern crate dom;

fn main() {
    objects();
}

fn objects() {
    use dom::Kind::*;
    use dom::Attribute::*;

    {
        let t = node![Stack];
        let u = node![Stack];

        let changeset = dom::diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);
    }

    {
        let t = node![Stack];
        let u = node![Button];

        let changeset = dom::diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);
    }

    {
        let t = node![Label |> Text("".into())];
        let u = node![Label |> Text("!".into())];

        let changeset = dom::diff(vec![t], vec![u]);
        println!("changeset: {:?}", changeset);
    }

    {
        let u = node![Stack => node![Button]
                             , node![Label |> Text("!".into())]
                             , node![Button]
                     ];

        let changeset = dom::diff(vec![], vec![u]);
        println!("changeset: {:#?}", changeset);
    }
}