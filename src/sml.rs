use crate::dom;
#[macro_export]
macro_rules! sml_attr {
    (click $value:expr) => {
        $crate::dom::Attribute::Click($value)
    };

    (input $value:expr) => {
        $crate::dom::Attribute::input($value)
    };

    (keydown $value:expr) => {
        $crate::dom::Attribute::keydown($value)
    };

    ($name:ident $value:expr) => {
        $crate::dom::Attribute::String {
            name: stringify!($name).to_string(),
            value: $value.to_string(),
        }
    };
}

#[macro_export]
macro_rules! sml {

    (@inc $object:expr => ) => { $object };

    (
        @inc
        $object:expr =>
        (@ $(( $attr_name:ident $attr_value:expr ))+ )
        $($body:tt)*
    ) => {{
        let obj = $object $( .attr( $crate::sml_attr!($attr_name $attr_value) ) )*;
        sml!(@inc obj => $($body)* )
    }};

    (
        @inc
        $object:expr =>
        ( $child:ident $($tail:tt)* )
        $($body:tt)*
    ) => {{
        let obj = $object .push( sml!(( $child $($tail)* )) );
        sml!(@inc obj => $($body)* )
    }};

    (
        @inc
        $object:expr =>
        $value:block
        $($body:tt)*
    ) => {{
        let obj = $object .push( $value );
        sml!(@inc obj => $($body)* )
    }};

    (
        @inc
        $object:expr =>
        (& $component:ident $($tail:tt)* )
        $($body:tt)*
    ) => {{
        let obj = $object .push( $component.render(()) );
        sml!(@inc obj => $($body)* )
    }};

    ((
        $name:ident
        $($body:tt)*
    )) => {
        sml!(@inc $crate::dom::Object::new(stringify!($name)) => $($body)* )
    };
}

// TODO: special syntax for components => maybe (& Thing)
// TODO: special syntax for list of components => maybe (* ...)

pub trait Component<S> {
    fn render(&self, props: ()) -> dom::Object<S>;
}

#[cfg(test)]
mod tests {
    use crate::dom::Object;
    use crate::sml::Component;

    fn dbg(object: Object<()>) {
        dbg!(object);
    }

    #[test]
    fn test_expression() {
        // TODO: build out tests here (add asserts)

        dbg(sml! {
            (tag)
        });

        dbg(sml! {
            (tag (@ (id "tag") (class "some-class")))
        });

        // <tag attr1="value1"
        //      attr2="value2">
        //   <nested>Text node</nested>
        //   <empty/>
        // </tag>

        dbg(sml! {
            (tag (@ (id "tag"))
                (nested { "Text node" })
                (end)
            )
        });

        dbg(sml! {
            (tag (@ (id "tag"))
                (nested { "Text node" })
                {"hello"}
            )
        });

        dbg(sml! {
            (tag (@ (id "tag"))
                (nested { "Text node" })
                {"hello"}
                (nested { "Text node" })
                {"hello"}
                (end)
            )
        });

        struct Custom;
        impl Component<()> for Custom {
            fn render(&self, props: ()) -> Object<()> {
                sml! { (div { "Hello" }) }
            }
        }

        dbg(sml! {
            (tag (@ (id "tag"))
                (nested { "Text node" })

                (& Custom (@ (id "custom")))

                (end)
            )
        });

        assert_eq!(1, 1);
    }
}
