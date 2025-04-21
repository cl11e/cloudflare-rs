#[macro_export]
macro_rules! api_resp {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        paste! {
            #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
            #[serde(rename_all = "snake_case")]
            pub struct [<$name Response>] {
                pub results: [<$name Results>],
                pub errors: Option<Vec<crate::framework::response::ResponseInfo>>,
                pub messages: Option<Vec<String>>,
                pub success: Option<bool>,
                #[serde(flatten)]
                pub other: std::collections::HashMap<String, Option<serde_json::Value>>,
            }

            #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
            #[serde(rename_all = "snake_case")]
            pub struct [<$name Results>] {
                $(
                    pub $field: Option<$ty>,
                )*
            }
        }
    };
}
#[macro_export]
macro_rules! api_gen {
    ($st:ty) => {
        impl ApiResult for $st {}
    };
}
#[macro_export]
macro_rules! api_endpoint {
    // Pattern 1: With params (explicit method and params marker) and return type
    ($method:ident, $name:ident => $response:ty > $resp_type:ty, $path:expr; $($field:ident),+; params: $params:ty) => {
        paste! {
            #[derive(Debug)]
            pub struct $name<'a> {
                $(pub $field: &'a str,)+
                pub params: $params,
            }
            api_resp!($name {
                result: $resp_type,
            });
            api_gen!([<$name Response>]);
            impl<'a> EndpointSpec for $name<'a> {
                type JsonResponse = [<$name Response>];
                type ResponseType = ApiSuccess<Self::JsonResponse>;

                fn method(&self) -> Method {
                    Method::$method
                }

                fn path(&self) -> String {
                    format!($path, $(self.$field),+)
                }

                #[inline]
                fn body(&self) -> Option<RequestBody> {
                    Some(RequestBody::Json(
                        serde_json::to_string(&self.params)
                            .expect("Failed to serialize request body")
                    ))
                }
            }
        }
    };

    // Pattern 2: Without params (explicit method) and return type
    ($method:ident, $name:ident => $response:ty > $resp_type:ty , $path:expr; $($field:ident),+) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            $(pub $field: &'a str,)+
        }

        api_resp!($name {
            result: $resp_type,
        });

        impl<'a> EndpointSpec for $name<'a> {
            type JsonResponse = $response;
            type ResponseType = ApiSuccess<Self::JsonResponse>;

            fn method(&self) -> Method {
                Method::$method
            }

            fn path(&self) -> String {
                format!($path, $(self.$field),+)
            }

        }
    };

    // Pattern 3: With params (explicit method and params marker)
    ($method:ident, $name:ident => $response:ty , $path:expr; $($field:ident),+; params: $params:ty) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            $(pub $field: &'a str,)+
            pub params: $params,
        }

        impl<'a> EndpointSpec for $name<'a> {
            type JsonResponse = $response;
            type ResponseType = ApiSuccess<Self::JsonResponse>;

            fn method(&self) -> Method {
                Method::$method
            }

            fn path(&self) -> String {
                format!($path, $(self.$field),+)
            }

            #[inline]
            fn body(&self) -> Option<RequestBody> {
                Some(RequestBody::Json(
                    serde_json::to_string(&self.params)
                        .expect("Failed to serialize request body")
                ))
            }
        }
    };

    // Pattern 4: Without params (explicit method) and return type
    ($method:ident, $name:ident => $response:ty , $path:expr; $($field:ident),+) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            $(pub $field: &'a str,)+
        }

        impl<'a> EndpointSpec for $name<'a> {
            type JsonResponse = $response;
            type ResponseType = ApiSuccess<Self::JsonResponse>;

            fn method(&self) -> Method {
                Method::$method
            }

            fn path(&self) -> String {
                format!($path, $(self.$field),+)
            }

        }
    };

}
