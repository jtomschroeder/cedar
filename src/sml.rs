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
    ) => {
        $object $( .attr( $crate::sml_attr!($attr_name $attr_value) ) )*
    };

    ((
        $name:ident
        // $((@ $(( $attr_name:ident $attr_value:expr ))+ ))?

        $($body:tt)*

        // $( $(( $child:ident $($tail:tt)* ))? $($value:block)? )*

    )) => {
//        $crate::dom::Object::new(stringify!($name))

        sml!(@inc $crate::dom::Object::new(stringify!($name)) => $($body)* )

//            $( $( .attr( $crate::sml_attr!($attr_name $attr_value) ) )* )?
//            $( .push( sml!(( $child $($tail)* )) ) )*
//            $( .push( $value )  )?
    };
}

#[cfg(test)]
mod tests {
    use crate::dom::Object;

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

//        dbg(sml! {
//            (tag (@ (id "tag"))
//                (nested { "Text node" })
//                (end)
//            )
//        });
//
//        dbg(sml! {
//            (tag (@ (id "tag"))
//                (nested { "Text node" })
//                {"hello"}
//            )
//        });
//
//        dbg(sml! {
//            (tag (@ (id "tag"))
//                (nested { "Text node" })
//                {"hello"}
//                (nested { "Text node" })
//                {"hello"}
//                (end)
//            )
//        });

        assert_eq!(1, 1);
    }
}
