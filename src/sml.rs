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
        $crate::dom::Attribute::Value($value.to_string())
    };
}

#[macro_export]
macro_rules! sml_properties {

    ($properties:expr => ) => { $properties };

    (
        $properties:expr =>
        (@ $(( $attr_name:ident $attr_value:expr ))+ )
        $($body:tt)*
    ) => {{
        let mut props = $properties;

        $( props.attributes.insert(
                stringify!( $attr_name ).into(),
                $crate::sml_attr!($attr_name $attr_value)
        ); )*

        $crate::sml_properties!(props => $($body)*)
    }};

    (
        $properties:expr =>
        ( $child:ident $($tail:tt)* )
        $($body:tt)*
    ) => {{
        let mut props = $properties;
        props.children.push($crate::sml!(( $child $($tail)* )));
        $crate::sml_properties!(props => $($body)*)
    }};

    (
        $properties:expr =>
        $value:block
        $($body:tt)*
    ) => {{
        use $crate::dom::Pushable;

        let mut props = $properties;
        $value.pushed(&mut props.children);

        $crate::sml_properties!(props => $($body)*)
    }};

    (
        $properties:expr =>
        (& $component:ident $($tail:tt)* )
        $($body:tt)*
    ) => {{
        let mut props = $properties;
        props.children.push($crate::sml!((& $component $($tail)* )));
        $crate::sml_properties!(props => $($body)*)
    }};

}

#[macro_export]
macro_rules! sml_cc_properties {
    (
        $component:tt =>
        (@ $(( $attr_name:ident $attr_value:expr ))+ )
        $( $($body:tt)+ )?
    ) => {{
        $component {
            $( $attr_name : $attr_value .into() ),*,

            $(children: {
                let props = $crate::dom::Properties::default();
                let props = $crate::sml_properties!(props => $($body)*);
                props.children
            })?

            // TODO: some way to have ..Default::default() for optional fields
        }
    }};

    (
        $component:tt =>
        $( $($body:tt)+ )?
    ) => {{

        $component {
            $(children: {
                let props = $crate::dom::Properties::default();
                let props = $crate::sml_properties!(props => $($body)*);
                props.children
            })?
        }
    }};

}

#[macro_export]
macro_rules! sml {
    ((
        $name:ident
        $($body:tt)*
    )) => {{
        let name = stringify!($name);

        let props = $crate::dom::Properties::default();
        let props = $crate::sml_properties!(props => $($body)*);

        $crate::dom::Object::create(name, props)
    }};

    ((&
        $component:ident
        $($body:tt)*
    )) => {{
        let component = $crate::sml_cc_properties!($component => $($body)*);
        component.render()
    }};
}

// TODO: special syntax for components => maybe (& Thing)
// TODO: special syntax for list of components => maybe (* ...)

pub trait Component<S> {
    fn render(self) -> dom::Object<S>;
}

#[cfg(test)]
mod tests {
    use crate::dom::{Attribute, Object};
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

        struct Empty;
        impl Component<()> for Empty {
            fn render(self) -> Object<()> {
                sml! { (tag) }
            }
        }

        struct Custom {
            id: String,
        }
        impl Component<()> for Custom {
            fn render(self) -> Object<()> {
                sml! { (tag) }
            }
        }

        struct Parent {
            id: String,
            children: Vec<Object<()>>,
        }
        impl Component<()> for Parent {
            fn render(self) -> Object<()> {
                sml! { (tag { self.children }) }
            }
        }

        struct ParentNoAttrs {
            children: Vec<Object<()>>,
        }
        impl Component<()> for ParentNoAttrs {
            fn render(self) -> Object<()> {
                sml! { (tag { self.children }) }
            }
        }

        dbg(sml! {
            (tag (@ (id "tag"))
                (nested { "Text node" })

                (& Empty)

                (& Custom (@ (id "custom")))

                (& Parent (@ (id "parent")) (child))
                (& Parent (@ (id "custom"))
                    (foo)
                    (bar (@ (id "tag") (class "some-class")))
                )

                (& ParentNoAttrs
                    (foo)
                    (bar)
                )
                (& ParentNoAttrs { "content" })

                (end)
            )
        });

        assert_eq!(1, 1);
    }
}
