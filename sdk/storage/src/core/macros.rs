/// create a "response" struct of a certain name, containing certain headers.
#[macro_export]
macro_rules! response_from_headers {
    ($cn:ident, $($fh:path => $na:ident: $typ:ty),+) => {
        use http::HeaderMap;

        #[derive(Debug, Clone, PartialEq)]
        pub struct $cn {
             $(pub $na: $typ),+,
        }

        impl $cn {
            pub(crate) fn from_headers(headers: &HeaderMap) -> Result<$cn, $crate::Error> {
               $(
                    let $na = $fh(headers)?;
                )+

                Ok($cn {
                    $($na,)+
                })
            }

        }
    };
}
