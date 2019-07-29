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

// TODO: figure out `attrs` for custom component

#[macro_export]
macro_rules! sml_properties {

    ($properties:expr => ) => { $properties };

    (
        $properties:expr =>
        (@ $(( $attr_name:ident $attr_value:expr ))+ )
        $($body:tt)*
    ) => {{
        let mut props = $properties;
        $( props.attributes.push($crate::sml_attr!($attr_name $attr_value)); )*
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
        let mut props = $crate::dom::Properties::default();
        let mut props = $crate::sml_properties!( props => $($tail)* );

        let component = $component.render(props.attributes, props.children);

        $properties.children.push(component);

        $crate::sml_properties!($properties => $($body)*)
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
        let props = $crate::sml_properties!( props => $($body)* );

        $crate::dom::Object::create(name, props.attributes, props.children)
    }};
}

// TODO: special syntax for components => maybe (& Thing)
// TODO: special syntax for list of components => maybe (* ...)

pub trait Component<S> {
    fn render(
        &self,
        attrs: Vec<dom::Attribute<S>>,
        children: Vec<dom::Object<S>>,
    ) -> dom::Object<S>;
}

impl<S, F> Component<S> for F
where
    F: Fn(Vec<dom::Attribute<S>>, Vec<dom::Object<S>>) -> dom::Object<S>,
{
    fn render(
        &self,
        attrs: Vec<dom::Attribute<S>>,
        children: Vec<dom::Object<S>>,
    ) -> dom::Object<S> {
        self(attrs, children)
    }
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

        struct Custom;
        impl Component<()> for Custom {
            fn render(&self, attrs: Vec<Attribute<()>>, children: Vec<Object<()>>) -> Object<()> {
                sml! { (div { "Hello" }) }
            }
        }

        dbg(sml! {
            (tag (@ (id "tag"))
                (nested { "Text node" })

                (& Custom)
                (& Custom (@ (id "custom")))
                (& Custom (@ (id "custom")) (child))
                (& Custom (@ (id "custom"))
                    (foo)
                    (bar (@ (id "tag") (class "some-class")))
                )
                (& Custom
                    (foo)
                    (bar)
                )
                // (& Custom { "content" })

                (end)
            )
        });

        assert_eq!(1, 1);
    }
}
