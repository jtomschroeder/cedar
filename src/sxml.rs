#[macro_export]
macro_rules! sml {
//             $( $( sml!(#attr ($attr_name $attr_value)) )* )?
//    (#attr ($name:ident $value:expr)) => {
//        .attr(stringify!($attr_name), $attr_value)
//    };

    ((
        $name:ident
        $((@ $(( $attr_name:ident $attr_value:expr ))+ ))?
        $(( $child:ident $($tail:tt)* ))*
        $( $value:block )?
    )) => {
        $crate::dom::Object::new(stringify!($name))
            $( $( .attr( stringify!($attr_name), $attr_value ) )* )?
            $( .push( sml!(( $child $($tail)* )) ) )*
            $( .value( $value )  )?
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

        assert_eq!(1, 1);
    }
}
