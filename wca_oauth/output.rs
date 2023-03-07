#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod wcif {
    use serde::{Deserialize, Serialize};
    mod activity {
        use serde::{Deserialize, Serialize};
        use super::DateTime;
        #[serde(rename_all = "camelCase")]
        pub struct Activity {
            pub id: usize,
            pub name: String,
            pub activity_code: String,
            #[serde(
                deserialize_with = "crate::de_date_time",
                serialize_with = "crate::ser_date_time"
            )]
            pub start_time: DateTime,
            #[serde(
                deserialize_with = "crate::de_date_time",
                serialize_with = "crate::ser_date_time"
            )]
            pub end_time: DateTime,
            pub child_activities: Vec<Activity>,
            pub scramble_set_id: Option<usize>,
            pub extensions: Vec<serde_json::Value>,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Activity {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Activity {
            #[inline]
            fn eq(&self, other: &Activity) -> bool {
                self.id == other.id && self.name == other.name
                    && self.activity_code == other.activity_code
                    && self.start_time == other.start_time
                    && self.end_time == other.end_time
                    && self.child_activities == other.child_activities
                    && self.scramble_set_id == other.scramble_set_id
                    && self.extensions == other.extensions
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Activity {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "name",
                    "activity_code",
                    "start_time",
                    "end_time",
                    "child_activities",
                    "scramble_set_id",
                    "extensions",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.name,
                    &self.activity_code,
                    &self.start_time,
                    &self.end_time,
                    &self.child_activities,
                    &self.scramble_set_id,
                    &&self.extensions,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Activity",
                    names,
                    values,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Activity {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "activityCode" => _serde::__private::Ok(__Field::__field2),
                                "startTime" => _serde::__private::Ok(__Field::__field3),
                                "endTime" => _serde::__private::Ok(__Field::__field4),
                                "childActivities" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                "scrambleSetId" => _serde::__private::Ok(__Field::__field6),
                                "extensions" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"activityCode" => _serde::__private::Ok(__Field::__field2),
                                b"startTime" => _serde::__private::Ok(__Field::__field3),
                                b"endTime" => _serde::__private::Ok(__Field::__field4),
                                b"childActivities" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                b"scrambleSetId" => _serde::__private::Ok(__Field::__field6),
                                b"extensions" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Activity>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Activity;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Activity",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match {
                                struct __DeserializeWith<'de> {
                                    value: DateTime,
                                    phantom: _serde::__private::PhantomData<Activity>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::Deserialize<'de>
                                for __DeserializeWith<'de> {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::__private::Ok(__DeserializeWith {
                                            value: match crate::de_date_time(__deserializer) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                            phantom: _serde::__private::PhantomData,
                                            lifetime: _serde::__private::PhantomData,
                                        })
                                    }
                                }
                                _serde::__private::Option::map(
                                    match _serde::de::SeqAccess::next_element::<
                                        __DeserializeWith<'de>,
                                    >(&mut __seq) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                    |__wrap| __wrap.value,
                                )
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match {
                                struct __DeserializeWith<'de> {
                                    value: DateTime,
                                    phantom: _serde::__private::PhantomData<Activity>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::Deserialize<'de>
                                for __DeserializeWith<'de> {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::__private::Ok(__DeserializeWith {
                                            value: match crate::de_date_time(__deserializer) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                            phantom: _serde::__private::PhantomData,
                                            lifetime: _serde::__private::PhantomData,
                                        })
                                    }
                                }
                                _serde::__private::Option::map(
                                    match _serde::de::SeqAccess::next_element::<
                                        __DeserializeWith<'de>,
                                    >(&mut __seq) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                    |__wrap| __wrap.value,
                                )
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Activity>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<usize>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Vec<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Activity with 8 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Activity {
                                id: __field0,
                                name: __field1,
                                activity_code: __field2,
                                start_time: __field3,
                                end_time: __field4,
                                child_activities: __field5,
                                scramble_set_id: __field6,
                                extensions: __field7,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<DateTime> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<DateTime> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Vec<Activity>> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<Option<usize>> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<
                                Vec<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "activityCode",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "startTime",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some({
                                            struct __DeserializeWith<'de> {
                                                value: DateTime,
                                                phantom: _serde::__private::PhantomData<Activity>,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private::Ok(__DeserializeWith {
                                                        value: match crate::de_date_time(__deserializer) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                        phantom: _serde::__private::PhantomData,
                                                        lifetime: _serde::__private::PhantomData,
                                                    })
                                                }
                                            }
                                            match _serde::de::MapAccess::next_value::<
                                                __DeserializeWith<'de>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__wrapper) => __wrapper.value,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        });
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "endTime",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some({
                                            struct __DeserializeWith<'de> {
                                                value: DateTime,
                                                phantom: _serde::__private::PhantomData<Activity>,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private::Ok(__DeserializeWith {
                                                        value: match crate::de_date_time(__deserializer) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                        phantom: _serde::__private::PhantomData,
                                                        lifetime: _serde::__private::PhantomData,
                                                    })
                                                }
                                            }
                                            match _serde::de::MapAccess::next_value::<
                                                __DeserializeWith<'de>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__wrapper) => __wrapper.value,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        });
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "childActivities",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Activity>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "scrambleSetId",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<usize>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "extensions",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("activityCode") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::missing_field(
                                            "startTime",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("endTime"),
                                    );
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "childActivities",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "scrambleSetId",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("extensions") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Activity {
                                id: __field0,
                                name: __field1,
                                activity_code: __field2,
                                start_time: __field3,
                                end_time: __field4,
                                child_activities: __field5,
                                scramble_set_id: __field6,
                                extensions: __field7,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "activityCode",
                        "startTime",
                        "endTime",
                        "childActivities",
                        "scrambleSetId",
                        "extensions",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Activity",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Activity>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Activity {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Activity",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "activityCode",
                        &self.activity_code,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "startTime",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a DateTime,),
                                phantom: _serde::__private::PhantomData<Activity>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    crate::ser_date_time(self.values.0, __s)
                                }
                            }
                            &__SerializeWith {
                                values: (&self.start_time,),
                                phantom: _serde::__private::PhantomData::<Activity>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "endTime",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a DateTime,),
                                phantom: _serde::__private::PhantomData<Activity>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    crate::ser_date_time(self.values.0, __s)
                                }
                            }
                            &__SerializeWith {
                                values: (&self.end_time,),
                                phantom: _serde::__private::PhantomData::<Activity>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "childActivities",
                        &self.child_activities,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "scrambleSetId",
                        &self.scramble_set_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "extensions",
                        &self.extensions,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Activity {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Activity {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<usize>;
                let _: ::core::cmp::AssertParamIsEq<String>;
                let _: ::core::cmp::AssertParamIsEq<DateTime>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Activity>>;
                let _: ::core::cmp::AssertParamIsEq<Option<usize>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<serde_json::Value>>;
            }
        }
        impl Activity {
            pub fn overlaps(&self, other: &Self) -> bool {
                self.start_time < other.end_time && other.start_time < self.end_time
                    && self != other
            }
            pub fn overlaps_any<'a>(
                &self,
                other: impl IntoIterator<Item = &'a Self> + 'a,
            ) -> bool {
                other.into_iter().any(|other| self.overlaps(other))
            }
        }
    }
    mod advancement_condition {
        use serde::{Deserialize, Serialize};
        #[serde(tag = "type", content = "level", rename_all = "camelCase")]
        pub enum AdvancementCondition {
            Percent(usize),
            Ranking(usize),
            AttemptResult(usize),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for AdvancementCondition {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 3",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "percent" => _serde::__private::Ok(__Field::__field0),
                                "ranking" => _serde::__private::Ok(__Field::__field1),
                                "attemptResult" => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"percent" => _serde::__private::Ok(__Field::__field0),
                                b"ranking" => _serde::__private::Ok(__Field::__field1),
                                b"attemptResult" => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    const VARIANTS: &'static [&'static str] = &[
                        "percent",
                        "ranking",
                        "attemptResult",
                    ];
                    struct __Seed<'de> {
                        field: __Field,
                        marker: _serde::__private::PhantomData<AdvancementCondition>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                        type Value = AdvancementCondition;
                        fn deserialize<__D>(
                            self,
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self::Value, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            match self.field {
                                __Field::__field0 => {
                                    _serde::__private::Result::map(
                                        <usize as _serde::Deserialize>::deserialize(__deserializer),
                                        AdvancementCondition::Percent,
                                    )
                                }
                                __Field::__field1 => {
                                    _serde::__private::Result::map(
                                        <usize as _serde::Deserialize>::deserialize(__deserializer),
                                        AdvancementCondition::Ranking,
                                    )
                                }
                                __Field::__field2 => {
                                    _serde::__private::Result::map(
                                        <usize as _serde::Deserialize>::deserialize(__deserializer),
                                        AdvancementCondition::AttemptResult,
                                    )
                                }
                            }
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<AdvancementCondition>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = AdvancementCondition;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "adjacently tagged enum AdvancementCondition",
                            )
                        }
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            match {
                                let mut __rk: _serde::__private::Option<
                                    _serde::__private::de::TagOrContentField,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__k)
                                    = match _serde::de::MapAccess::next_key_seed(
                                        &mut __map,
                                        _serde::__private::de::TagContentOtherFieldVisitor {
                                            tag: "type",
                                            content: "level",
                                        },
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    match __k {
                                        _serde::__private::de::TagContentOtherField::Other => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                            continue;
                                        }
                                        _serde::__private::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private::Some(
                                                _serde::__private::de::TagOrContentField::Content,
                                            );
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Tag,
                                ) => {
                                    let __field = match _serde::de::MapAccess::next_value(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k)
                                            = match _serde::de::MapAccess::next_key_seed(
                                                &mut __map,
                                                _serde::__private::de::TagContentOtherFieldVisitor {
                                                    tag: "type",
                                                    content: "level",
                                                },
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => {
                                            _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            )
                                        }
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => {
                                            let __ret = match _serde::de::MapAccess::next_value_seed(
                                                &mut __map,
                                                __Seed {
                                                    field: __field,
                                                    marker: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                            match {
                                                let mut __rk: _serde::__private::Option<
                                                    _serde::__private::de::TagOrContentField,
                                                > = _serde::__private::None;
                                                while let _serde::__private::Some(__k)
                                                    = match _serde::de::MapAccess::next_key_seed(
                                                        &mut __map,
                                                        _serde::__private::de::TagContentOtherFieldVisitor {
                                                            tag: "type",
                                                            content: "level",
                                                        },
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    } {
                                                    match __k {
                                                        _serde::__private::de::TagContentOtherField::Other => {
                                                            let _ = match _serde::de::MapAccess::next_value::<
                                                                _serde::de::IgnoredAny,
                                                            >(&mut __map) {
                                                                _serde::__private::Ok(__val) => __val,
                                                                _serde::__private::Err(__err) => {
                                                                    return _serde::__private::Err(__err);
                                                                }
                                                            };
                                                            continue;
                                                        }
                                                        _serde::__private::de::TagContentOtherField::Tag => {
                                                            __rk = _serde::__private::Some(
                                                                _serde::__private::de::TagOrContentField::Tag,
                                                            );
                                                            break;
                                                        }
                                                        _serde::__private::de::TagContentOtherField::Content => {
                                                            __rk = _serde::__private::Some(
                                                                _serde::__private::de::TagOrContentField::Content,
                                                            );
                                                            break;
                                                        }
                                                    }
                                                }
                                                __rk
                                            } {
                                                _serde::__private::Some(
                                                    _serde::__private::de::TagOrContentField::Tag,
                                                ) => {
                                                    _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                                    )
                                                }
                                                _serde::__private::Some(
                                                    _serde::__private::de::TagOrContentField::Content,
                                                ) => {
                                                    _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("level"),
                                                    )
                                                }
                                                _serde::__private::None => _serde::__private::Ok(__ret),
                                            }
                                        }
                                        _serde::__private::None => {
                                            match __field {
                                                __Field::__field0 => {
                                                    _serde::__private::de::missing_field("level")
                                                        .map(AdvancementCondition::Percent)
                                                }
                                                __Field::__field1 => {
                                                    _serde::__private::de::missing_field("level")
                                                        .map(AdvancementCondition::Ranking)
                                                }
                                                __Field::__field2 => {
                                                    _serde::__private::de::missing_field("level")
                                                        .map(AdvancementCondition::AttemptResult)
                                                }
                                            }
                                        }
                                    }
                                }
                                _serde::__private::Some(
                                    _serde::__private::de::TagOrContentField::Content,
                                ) => {
                                    let __content = match _serde::de::MapAccess::next_value::<
                                        _serde::__private::de::Content,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    match {
                                        let mut __rk: _serde::__private::Option<
                                            _serde::__private::de::TagOrContentField,
                                        > = _serde::__private::None;
                                        while let _serde::__private::Some(__k)
                                            = match _serde::de::MapAccess::next_key_seed(
                                                &mut __map,
                                                _serde::__private::de::TagContentOtherFieldVisitor {
                                                    tag: "type",
                                                    content: "level",
                                                },
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __k {
                                                _serde::__private::de::TagContentOtherField::Other => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                    continue;
                                                }
                                                _serde::__private::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private::Some(
                                                        _serde::__private::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Tag,
                                        ) => {
                                            let __deserializer = _serde::__private::de::ContentDeserializer::<
                                                __A::Error,
                                            >::new(__content);
                                            let __ret = match match match _serde::de::MapAccess::next_value(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                __Field::__field0 => {
                                                    _serde::__private::Result::map(
                                                        <usize as _serde::Deserialize>::deserialize(__deserializer),
                                                        AdvancementCondition::Percent,
                                                    )
                                                }
                                                __Field::__field1 => {
                                                    _serde::__private::Result::map(
                                                        <usize as _serde::Deserialize>::deserialize(__deserializer),
                                                        AdvancementCondition::Ranking,
                                                    )
                                                }
                                                __Field::__field2 => {
                                                    _serde::__private::Result::map(
                                                        <usize as _serde::Deserialize>::deserialize(__deserializer),
                                                        AdvancementCondition::AttemptResult,
                                                    )
                                                }
                                            } {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                            match {
                                                let mut __rk: _serde::__private::Option<
                                                    _serde::__private::de::TagOrContentField,
                                                > = _serde::__private::None;
                                                while let _serde::__private::Some(__k)
                                                    = match _serde::de::MapAccess::next_key_seed(
                                                        &mut __map,
                                                        _serde::__private::de::TagContentOtherFieldVisitor {
                                                            tag: "type",
                                                            content: "level",
                                                        },
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    } {
                                                    match __k {
                                                        _serde::__private::de::TagContentOtherField::Other => {
                                                            let _ = match _serde::de::MapAccess::next_value::<
                                                                _serde::de::IgnoredAny,
                                                            >(&mut __map) {
                                                                _serde::__private::Ok(__val) => __val,
                                                                _serde::__private::Err(__err) => {
                                                                    return _serde::__private::Err(__err);
                                                                }
                                                            };
                                                            continue;
                                                        }
                                                        _serde::__private::de::TagContentOtherField::Tag => {
                                                            __rk = _serde::__private::Some(
                                                                _serde::__private::de::TagOrContentField::Tag,
                                                            );
                                                            break;
                                                        }
                                                        _serde::__private::de::TagContentOtherField::Content => {
                                                            __rk = _serde::__private::Some(
                                                                _serde::__private::de::TagOrContentField::Content,
                                                            );
                                                            break;
                                                        }
                                                    }
                                                }
                                                __rk
                                            } {
                                                _serde::__private::Some(
                                                    _serde::__private::de::TagOrContentField::Tag,
                                                ) => {
                                                    _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                                    )
                                                }
                                                _serde::__private::Some(
                                                    _serde::__private::de::TagOrContentField::Content,
                                                ) => {
                                                    _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("level"),
                                                    )
                                                }
                                                _serde::__private::None => _serde::__private::Ok(__ret),
                                            }
                                        }
                                        _serde::__private::Some(
                                            _serde::__private::de::TagOrContentField::Content,
                                        ) => {
                                            _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("level"),
                                            )
                                        }
                                        _serde::__private::None => {
                                            _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::missing_field("type"),
                                            )
                                        }
                                    }
                                }
                                _serde::__private::None => {
                                    _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("type"),
                                    )
                                }
                            }
                        }
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            match match _serde::de::SeqAccess::next_element(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__field) => {
                                    match match _serde::de::SeqAccess::next_element_seed(
                                        &mut __seq,
                                        __Seed {
                                            field: __field,
                                            marker: _serde::__private::PhantomData,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__ret) => {
                                            _serde::__private::Ok(__ret)
                                        }
                                        _serde::__private::None => {
                                            _serde::__private::Err(
                                                _serde::de::Error::invalid_length(1, &self),
                                            )
                                        }
                                    }
                                }
                                _serde::__private::None => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_length(0, &self),
                                    )
                                }
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["type", "level"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AdvancementCondition",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                AdvancementCondition,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for AdvancementCondition {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        AdvancementCondition::Percent(ref __field0) => {
                            let mut __struct = match _serde::Serializer::serialize_struct(
                                __serializer,
                                "AdvancementCondition",
                                2,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeStruct::serialize_field(
                                &mut __struct,
                                "type",
                                "percent",
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeStruct::serialize_field(
                                &mut __struct,
                                "level",
                                __field0,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::ser::SerializeStruct::end(__struct)
                        }
                        AdvancementCondition::Ranking(ref __field0) => {
                            let mut __struct = match _serde::Serializer::serialize_struct(
                                __serializer,
                                "AdvancementCondition",
                                2,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeStruct::serialize_field(
                                &mut __struct,
                                "type",
                                "ranking",
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeStruct::serialize_field(
                                &mut __struct,
                                "level",
                                __field0,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::ser::SerializeStruct::end(__struct)
                        }
                        AdvancementCondition::AttemptResult(ref __field0) => {
                            let mut __struct = match _serde::Serializer::serialize_struct(
                                __serializer,
                                "AdvancementCondition",
                                2,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeStruct::serialize_field(
                                &mut __struct,
                                "type",
                                "attemptResult",
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeStruct::serialize_field(
                                &mut __struct,
                                "level",
                                __field0,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::ser::SerializeStruct::end(__struct)
                        }
                    }
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for AdvancementCondition {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    AdvancementCondition::Percent(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Percent",
                            &__self_0,
                        )
                    }
                    AdvancementCondition::Ranking(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ranking",
                            &__self_0,
                        )
                    }
                    AdvancementCondition::AttemptResult(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "AttemptResult",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AdvancementCondition {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AdvancementCondition {
            #[inline]
            fn eq(&self, other: &AdvancementCondition) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            AdvancementCondition::Percent(__self_0),
                            AdvancementCondition::Percent(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            AdvancementCondition::Ranking(__self_0),
                            AdvancementCondition::Ranking(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            AdvancementCondition::AttemptResult(__self_0),
                            AdvancementCondition::AttemptResult(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
    }
    mod assignment {
        use serde::{Deserialize, Serialize};
        #[serde(rename_all = "camelCase")]
        pub struct Assignment {
            pub activity_id: usize,
            pub assignment_code: AssignmentCode,
            pub station_number: Option<usize>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Assignment {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "activityId" => _serde::__private::Ok(__Field::__field0),
                                "assignmentCode" => _serde::__private::Ok(__Field::__field1),
                                "stationNumber" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"activityId" => _serde::__private::Ok(__Field::__field0),
                                b"assignmentCode" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"stationNumber" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Assignment>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Assignment;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Assignment",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Assignment with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                AssignmentCode,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Assignment with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<usize>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Assignment with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Assignment {
                                activity_id: __field0,
                                assignment_code: __field1,
                                station_number: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                AssignmentCode,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Option<usize>> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "activityId",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "assignmentCode",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                AssignmentCode,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "stationNumber",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<usize>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("activityId") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "assignmentCode",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "stationNumber",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Assignment {
                                activity_id: __field0,
                                assignment_code: __field1,
                                station_number: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "activityId",
                        "assignmentCode",
                        "stationNumber",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Assignment",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Assignment>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Assignment {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Assignment",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "activityId",
                        &self.activity_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "assignmentCode",
                        &self.assignment_code,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "stationNumber",
                        &self.station_number,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Assignment {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Assignment",
                    "activity_id",
                    &self.activity_id,
                    "assignment_code",
                    &self.assignment_code,
                    "station_number",
                    &&self.station_number,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Assignment {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Assignment {
            #[inline]
            fn eq(&self, other: &Assignment) -> bool {
                self.activity_id == other.activity_id
                    && self.assignment_code == other.assignment_code
                    && self.station_number == other.station_number
            }
        }
        use serde::Deserializer;
        use serde::de::Visitor;
        pub enum AssignmentCode {
            Competitor,
            Judge,
            Scrambler,
            Runner,
            DataEntry,
            Announcer,
            Other(String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AssignmentCode {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    AssignmentCode::Competitor => {
                        ::core::fmt::Formatter::write_str(f, "Competitor")
                    }
                    AssignmentCode::Judge => {
                        ::core::fmt::Formatter::write_str(f, "Judge")
                    }
                    AssignmentCode::Scrambler => {
                        ::core::fmt::Formatter::write_str(f, "Scrambler")
                    }
                    AssignmentCode::Runner => {
                        ::core::fmt::Formatter::write_str(f, "Runner")
                    }
                    AssignmentCode::DataEntry => {
                        ::core::fmt::Formatter::write_str(f, "DataEntry")
                    }
                    AssignmentCode::Announcer => {
                        ::core::fmt::Formatter::write_str(f, "Announcer")
                    }
                    AssignmentCode::Other(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Other",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AssignmentCode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AssignmentCode {
            #[inline]
            fn eq(&self, other: &AssignmentCode) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            AssignmentCode::Other(__self_0),
                            AssignmentCode::Other(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => true,
                    }
            }
        }
        impl<'de> Deserialize<'de> for AssignmentCode {
            fn deserialize<D>(deserializer: D) -> Result<AssignmentCode, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(AssignmentCodeVisitor)
            }
        }
        struct AssignmentCodeVisitor;
        impl<'de> Visitor<'de> for AssignmentCodeVisitor {
            type Value = AssignmentCode;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a role enum variant")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(
                    match v {
                        "competitor" => AssignmentCode::Competitor,
                        "staff-judge" => AssignmentCode::Judge,
                        "staff-scrambler" => AssignmentCode::Scrambler,
                        "staff-runner" => AssignmentCode::Runner,
                        "staff-dataentry" => AssignmentCode::DataEntry,
                        "staff-announcer" => AssignmentCode::Announcer,
                        v => AssignmentCode::Other(v.to_string()),
                    },
                )
            }
        }
        impl Serialize for AssignmentCode {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer
                    .serialize_str(
                        match self {
                            AssignmentCode::Competitor => "competitor",
                            AssignmentCode::Judge => "staff-judge",
                            AssignmentCode::Scrambler => "staff-scrambler",
                            AssignmentCode::Runner => "staff-runner",
                            AssignmentCode::DataEntry => "staff-dataentry",
                            AssignmentCode::Announcer => "staff-announcer",
                            AssignmentCode::Other(v) => &v,
                        },
                    )
            }
        }
    }
    mod attempt_result {
        use serde::Deserialize;
        use serde::Serialize;
        use serde::Deserializer;
        use serde::de::Visitor;
        pub enum AttemptResult {
            DNF,
            DNS,
            Skip,
            Ok(usize),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AttemptResult {
            #[inline]
            fn clone(&self) -> AttemptResult {
                match self {
                    AttemptResult::DNF => AttemptResult::DNF,
                    AttemptResult::DNS => AttemptResult::DNS,
                    AttemptResult::Skip => AttemptResult::Skip,
                    AttemptResult::Ok(__self_0) => {
                        AttemptResult::Ok(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AttemptResult {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    AttemptResult::DNF => ::core::fmt::Formatter::write_str(f, "DNF"),
                    AttemptResult::DNS => ::core::fmt::Formatter::write_str(f, "DNS"),
                    AttemptResult::Skip => ::core::fmt::Formatter::write_str(f, "Skip"),
                    AttemptResult::Ok(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ok",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AttemptResult {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AttemptResult {
            #[inline]
            fn eq(&self, other: &AttemptResult) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (AttemptResult::Ok(__self_0), AttemptResult::Ok(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        impl<'de> Deserialize<'de> for AttemptResult {
            fn deserialize<D>(deserializer: D) -> Result<AttemptResult, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_i64(AttemptResultVisitor)
            }
        }
        struct AttemptResultVisitor;
        impl<'de> Visitor<'de> for AttemptResultVisitor {
            type Value = AttemptResult;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("an attemptresult")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(
                    match v {
                        -1 => AttemptResult::DNF,
                        -2 => AttemptResult::DNS,
                        0 => AttemptResult::Skip,
                        _ => AttemptResult::Ok(v as usize),
                    },
                )
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(
                    match v {
                        0 => AttemptResult::Skip,
                        _ => AttemptResult::Ok(v as usize),
                    },
                )
            }
        }
        impl Serialize for AttemptResult {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    AttemptResult::Ok(v) => return v.serialize(serializer),
                    _ => {}
                }
                serializer
                    .serialize_i64(
                        match self {
                            AttemptResult::DNF => -1,
                            AttemptResult::DNS => -2,
                            AttemptResult::Skip => 0,
                            AttemptResult::Ok(v) => *v as i64,
                        },
                    )
            }
        }
    }
    mod avatar {
        use serde::{Deserialize, Serialize};
        #[serde(rename_all = "camelCase")]
        pub struct Avatar {
            pub url: String,
            pub thumb_url: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Avatar {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "url" => _serde::__private::Ok(__Field::__field0),
                                "thumbUrl" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"url" => _serde::__private::Ok(__Field::__field0),
                                b"thumbUrl" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Avatar>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Avatar;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Avatar",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Avatar with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Avatar with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Avatar {
                                url: __field0,
                                thumb_url: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "thumbUrl",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("url") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("thumbUrl") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Avatar {
                                url: __field0,
                                thumb_url: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["url", "thumbUrl"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Avatar",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Avatar>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Avatar {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Avatar",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "thumbUrl",
                        &self.thumb_url,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Avatar {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Avatar",
                    "url",
                    &self.url,
                    "thumb_url",
                    &&self.thumb_url,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Avatar {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Avatar {
            #[inline]
            fn eq(&self, other: &Avatar) -> bool {
                self.url == other.url && self.thumb_url == other.thumb_url
            }
        }
    }
    mod cutoff {
        use serde::{Deserialize, Serialize};
        use crate::AttemptResult;
        #[serde(rename_all = "camelCase")]
        pub struct Cutoff {
            pub number_of_attempts: usize,
            pub attempt_result: AttemptResult,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Cutoff {
            #[inline]
            fn clone(&self) -> Cutoff {
                Cutoff {
                    number_of_attempts: ::core::clone::Clone::clone(
                        &self.number_of_attempts,
                    ),
                    attempt_result: ::core::clone::Clone::clone(&self.attempt_result),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Cutoff {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "numberOfAttempts" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "attemptResult" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"numberOfAttempts" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"attemptResult" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Cutoff>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Cutoff;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Cutoff",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Cutoff with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                AttemptResult,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Cutoff with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Cutoff {
                                number_of_attempts: __field0,
                                attempt_result: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<AttemptResult> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "numberOfAttempts",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "attemptResult",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                AttemptResult,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "numberOfAttempts",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "attemptResult",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Cutoff {
                                number_of_attempts: __field0,
                                attempt_result: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "numberOfAttempts",
                        "attemptResult",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Cutoff",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Cutoff>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Cutoff {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Cutoff",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "numberOfAttempts",
                        &self.number_of_attempts,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "attemptResult",
                        &self.attempt_result,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Cutoff {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Cutoff",
                    "number_of_attempts",
                    &self.number_of_attempts,
                    "attempt_result",
                    &&self.attempt_result,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Cutoff {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Cutoff {
            #[inline]
            fn eq(&self, other: &Cutoff) -> bool {
                self.number_of_attempts == other.number_of_attempts
                    && self.attempt_result == other.attempt_result
            }
        }
    }
    mod event {
        use serde::{Deserialize, Serialize};
        use super::*;
        pub struct Event {
            pub id: String,
            pub rounds: Vec<Round>,
            pub competitor_limit: Option<usize>,
            pub qualification: Option<serde_json::Value>,
            pub extensions: Vec<serde_json::Value>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Event {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "rounds" => _serde::__private::Ok(__Field::__field1),
                                "competitor_limit" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                "qualification" => _serde::__private::Ok(__Field::__field3),
                                "extensions" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"rounds" => _serde::__private::Ok(__Field::__field1),
                                b"competitor_limit" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                b"qualification" => _serde::__private::Ok(__Field::__field3),
                                b"extensions" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Event>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Event;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Event",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Event with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Round>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Event with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<usize>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Event with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Event with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Vec<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Event with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Event {
                                id: __field0,
                                rounds: __field1,
                                competitor_limit: __field2,
                                qualification: __field3,
                                extensions: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Vec<Round>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Option<usize>> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Vec<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("rounds"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Round>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "competitor_limit",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<usize>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "qualification",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "extensions",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("rounds") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "competitor_limit",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "qualification",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("extensions") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Event {
                                id: __field0,
                                rounds: __field1,
                                competitor_limit: __field2,
                                qualification: __field3,
                                extensions: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "rounds",
                        "competitor_limit",
                        "qualification",
                        "extensions",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Event",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Event>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Event {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Event",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "rounds",
                        &self.rounds,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "competitor_limit",
                        &self.competitor_limit,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "qualification",
                        &self.qualification,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "extensions",
                        &self.extensions,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Event {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Event",
                    "id",
                    &self.id,
                    "rounds",
                    &self.rounds,
                    "competitor_limit",
                    &self.competitor_limit,
                    "qualification",
                    &self.qualification,
                    "extensions",
                    &&self.extensions,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Event {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Event {
            #[inline]
            fn eq(&self, other: &Event) -> bool {
                self.id == other.id && self.rounds == other.rounds
                    && self.competitor_limit == other.competitor_limit
                    && self.qualification == other.qualification
                    && self.extensions == other.extensions
            }
        }
    }
    mod person {
        use serde::{Deserialize, Serialize};
        use super::*;
        #[serde(rename_all = "camelCase")]
        pub struct Person {
            pub registrant_id: Option<usize>,
            pub name: String,
            pub wca_user_id: usize,
            pub wca_id: Option<WcaId>,
            pub country_iso_2: String,
            pub gender: char,
            pub birthdate: serde_with::chrono::NaiveDate,
            pub email: String,
            pub avatar: Option<Avatar>,
            pub roles: Vec<Role>,
            pub registration: Option<Registration>,
            pub assignments: Vec<Assignment>,
            pub personal_bests: Vec<PersonalBest>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Person {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "registrantId" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "wcaUserId" => _serde::__private::Ok(__Field::__field2),
                                "wcaId" => _serde::__private::Ok(__Field::__field3),
                                "countryIso2" => _serde::__private::Ok(__Field::__field4),
                                "gender" => _serde::__private::Ok(__Field::__field5),
                                "birthdate" => _serde::__private::Ok(__Field::__field6),
                                "email" => _serde::__private::Ok(__Field::__field7),
                                "avatar" => _serde::__private::Ok(__Field::__field8),
                                "roles" => _serde::__private::Ok(__Field::__field9),
                                "registration" => _serde::__private::Ok(__Field::__field10),
                                "assignments" => _serde::__private::Ok(__Field::__field11),
                                "personalBests" => _serde::__private::Ok(__Field::__field12),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"registrantId" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"wcaUserId" => _serde::__private::Ok(__Field::__field2),
                                b"wcaId" => _serde::__private::Ok(__Field::__field3),
                                b"countryIso2" => _serde::__private::Ok(__Field::__field4),
                                b"gender" => _serde::__private::Ok(__Field::__field5),
                                b"birthdate" => _serde::__private::Ok(__Field::__field6),
                                b"email" => _serde::__private::Ok(__Field::__field7),
                                b"avatar" => _serde::__private::Ok(__Field::__field8),
                                b"roles" => _serde::__private::Ok(__Field::__field9),
                                b"registration" => _serde::__private::Ok(__Field::__field10),
                                b"assignments" => _serde::__private::Ok(__Field::__field11),
                                b"personalBests" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Person>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Person;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Person",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<usize>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<WcaId>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                char,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                serde_with::chrono::NaiveDate,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                Option<Avatar>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Role>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match match _serde::de::SeqAccess::next_element::<
                                Option<Registration>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Assignment>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match match _serde::de::SeqAccess::next_element::<
                                Vec<PersonalBest>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct Person with 13 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Person {
                                registrant_id: __field0,
                                name: __field1,
                                wca_user_id: __field2,
                                wca_id: __field3,
                                country_iso_2: __field4,
                                gender: __field5,
                                birthdate: __field6,
                                email: __field7,
                                avatar: __field8,
                                roles: __field9,
                                registration: __field10,
                                assignments: __field11,
                                personal_bests: __field12,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<usize>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Option<WcaId>> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<char> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<
                                serde_with::chrono::NaiveDate,
                            > = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<
                                Option<Avatar>,
                            > = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<Vec<Role>> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<
                                Option<Registration>,
                            > = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<
                                Vec<Assignment>,
                            > = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<
                                Vec<PersonalBest>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "registrantId",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<usize>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "wcaUserId",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("wcaId"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<WcaId>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "countryIso2",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("gender"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                char,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "birthdate",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                serde_with::chrono::NaiveDate,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("email"),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("avatar"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Avatar>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("roles"),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Role>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "registration",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Registration>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "assignments",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Assignment>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "personalBests",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<PersonalBest>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("registrantId") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("wcaUserId") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("wcaId") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("countryIso2") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("gender") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("birthdate") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("email") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("avatar") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("roles") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("registration") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("assignments") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "personalBests",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Person {
                                registrant_id: __field0,
                                name: __field1,
                                wca_user_id: __field2,
                                wca_id: __field3,
                                country_iso_2: __field4,
                                gender: __field5,
                                birthdate: __field6,
                                email: __field7,
                                avatar: __field8,
                                roles: __field9,
                                registration: __field10,
                                assignments: __field11,
                                personal_bests: __field12,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "registrantId",
                        "name",
                        "wcaUserId",
                        "wcaId",
                        "countryIso2",
                        "gender",
                        "birthdate",
                        "email",
                        "avatar",
                        "roles",
                        "registration",
                        "assignments",
                        "personalBests",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Person",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Person>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Person {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Person",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "registrantId",
                        &self.registrant_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "wcaUserId",
                        &self.wca_user_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "wcaId",
                        &self.wca_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "countryIso2",
                        &self.country_iso_2,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gender",
                        &self.gender,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "birthdate",
                        &self.birthdate,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "email",
                        &self.email,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "avatar",
                        &self.avatar,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "roles",
                        &self.roles,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "registration",
                        &self.registration,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "assignments",
                        &self.assignments,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "personalBests",
                        &self.personal_bests,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Person {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "registrant_id",
                    "name",
                    "wca_user_id",
                    "wca_id",
                    "country_iso_2",
                    "gender",
                    "birthdate",
                    "email",
                    "avatar",
                    "roles",
                    "registration",
                    "assignments",
                    "personal_bests",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.registrant_id,
                    &self.name,
                    &self.wca_user_id,
                    &self.wca_id,
                    &self.country_iso_2,
                    &self.gender,
                    &self.birthdate,
                    &self.email,
                    &self.avatar,
                    &self.roles,
                    &self.registration,
                    &self.assignments,
                    &&self.personal_bests,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Person",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Person {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Person {
            #[inline]
            fn eq(&self, other: &Person) -> bool {
                self.registrant_id == other.registrant_id && self.name == other.name
                    && self.wca_user_id == other.wca_user_id
                    && self.wca_id == other.wca_id
                    && self.country_iso_2 == other.country_iso_2
                    && self.gender == other.gender && self.birthdate == other.birthdate
                    && self.email == other.email && self.avatar == other.avatar
                    && self.roles == other.roles
                    && self.registration == other.registration
                    && self.assignments == other.assignments
                    && self.personal_bests == other.personal_bests
            }
        }
    }
    mod personal_best {
        use serde::{Deserialize, Serialize};
        use crate::AttemptResult;
        #[serde(rename_all = "camelCase")]
        pub struct PersonalBest {
            pub event_id: String,
            pub best: AttemptResult,
            #[serde(rename = "type")]
            pub t: String,
            pub world_ranking: usize,
            pub continental_ranking: usize,
            pub national_ranking: usize,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PersonalBest {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "eventId" => _serde::__private::Ok(__Field::__field0),
                                "best" => _serde::__private::Ok(__Field::__field1),
                                "type" => _serde::__private::Ok(__Field::__field2),
                                "worldRanking" => _serde::__private::Ok(__Field::__field3),
                                "continentalRanking" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                "nationalRanking" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"eventId" => _serde::__private::Ok(__Field::__field0),
                                b"best" => _serde::__private::Ok(__Field::__field1),
                                b"type" => _serde::__private::Ok(__Field::__field2),
                                b"worldRanking" => _serde::__private::Ok(__Field::__field3),
                                b"continentalRanking" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                b"nationalRanking" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<PersonalBest>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PersonalBest;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct PersonalBest",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct PersonalBest with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                AttemptResult,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct PersonalBest with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct PersonalBest with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct PersonalBest with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct PersonalBest with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct PersonalBest with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(PersonalBest {
                                event_id: __field0,
                                best: __field1,
                                t: __field2,
                                world_ranking: __field3,
                                continental_ranking: __field4,
                                national_ranking: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<AttemptResult> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<usize> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "eventId",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("best"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                AttemptResult,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "worldRanking",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "continentalRanking",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "nationalRanking",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("eventId") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("best") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("type") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("worldRanking") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "continentalRanking",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "nationalRanking",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(PersonalBest {
                                event_id: __field0,
                                best: __field1,
                                t: __field2,
                                world_ranking: __field3,
                                continental_ranking: __field4,
                                national_ranking: __field5,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "eventId",
                        "best",
                        "type",
                        "worldRanking",
                        "continentalRanking",
                        "nationalRanking",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "PersonalBest",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<PersonalBest>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for PersonalBest {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "PersonalBest",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "eventId",
                        &self.event_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "best",
                        &self.best,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "type",
                        &self.t,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "worldRanking",
                        &self.world_ranking,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "continentalRanking",
                        &self.continental_ranking,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "nationalRanking",
                        &self.national_ranking,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for PersonalBest {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "event_id",
                    "best",
                    "t",
                    "world_ranking",
                    "continental_ranking",
                    "national_ranking",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.event_id,
                    &self.best,
                    &self.t,
                    &self.world_ranking,
                    &self.continental_ranking,
                    &&self.national_ranking,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "PersonalBest",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PersonalBest {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PersonalBest {
            #[inline]
            fn eq(&self, other: &PersonalBest) -> bool {
                self.event_id == other.event_id && self.best == other.best
                    && self.t == other.t && self.world_ranking == other.world_ranking
                    && self.continental_ranking == other.continental_ranking
                    && self.national_ranking == other.national_ranking
            }
        }
    }
    mod registration {
        use serde::{Deserialize, Serialize};
        #[serde(rename_all = "camelCase")]
        pub struct Registration {
            pub wca_registration_id: usize,
            pub event_ids: Vec<String>,
            pub status: String,
            pub guests: usize,
            pub comments: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Registration {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "wcaRegistrationId" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "eventIds" => _serde::__private::Ok(__Field::__field1),
                                "status" => _serde::__private::Ok(__Field::__field2),
                                "guests" => _serde::__private::Ok(__Field::__field3),
                                "comments" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"wcaRegistrationId" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"eventIds" => _serde::__private::Ok(__Field::__field1),
                                b"status" => _serde::__private::Ok(__Field::__field2),
                                b"guests" => _serde::__private::Ok(__Field::__field3),
                                b"comments" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Registration>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Registration;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Registration",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Registration with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Vec<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Registration with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Registration with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Registration with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Registration with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Registration {
                                wca_registration_id: __field0,
                                event_ids: __field1,
                                status: __field2,
                                guests: __field3,
                                comments: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "wcaRegistrationId",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "eventIds",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("guests"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "comments",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "wcaRegistrationId",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("eventIds") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("status") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("guests") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("comments") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Registration {
                                wca_registration_id: __field0,
                                event_ids: __field1,
                                status: __field2,
                                guests: __field3,
                                comments: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "wcaRegistrationId",
                        "eventIds",
                        "status",
                        "guests",
                        "comments",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Registration",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Registration>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Registration {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Registration",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "wcaRegistrationId",
                        &self.wca_registration_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "eventIds",
                        &self.event_ids,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "status",
                        &self.status,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "guests",
                        &self.guests,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "comments",
                        &self.comments,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Registration {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Registration",
                    "wca_registration_id",
                    &self.wca_registration_id,
                    "event_ids",
                    &self.event_ids,
                    "status",
                    &self.status,
                    "guests",
                    &self.guests,
                    "comments",
                    &&self.comments,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Registration {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Registration {
            #[inline]
            fn eq(&self, other: &Registration) -> bool {
                self.wca_registration_id == other.wca_registration_id
                    && self.event_ids == other.event_ids && self.status == other.status
                    && self.guests == other.guests && self.comments == other.comments
            }
        }
    }
    mod result {
        use serde::{Deserialize, Serialize};
        use crate::AttemptResult;
        #[serde(rename_all = "camelCase")]
        pub struct Result {
            pub person_id: usize,
            pub ranking: Option<usize>,
            pub attempts: Vec<Attempt>,
            pub best: AttemptResult,
            pub average: AttemptResult,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Result {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "personId" => _serde::__private::Ok(__Field::__field0),
                                "ranking" => _serde::__private::Ok(__Field::__field1),
                                "attempts" => _serde::__private::Ok(__Field::__field2),
                                "best" => _serde::__private::Ok(__Field::__field3),
                                "average" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"personId" => _serde::__private::Ok(__Field::__field0),
                                b"ranking" => _serde::__private::Ok(__Field::__field1),
                                b"attempts" => _serde::__private::Ok(__Field::__field2),
                                b"best" => _serde::__private::Ok(__Field::__field3),
                                b"average" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Result>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Result;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Result",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Result with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<usize>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Result with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Attempt>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Result with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                AttemptResult,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Result with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                AttemptResult,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Result with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Result {
                                person_id: __field0,
                                ranking: __field1,
                                attempts: __field2,
                                best: __field3,
                                average: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Option<usize>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Vec<Attempt>> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<AttemptResult> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<AttemptResult> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "personId",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "ranking",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<usize>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "attempts",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Attempt>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("best"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                AttemptResult,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "average",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                AttemptResult,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("personId") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("ranking") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("attempts") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("best") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("average") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Result {
                                person_id: __field0,
                                ranking: __field1,
                                attempts: __field2,
                                best: __field3,
                                average: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "personId",
                        "ranking",
                        "attempts",
                        "best",
                        "average",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Result",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Result>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Result {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Result",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "personId",
                        &self.person_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "ranking",
                        &self.ranking,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "attempts",
                        &self.attempts,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "best",
                        &self.best,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "average",
                        &self.average,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Result {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Result",
                    "person_id",
                    &self.person_id,
                    "ranking",
                    &self.ranking,
                    "attempts",
                    &self.attempts,
                    "best",
                    &self.best,
                    "average",
                    &&self.average,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Result {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Result {
            #[inline]
            fn eq(&self, other: &Result) -> bool {
                self.person_id == other.person_id && self.ranking == other.ranking
                    && self.attempts == other.attempts && self.best == other.best
                    && self.average == other.average
            }
        }
        #[serde(rename_all = "camelCase")]
        pub struct Attempt {
            pub result: AttemptResult,
            pub reconstruction: Option<serde_json::Value>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Attempt {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "result" => _serde::__private::Ok(__Field::__field0),
                                "reconstruction" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"result" => _serde::__private::Ok(__Field::__field0),
                                b"reconstruction" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Attempt>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Attempt;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Attempt",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                AttemptResult,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Attempt with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Attempt with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Attempt {
                                result: __field0,
                                reconstruction: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<AttemptResult> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("result"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                AttemptResult,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "reconstruction",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("result") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "reconstruction",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Attempt {
                                result: __field0,
                                reconstruction: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "result",
                        "reconstruction",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Attempt",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Attempt>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Attempt {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Attempt",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "result",
                        &self.result,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "reconstruction",
                        &self.reconstruction,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Attempt {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Attempt",
                    "result",
                    &self.result,
                    "reconstruction",
                    &&self.reconstruction,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Attempt {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Attempt {
            #[inline]
            fn eq(&self, other: &Attempt) -> bool {
                self.result == other.result
                    && self.reconstruction == other.reconstruction
            }
        }
    }
    mod role {
        use serde::Deserialize;
        use serde::Serialize;
        use serde::Deserializer;
        use serde::de::Visitor;
        pub enum Role {
            Delegate,
            TraineeDelegate,
            Organizer,
            Other(String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Role {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Role::Delegate => ::core::fmt::Formatter::write_str(f, "Delegate"),
                    Role::TraineeDelegate => {
                        ::core::fmt::Formatter::write_str(f, "TraineeDelegate")
                    }
                    Role::Organizer => ::core::fmt::Formatter::write_str(f, "Organizer"),
                    Role::Other(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Other",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Role {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Role {
            #[inline]
            fn eq(&self, other: &Role) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Role::Other(__self_0), Role::Other(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Role {
            #[inline]
            fn clone(&self) -> Role {
                match self {
                    Role::Delegate => Role::Delegate,
                    Role::TraineeDelegate => Role::TraineeDelegate,
                    Role::Organizer => Role::Organizer,
                    Role::Other(__self_0) => {
                        Role::Other(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        impl<'de> Deserialize<'de> for Role {
            fn deserialize<D>(deserializer: D) -> Result<Role, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(RoleVisitor)
            }
        }
        struct RoleVisitor;
        impl<'de> Visitor<'de> for RoleVisitor {
            type Value = Role;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a role enum variant")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(
                    match v {
                        "delegate" => Role::Delegate,
                        "trainee-delegate" => Role::TraineeDelegate,
                        "organizer" => Role::Organizer,
                        v => Role::Other(v.to_string()),
                    },
                )
            }
        }
        impl Serialize for Role {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer
                    .serialize_str(
                        match self {
                            Role::Delegate => "delegate",
                            Role::TraineeDelegate => "trainee-delegate",
                            Role::Organizer => "organizer",
                            Role::Other(v) => v,
                        },
                    )
            }
        }
    }
    mod room {
        use serde::{Deserialize, Serialize};
        use super::*;
        #[serde(rename_all = "camelCase")]
        pub struct Room {
            pub id: usize,
            pub name: String,
            pub color: String,
            pub activities: Vec<Activity>,
            pub extensions: Vec<serde_json::Value>,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Room {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Room {
            #[inline]
            fn eq(&self, other: &Room) -> bool {
                self.id == other.id && self.name == other.name
                    && self.color == other.color && self.activities == other.activities
                    && self.extensions == other.extensions
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Room {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Room",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "color",
                    &self.color,
                    "activities",
                    &self.activities,
                    "extensions",
                    &&self.extensions,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Room {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "color" => _serde::__private::Ok(__Field::__field2),
                                "activities" => _serde::__private::Ok(__Field::__field3),
                                "extensions" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"color" => _serde::__private::Ok(__Field::__field2),
                                b"activities" => _serde::__private::Ok(__Field::__field3),
                                b"extensions" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Room>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Room;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Room",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Room with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Room with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Room with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Activity>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Room with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Vec<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Room with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Room {
                                id: __field0,
                                name: __field1,
                                color: __field2,
                                activities: __field3,
                                extensions: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Vec<Activity>> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Vec<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("color"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "activities",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Activity>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "extensions",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("color") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("activities") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("extensions") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Room {
                                id: __field0,
                                name: __field1,
                                color: __field2,
                                activities: __field3,
                                extensions: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "color",
                        "activities",
                        "extensions",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Room",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Room>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Room {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Room",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "color",
                        &self.color,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "activities",
                        &self.activities,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "extensions",
                        &self.extensions,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    }
    mod round {
        use serde::{Deserialize, Serialize};
        use super::*;
        #[serde(rename_all = "camelCase")]
        pub struct Round {
            pub id: String,
            pub format: char,
            pub time_limit: Option<TimeLimit>,
            pub cutoff: Option<Cutoff>,
            pub advancement_condition: Option<AdvancementCondition>,
            pub results: Vec<Result>,
            pub scramble_set_count: usize,
            pub extensions: Vec<serde_json::Value>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Round {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "format" => _serde::__private::Ok(__Field::__field1),
                                "timeLimit" => _serde::__private::Ok(__Field::__field2),
                                "cutoff" => _serde::__private::Ok(__Field::__field3),
                                "advancementCondition" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                "results" => _serde::__private::Ok(__Field::__field5),
                                "scrambleSetCount" => {
                                    _serde::__private::Ok(__Field::__field6)
                                }
                                "extensions" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"format" => _serde::__private::Ok(__Field::__field1),
                                b"timeLimit" => _serde::__private::Ok(__Field::__field2),
                                b"cutoff" => _serde::__private::Ok(__Field::__field3),
                                b"advancementCondition" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                b"results" => _serde::__private::Ok(__Field::__field5),
                                b"scrambleSetCount" => {
                                    _serde::__private::Ok(__Field::__field6)
                                }
                                b"extensions" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Round>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Round;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Round",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                char,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<TimeLimit>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<Cutoff>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<AdvancementCondition>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Result>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Vec<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Round with 8 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Round {
                                id: __field0,
                                format: __field1,
                                time_limit: __field2,
                                cutoff: __field3,
                                advancement_condition: __field4,
                                results: __field5,
                                scramble_set_count: __field6,
                                extensions: __field7,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<char> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<TimeLimit>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<Cutoff>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<AdvancementCondition>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Vec<Result>> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<
                                Vec<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("format"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                char,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "timeLimit",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<TimeLimit>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("cutoff"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Cutoff>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "advancementCondition",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<AdvancementCondition>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "results",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Result>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "scrambleSetCount",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "extensions",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("format") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("timeLimit") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("cutoff") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "advancementCondition",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("results") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "scrambleSetCount",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("extensions") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Round {
                                id: __field0,
                                format: __field1,
                                time_limit: __field2,
                                cutoff: __field3,
                                advancement_condition: __field4,
                                results: __field5,
                                scramble_set_count: __field6,
                                extensions: __field7,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "format",
                        "timeLimit",
                        "cutoff",
                        "advancementCondition",
                        "results",
                        "scrambleSetCount",
                        "extensions",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Round",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Round>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Round {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Round",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "format",
                        &self.format,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "timeLimit",
                        &self.time_limit,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "cutoff",
                        &self.cutoff,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "advancementCondition",
                        &self.advancement_condition,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "results",
                        &self.results,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "scrambleSetCount",
                        &self.scramble_set_count,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "extensions",
                        &self.extensions,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Round {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "format",
                    "time_limit",
                    "cutoff",
                    "advancement_condition",
                    "results",
                    "scramble_set_count",
                    "extensions",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.format,
                    &self.time_limit,
                    &self.cutoff,
                    &self.advancement_condition,
                    &self.results,
                    &self.scramble_set_count,
                    &&self.extensions,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Round",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Round {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Round {
            #[inline]
            fn eq(&self, other: &Round) -> bool {
                self.id == other.id && self.format == other.format
                    && self.time_limit == other.time_limit && self.cutoff == other.cutoff
                    && self.advancement_condition == other.advancement_condition
                    && self.results == other.results
                    && self.scramble_set_count == other.scramble_set_count
                    && self.extensions == other.extensions
            }
        }
    }
    mod schedule {
        use serde::{Deserialize, Serialize};
        use super::*;
        #[serde(rename_all = "camelCase")]
        pub struct Schedule {
            pub start_date: Date,
            pub number_of_days: usize,
            pub venues: Vec<Venue>,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Schedule {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Schedule {
            #[inline]
            fn eq(&self, other: &Schedule) -> bool {
                self.start_date == other.start_date
                    && self.number_of_days == other.number_of_days
                    && self.venues == other.venues
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Schedule {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Schedule",
                    "start_date",
                    &self.start_date,
                    "number_of_days",
                    &self.number_of_days,
                    "venues",
                    &&self.venues,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Schedule {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "startDate" => _serde::__private::Ok(__Field::__field0),
                                "numberOfDays" => _serde::__private::Ok(__Field::__field1),
                                "venues" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"startDate" => _serde::__private::Ok(__Field::__field0),
                                b"numberOfDays" => _serde::__private::Ok(__Field::__field1),
                                b"venues" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Schedule>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Schedule;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Schedule",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Date,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Schedule with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Schedule with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Venue>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Schedule with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Schedule {
                                start_date: __field0,
                                number_of_days: __field1,
                                venues: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Date> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Vec<Venue>> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "startDate",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Date,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "numberOfDays",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("venues"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Venue>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("startDate") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("numberOfDays") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("venues") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Schedule {
                                start_date: __field0,
                                number_of_days: __field1,
                                venues: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "startDate",
                        "numberOfDays",
                        "venues",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Schedule",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Schedule>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Schedule {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Schedule",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "startDate",
                        &self.start_date,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "numberOfDays",
                        &self.number_of_days,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "venues",
                        &self.venues,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    }
    mod time_limit {
        use serde::{Deserialize, Serialize};
        #[serde(rename_all = "camelCase")]
        pub struct TimeLimit {
            pub centiseconds: usize,
            pub cumulative_round_ids: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TimeLimit {
            #[inline]
            fn clone(&self) -> TimeLimit {
                TimeLimit {
                    centiseconds: ::core::clone::Clone::clone(&self.centiseconds),
                    cumulative_round_ids: ::core::clone::Clone::clone(
                        &self.cumulative_round_ids,
                    ),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TimeLimit {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "centiseconds" => _serde::__private::Ok(__Field::__field0),
                                "cumulativeRoundIds" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"centiseconds" => _serde::__private::Ok(__Field::__field0),
                                b"cumulativeRoundIds" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<TimeLimit>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TimeLimit;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct TimeLimit",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TimeLimit with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Vec<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TimeLimit with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(TimeLimit {
                                centiseconds: __field0,
                                cumulative_round_ids: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "centiseconds",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "cumulativeRoundIds",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("centiseconds") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "cumulativeRoundIds",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(TimeLimit {
                                centiseconds: __field0,
                                cumulative_round_ids: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "centiseconds",
                        "cumulativeRoundIds",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TimeLimit",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<TimeLimit>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for TimeLimit {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "TimeLimit",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "centiseconds",
                        &self.centiseconds,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "cumulativeRoundIds",
                        &self.cumulative_round_ids,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for TimeLimit {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TimeLimit",
                    "centiseconds",
                    &self.centiseconds,
                    "cumulative_round_ids",
                    &&self.cumulative_round_ids,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TimeLimit {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TimeLimit {
            #[inline]
            fn eq(&self, other: &TimeLimit) -> bool {
                self.centiseconds == other.centiseconds
                    && self.cumulative_round_ids == other.cumulative_round_ids
            }
        }
    }
    mod venue {
        use serde::{Deserialize, Serialize};
        use super::*;
        #[serde(rename_all = "camelCase")]
        pub struct Venue {
            pub id: usize,
            pub name: String,
            pub latitude_microdegrees: i64,
            pub longitude_microdegrees: i64,
            pub country_iso_2: String,
            pub timezone: String,
            pub rooms: Vec<Room>,
            pub extensions: Vec<serde_json::Value>,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Venue {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Venue {
            #[inline]
            fn eq(&self, other: &Venue) -> bool {
                self.id == other.id && self.name == other.name
                    && self.latitude_microdegrees == other.latitude_microdegrees
                    && self.longitude_microdegrees == other.longitude_microdegrees
                    && self.country_iso_2 == other.country_iso_2
                    && self.timezone == other.timezone && self.rooms == other.rooms
                    && self.extensions == other.extensions
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Venue {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "name",
                    "latitude_microdegrees",
                    "longitude_microdegrees",
                    "country_iso_2",
                    "timezone",
                    "rooms",
                    "extensions",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.name,
                    &self.latitude_microdegrees,
                    &self.longitude_microdegrees,
                    &self.country_iso_2,
                    &self.timezone,
                    &self.rooms,
                    &&self.extensions,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Venue",
                    names,
                    values,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Venue {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "latitudeMicrodegrees" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                "longitudeMicrodegrees" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                "countryIso2" => _serde::__private::Ok(__Field::__field4),
                                "timezone" => _serde::__private::Ok(__Field::__field5),
                                "rooms" => _serde::__private::Ok(__Field::__field6),
                                "extensions" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"latitudeMicrodegrees" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                b"longitudeMicrodegrees" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                b"countryIso2" => _serde::__private::Ok(__Field::__field4),
                                b"timezone" => _serde::__private::Ok(__Field::__field5),
                                b"rooms" => _serde::__private::Ok(__Field::__field6),
                                b"extensions" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Venue>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Venue;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Venue",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Room>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Vec<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Venue with 8 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Venue {
                                id: __field0,
                                name: __field1,
                                latitude_microdegrees: __field2,
                                longitude_microdegrees: __field3,
                                country_iso_2: __field4,
                                timezone: __field5,
                                rooms: __field6,
                                extensions: __field7,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<Vec<Room>> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<
                                Vec<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                usize,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "latitudeMicrodegrees",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "longitudeMicrodegrees",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "countryIso2",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "timezone",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("rooms"),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<Room>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "extensions",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Vec<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "latitudeMicrodegrees",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "longitudeMicrodegrees",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("countryIso2") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("timezone") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("rooms") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("extensions") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Venue {
                                id: __field0,
                                name: __field1,
                                latitude_microdegrees: __field2,
                                longitude_microdegrees: __field3,
                                country_iso_2: __field4,
                                timezone: __field5,
                                rooms: __field6,
                                extensions: __field7,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "latitudeMicrodegrees",
                        "longitudeMicrodegrees",
                        "countryIso2",
                        "timezone",
                        "rooms",
                        "extensions",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Venue",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Venue>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Venue {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Venue",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "latitudeMicrodegrees",
                        &self.latitude_microdegrees,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "longitudeMicrodegrees",
                        &self.longitude_microdegrees,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "countryIso2",
                        &self.country_iso_2,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "timezone",
                        &self.timezone,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "rooms",
                        &self.rooms,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "extensions",
                        &self.extensions,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    }
    mod wca_id {
        use serde::Deserialize;
        use serde::Deserializer;
        use serde::Serialize;
        use serde::de::Visitor;
        pub struct WcaId {
            pub year: u16,
            pub chars: [u8; 4],
            pub id: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for WcaId {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "WcaId",
                    "year",
                    &self.year,
                    "chars",
                    &self.chars,
                    "id",
                    &&self.id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WcaId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WcaId {
            #[inline]
            fn eq(&self, other: &WcaId) -> bool {
                self.year == other.year && self.chars == other.chars
                    && self.id == other.id
            }
        }
        impl<'de> Deserialize<'de> for WcaId {
            fn deserialize<D>(deserializer: D) -> Result<WcaId, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(WcaIdVisitor)
            }
        }
        struct WcaIdVisitor;
        impl<'de> Visitor<'de> for WcaIdVisitor {
            type Value = WcaId;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "a string consisting of 4 integers followed by 4 charachters follwed by 2 integers",
                    )
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() != 10 {
                    return Err(
                        E::custom({
                            let res = ::alloc::fmt::format(
                                format_args!("WcaId too short or too long"),
                            );
                            res
                        }),
                    );
                }
                let year = &v[0..4];
                let chars = &v[4..8];
                let id = &v[8..10];
                let year = u16::from_str_radix(year, 10)
                    .map_err(|_| E::custom({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The first four characters of a WcaId is not numerical"
                            ),
                        );
                        res
                    }))?;
                let id = u8::from_str_radix(id, 10)
                    .map_err(|_| E::custom({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The last two characters of a WcaId is not numerical"
                            ),
                        );
                        res
                    }))?;
                Ok(WcaId {
                    year,
                    chars: chars.as_bytes().try_into().unwrap(),
                    id,
                })
            }
        }
        impl Serialize for WcaId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let str = {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0:04}{1}{2:02}", self.year, self.chars.iter().map(| u | * u
                            as char).collect::< String > (), self.id
                        ),
                    );
                    res
                };
                serializer.serialize_str(&str)
            }
        }
    }
    pub use activity::*;
    pub use advancement_condition::*;
    pub use assignment::*;
    pub use attempt_result::*;
    pub use avatar::*;
    pub use cutoff::*;
    pub use event::*;
    pub use person::*;
    pub use personal_best::*;
    pub use registration::*;
    pub use result::*;
    pub use role::*;
    pub use room::*;
    pub use round::*;
    pub use schedule::*;
    pub use time_limit::*;
    pub use venue::*;
    pub use wca_id::*;
    pub use super::{Date, DateTime};
    use crate::WcifContainer;
    pub type WcifResult = std::result::Result<WcifContainer, WcifError>;
    #[serde(rename_all = "camelCase")]
    pub struct Wcif {
        pub format_version: String,
        pub id: String,
        pub name: String,
        pub short_name: String,
        pub persons: Vec<Person>,
        pub events: Vec<Event>,
        pub schedule: Schedule,
        pub competitor_limit: Option<usize>,
        pub extensions: Vec<serde_json::Value>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Wcif {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "formatVersion" => _serde::__private::Ok(__Field::__field0),
                            "id" => _serde::__private::Ok(__Field::__field1),
                            "name" => _serde::__private::Ok(__Field::__field2),
                            "shortName" => _serde::__private::Ok(__Field::__field3),
                            "persons" => _serde::__private::Ok(__Field::__field4),
                            "events" => _serde::__private::Ok(__Field::__field5),
                            "schedule" => _serde::__private::Ok(__Field::__field6),
                            "competitorLimit" => _serde::__private::Ok(__Field::__field7),
                            "extensions" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"formatVersion" => _serde::__private::Ok(__Field::__field0),
                            b"id" => _serde::__private::Ok(__Field::__field1),
                            b"name" => _serde::__private::Ok(__Field::__field2),
                            b"shortName" => _serde::__private::Ok(__Field::__field3),
                            b"persons" => _serde::__private::Ok(__Field::__field4),
                            b"events" => _serde::__private::Ok(__Field::__field5),
                            b"schedule" => _serde::__private::Ok(__Field::__field6),
                            b"competitorLimit" => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            b"extensions" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Wcif>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Wcif;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Wcif",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Person>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Event>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            Schedule,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            Option<usize>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match match _serde::de::SeqAccess::next_element::<
                            Vec<serde_json::Value>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct Wcif with 9 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Wcif {
                            format_version: __field0,
                            id: __field1,
                            name: __field2,
                            short_name: __field3,
                            persons: __field4,
                            events: __field5,
                            schedule: __field6,
                            competitor_limit: __field7,
                            extensions: __field8,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Vec<Person>> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Vec<Event>> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Schedule> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<Option<usize>> = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<
                            Vec<serde_json::Value>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "formatVersion",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "shortName",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "persons",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<Person>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("events"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<Event>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "schedule",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Schedule,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "competitorLimit",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<usize>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "extensions",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<serde_json::Value>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "formatVersion",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("shortName") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("persons") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("events") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("schedule") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "competitorLimit",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("extensions") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Wcif {
                            format_version: __field0,
                            id: __field1,
                            name: __field2,
                            short_name: __field3,
                            persons: __field4,
                            events: __field5,
                            schedule: __field6,
                            competitor_limit: __field7,
                            extensions: __field8,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "formatVersion",
                    "id",
                    "name",
                    "shortName",
                    "persons",
                    "events",
                    "schedule",
                    "competitorLimit",
                    "extensions",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Wcif",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Wcif>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Wcif {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Wcif",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "formatVersion",
                    &self.format_version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "shortName",
                    &self.short_name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "persons",
                    &self.persons,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "events",
                    &self.events,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "schedule",
                    &self.schedule,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "competitorLimit",
                    &self.competitor_limit,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "extensions",
                    &self.extensions,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Wcif {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "format_version",
                "id",
                "name",
                "short_name",
                "persons",
                "events",
                "schedule",
                "competitor_limit",
                "extensions",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.format_version,
                &self.id,
                &self.name,
                &self.short_name,
                &self.persons,
                &self.events,
                &self.schedule,
                &self.competitor_limit,
                &&self.extensions,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Wcif", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Wcif {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Wcif {
        #[inline]
        fn eq(&self, other: &Wcif) -> bool {
            self.format_version == other.format_version && self.id == other.id
                && self.name == other.name && self.short_name == other.short_name
                && self.persons == other.persons && self.events == other.events
                && self.schedule == other.schedule
                && self.competitor_limit == other.competitor_limit
                && self.extensions == other.extensions
        }
    }
    pub struct WcifError {
        pub error: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for WcifError {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "error" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"error" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WcifError>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WcifError;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct WcifError",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct WcifError with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(WcifError { error: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("error") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(WcifError { error: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["error"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "WcifError",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WcifError>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for WcifError {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "WcifError",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "error",
                    &self.error,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for WcifError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "WcifError",
                "error",
                &&self.error,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for WcifError {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for WcifError {
        #[inline]
        fn eq(&self, other: &WcifError) -> bool {
            self.error == other.error
        }
    }
    pub fn parse(json: String) -> WcifResult {
        serde_json::from_str(&json)
            .map(|wcif| WcifContainer::new(wcif))
            .map_err(|_| serde_json::from_str(&json).unwrap())
    }
}
mod oauth {
    use std::collections::HashMap;
    use reqwest::Client;
    use serde::Deserialize;
    use crate::*;
    use crate::Competition;
    struct AuthResponse {
        access_token: String,
        refresh_token: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AuthResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "access_token" => _serde::__private::Ok(__Field::__field0),
                            "refresh_token" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"access_token" => _serde::__private::Ok(__Field::__field0),
                            b"refresh_token" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AuthResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AuthResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct AuthResponse",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct AuthResponse with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct AuthResponse with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(AuthResponse {
                            access_token: __field0,
                            refresh_token: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "access_token",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "refresh_token",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("access_token") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "refresh_token",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(AuthResponse {
                            access_token: __field0,
                            refresh_token: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "access_token",
                    "refresh_token",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AuthResponse",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AuthResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct OAuth {
        access_token: String,
        refresh_token: String,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        client: Client,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OAuth {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "access_token",
                "refresh_token",
                "client_id",
                "client_secret",
                "redirect_uri",
                "client",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.access_token,
                &self.refresh_token,
                &self.client_id,
                &self.client_secret,
                &self.redirect_uri,
                &&self.client,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "OAuth", names, values)
        }
    }
    impl OAuth {
        pub async fn get_auth(
            client_id: String,
            client_secret: String,
            redirect_uri: String,
            auth_code: String,
        ) -> Self {
            let mut oauth = Self {
                access_token: {
                    let res = ::alloc::fmt::format(format_args!(""));
                    res
                },
                refresh_token: {
                    let res = ::alloc::fmt::format(format_args!(""));
                    res
                },
                client_id,
                client_secret,
                redirect_uri,
                client: reqwest::Client::new(),
            };
            oauth.get_auth_explicit_flow(auth_code).await;
            oauth
        }
        /// If you use this you need to get a token before hand. Refresh cannot be done with this type and will crash.
        pub async fn get_auth_implicit(
            client_id: String,
            access_token: String,
            redirect_uri: String,
        ) -> Self {
            Self {
                access_token,
                refresh_token: String::new(),
                client_id,
                client_secret: String::new(),
                redirect_uri,
                client: reqwest::Client::new(),
            }
        }
        async fn get_auth_explicit_flow(&mut self, code: String) {
            let mut params = HashMap::new();
            params.insert("grant_type", "authorization_code");
            params.insert("client_id", &self.client_id);
            params.insert("client_secret", &self.client_secret);
            params.insert("redirect_uri", &self.redirect_uri);
            params.insert("code", &code.trim());
            let response = self
                .client
                .post("https://www.worldcubeassociation.org/oauth/token")
                .form(&params)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let auth_response: AuthResponse = serde_json::from_str(&response).unwrap();
            self.access_token = auth_response.access_token;
            self.refresh_token = auth_response.refresh_token;
        }
        pub async fn refresh_auth(&mut self) {
            let mut params = HashMap::new();
            params.insert("grant_type", "refresh_token");
            params.insert("client_id", &self.client_id);
            params.insert("client_secret", &self.client_secret);
            params.insert("refresh_token", &self.refresh_token.trim());
            let response = self
                .client
                .post("https://www.worldcubeassociation.org/oauth/token")
                .form(&params)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let auth_response: AuthResponse = serde_json::from_str(&response).unwrap();
            self.access_token = auth_response.access_token;
            self.refresh_token = auth_response.refresh_token;
        }
        pub async fn get_wcif_api(self: &Self, id: &str) -> String {
            let get_url = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "https://www.worldcubeassociation.org/api/v0/competitions/{0}/wcif",
                        id
                    ),
                );
                res
            };
            let response = self
                .client
                .get(&get_url)
                .header(
                    "Authorization",
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Bearer {0}", self.access_token),
                        );
                        res
                    },
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            response
        }
        pub async fn get_wcif(&self, id: &str) -> WcifResult {
            let json = self.get_wcif_api(id).await;
            parse(json)
        }
        pub async fn get_competitions_managed_by_me(&self) -> Vec<Competition> {
            let url = "https://www.worldcubeassociation.org/api/v0/competitions?managed_by_me=true";
            let json = self
                .client
                .get(url)
                .header(
                    "Authorization",
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Bearer {0}", self.access_token),
                        );
                        res
                    },
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            Competition::from_json(&json)
        }
        async fn patch_wcif(&self, wcif: &Wcif, id: &str) -> String {
            let patch_url = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "https://www.worldcubeassociation.org/api/v0/competitions/{0}/wcif",
                        id
                    ),
                );
                res
            };
            let json = serde_json::to_string(wcif).unwrap();
            let response = self
                .client
                .patch(&patch_url)
                .header(
                    "Authorization",
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Bearer {0}", self.access_token),
                        );
                        res
                    },
                )
                .header("Content-Type", "application/json")
                .body(json)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            response
        }
    }
    pub struct WcifContainer {
        pub(crate) wcif: Wcif,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WcifContainer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "WcifContainer",
                "wcif",
                &&self.wcif,
            )
        }
    }
    impl WcifContainer {
        pub fn new(wcif: Wcif) -> WcifContainer {
            WcifContainer { wcif }
        }
        pub fn add_oauth(self, oauth: OAuth) -> WcifOAuth {
            WcifOAuth { cont: self, oauth }
        }
        pub fn get_mut<'a>(&'a mut self) -> &'a mut Wcif {
            &mut self.wcif
        }
        pub fn get<'a>(&'a self) -> &'a Wcif {
            &self.wcif
        }
        pub async fn patch(&self, oauth: &OAuth) -> String {
            oauth.patch_wcif(&self.wcif, &self.wcif.id).await
        }
        pub fn date(&self) -> Date {
            self.wcif.schedule.start_date
        }
        pub fn events_iter(&self) -> impl Iterator<Item = &Event> {
            self.wcif.events.iter()
        }
        pub fn events_iter_mut(&mut self) -> impl Iterator<Item = &mut Event> {
            self.wcif.events.iter_mut()
        }
        pub fn patch_events(&mut self, func: impl FnMut(&mut Event)) {
            self.events_iter_mut().for_each(func);
        }
        pub fn persons_iter(&self) -> impl Iterator<Item = &Person> {
            self.wcif.persons.iter()
        }
        pub fn persons_iter_mut(&mut self) -> impl Iterator<Item = &mut Person> {
            self.wcif.persons.iter_mut()
        }
        pub fn patch_persons(&mut self, func: impl FnMut(&mut Person)) {
            self.persons_iter_mut().for_each(func);
        }
        pub fn round_iter(&self) -> impl Iterator<Item = &Round> {
            self.events_iter().flat_map(|e| e.rounds.iter())
        }
        pub fn round_iter_mut(&mut self) -> impl Iterator<Item = &mut Round> {
            self.events_iter_mut().flat_map(|e| e.rounds.iter_mut())
        }
        pub fn patch_rounds(&mut self, func: impl FnMut(&mut Round)) {
            self.round_iter_mut().for_each(func);
        }
        pub fn activity_iter(&self) -> impl Iterator<Item = &Activity> {
            self.wcif
                .schedule
                .venues
                .iter()
                .flat_map(|venue| {
                    venue
                        .rooms
                        .iter()
                        .flat_map(|room| { ActivityIter::new(&room.activities) })
                })
        }
        pub fn schedule_activity_iter(&self) -> impl Iterator<Item = &Activity> {
            self.wcif
                .schedule
                .venues
                .iter()
                .flat_map(|venue| {
                    venue.rooms.iter().flat_map(|room| { room.activities.iter() })
                })
        }
        pub fn reg_ids_of_delegates(&self) -> Vec<usize> {
            self.persons_iter()
                .filter(|person| {
                    person.roles.contains(&Role::Delegate)
                        || person.roles.contains(&Role::TraineeDelegate)
                })
                .filter_map(|person| person.registrant_id)
                .collect()
        }
        pub fn overlapping_activities<'a>(
            &'a self,
        ) -> Vec<(&'a Activity, &'a Activity)> {
            self.schedule_activity_iter()
                .map(|act_1| {
                    self.schedule_activity_iter()
                        .filter(|act_2| act_1.overlaps(act_2))
                        .zip(std::iter::repeat(act_1))
                })
                .flatten()
                .collect()
        }
        pub fn add_groups_to_event(
            &mut self,
            event: &str,
            round: usize,
            no: usize,
        ) -> std::result::Result<&mut Vec<Activity>, ()> {
            let act = self
                .wcif
                .schedule
                .venues
                .iter_mut()
                .flat_map(|v| &mut v.rooms)
                .flat_map(|r| &mut r.activities)
                .find(|a| {
                    a
                        .activity_code
                        .contains(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}-r{1}", event, round),
                                );
                                res
                            },
                        )
                })
                .map(|a| {
                    if a.child_activities.len() != 0 {
                        return None;
                    }
                    a
                        .child_activities = (0..no)
                        .map(|g| {
                            let group_time = (a.end_time - a.start_time) / no as i32;
                            let start_time = a.start_time + (group_time * g as i32);
                            let end_time = a.start_time + (group_time * (g as i32 + 1));
                            Activity {
                                id: a.id * 1000 + g,
                                name: {
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}, Group {1}", a.name, g + 1),
                                    );
                                    res
                                },
                                activity_code: {
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}-g{1}", a.activity_code, g + 1),
                                    );
                                    res
                                },
                                start_time,
                                end_time,
                                child_activities: ::alloc::vec::Vec::new(),
                                scramble_set_id: None,
                                extensions: ::alloc::vec::Vec::new(),
                            }
                        })
                        .collect();
                    Some(a)
                });
            match act {
                Some(Some(v)) => Ok(&mut v.child_activities),
                _ => Err(()),
            }
        }
    }
    struct ActivityIter<'a> {
        activites: Vec<Box<dyn Iterator<Item = &'a Activity> + 'a>>,
    }
    impl<'a> ActivityIter<'a> {
        fn new(vec: &'a Vec<Activity>) -> Self {
            ActivityIter {
                activites: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Box::new(vec.iter())]),
                ),
            }
        }
    }
    impl<'a> Iterator for ActivityIter<'a> {
        type Item = &'a Activity;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(iter) = self.activites.last_mut() {
                match iter.next() {
                    None => {
                        self.activites.pop();
                        self.next()
                    }
                    Some(v) => {
                        self.activites.push(Box::new(v.child_activities.iter()));
                        Some(v)
                    }
                }
            } else {
                None
            }
        }
    }
}
mod competition {
    use serde::{Serialize, Deserialize};
    pub struct Competition {
        id: String,
        name: String,
        registration_open: String,
        registration_close: String,
        announced_at: Option<String>,
        start_date: String,
        end_date: String,
        competitor_limit: Option<u64>,
        cancelled_at: Option<String>,
        url: String,
        website: String,
        short_name: String,
        city: String,
        venue_address: String,
        venue_details: String,
        latitude_degrees: f64,
        longitude_degrees: f64,
        country_iso2: String,
        event_ids: Vec<String>,
        delegates: Vec<serde_json::Value>,
        organizers: Vec<serde_json::Value>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Competition {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Competition",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                        + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "registration_open",
                    &self.registration_open,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "registration_close",
                    &self.registration_close,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "announced_at",
                    &self.announced_at,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "start_date",
                    &self.start_date,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "end_date",
                    &self.end_date,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "competitor_limit",
                    &self.competitor_limit,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "cancelled_at",
                    &self.cancelled_at,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "url",
                    &self.url,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "website",
                    &self.website,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "short_name",
                    &self.short_name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "city",
                    &self.city,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "venue_address",
                    &self.venue_address,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "venue_details",
                    &self.venue_details,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "latitude_degrees",
                    &self.latitude_degrees,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "longitude_degrees",
                    &self.longitude_degrees,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "country_iso2",
                    &self.country_iso2,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "event_ids",
                    &self.event_ids,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "delegates",
                    &self.delegates,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "organizers",
                    &self.organizers,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Competition {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                    __field17,
                    __field18,
                    __field19,
                    __field20,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            15u64 => _serde::__private::Ok(__Field::__field15),
                            16u64 => _serde::__private::Ok(__Field::__field16),
                            17u64 => _serde::__private::Ok(__Field::__field17),
                            18u64 => _serde::__private::Ok(__Field::__field18),
                            19u64 => _serde::__private::Ok(__Field::__field19),
                            20u64 => _serde::__private::Ok(__Field::__field20),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "registration_open" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            "registration_close" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            "announced_at" => _serde::__private::Ok(__Field::__field4),
                            "start_date" => _serde::__private::Ok(__Field::__field5),
                            "end_date" => _serde::__private::Ok(__Field::__field6),
                            "competitor_limit" => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            "cancelled_at" => _serde::__private::Ok(__Field::__field8),
                            "url" => _serde::__private::Ok(__Field::__field9),
                            "website" => _serde::__private::Ok(__Field::__field10),
                            "short_name" => _serde::__private::Ok(__Field::__field11),
                            "city" => _serde::__private::Ok(__Field::__field12),
                            "venue_address" => _serde::__private::Ok(__Field::__field13),
                            "venue_details" => _serde::__private::Ok(__Field::__field14),
                            "latitude_degrees" => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            "longitude_degrees" => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            "country_iso2" => _serde::__private::Ok(__Field::__field17),
                            "event_ids" => _serde::__private::Ok(__Field::__field18),
                            "delegates" => _serde::__private::Ok(__Field::__field19),
                            "organizers" => _serde::__private::Ok(__Field::__field20),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"registration_open" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            b"registration_close" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"announced_at" => _serde::__private::Ok(__Field::__field4),
                            b"start_date" => _serde::__private::Ok(__Field::__field5),
                            b"end_date" => _serde::__private::Ok(__Field::__field6),
                            b"competitor_limit" => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            b"cancelled_at" => _serde::__private::Ok(__Field::__field8),
                            b"url" => _serde::__private::Ok(__Field::__field9),
                            b"website" => _serde::__private::Ok(__Field::__field10),
                            b"short_name" => _serde::__private::Ok(__Field::__field11),
                            b"city" => _serde::__private::Ok(__Field::__field12),
                            b"venue_address" => _serde::__private::Ok(__Field::__field13),
                            b"venue_details" => _serde::__private::Ok(__Field::__field14),
                            b"latitude_degrees" => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            b"longitude_degrees" => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            b"country_iso2" => _serde::__private::Ok(__Field::__field17),
                            b"event_ids" => _serde::__private::Ok(__Field::__field18),
                            b"delegates" => _serde::__private::Ok(__Field::__field19),
                            b"organizers" => _serde::__private::Ok(__Field::__field20),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Competition>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Competition;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Competition",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            Option<u64>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field9 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        9usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field10 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        10usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field11 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        11usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field12 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        12usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field13 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        13usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field14 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        14usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field15 = match match _serde::de::SeqAccess::next_element::<
                            f64,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        15usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field16 = match match _serde::de::SeqAccess::next_element::<
                            f64,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        16usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field17 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        17usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field18 = match match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        18usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field19 = match match _serde::de::SeqAccess::next_element::<
                            Vec<serde_json::Value>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        19usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        let __field20 = match match _serde::de::SeqAccess::next_element::<
                            Vec<serde_json::Value>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        20usize,
                                        &"struct Competition with 21 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Competition {
                            id: __field0,
                            name: __field1,
                            registration_open: __field2,
                            registration_close: __field3,
                            announced_at: __field4,
                            start_date: __field5,
                            end_date: __field6,
                            competitor_limit: __field7,
                            cancelled_at: __field8,
                            url: __field9,
                            website: __field10,
                            short_name: __field11,
                            city: __field12,
                            venue_address: __field13,
                            venue_details: __field14,
                            latitude_degrees: __field15,
                            longitude_degrees: __field16,
                            country_iso2: __field17,
                            event_ids: __field18,
                            delegates: __field19,
                            organizers: __field20,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<Option<u64>> = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field9: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field10: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field11: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field12: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field13: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field14: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field15: _serde::__private::Option<f64> = _serde::__private::None;
                        let mut __field16: _serde::__private::Option<f64> = _serde::__private::None;
                        let mut __field17: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field18: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field19: _serde::__private::Option<
                            Vec<serde_json::Value>,
                        > = _serde::__private::None;
                        let mut __field20: _serde::__private::Option<
                            Vec<serde_json::Value>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "registration_open",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "registration_close",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "announced_at",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "start_date",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "end_date",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "competitor_limit",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<u64>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "cancelled_at",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                        );
                                    }
                                    __field9 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::__private::Option::is_some(&__field10) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "website",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::__private::Option::is_some(&__field11) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "short_name",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field12 => {
                                    if _serde::__private::Option::is_some(&__field12) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("city"),
                                        );
                                    }
                                    __field12 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field13 => {
                                    if _serde::__private::Option::is_some(&__field13) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "venue_address",
                                            ),
                                        );
                                    }
                                    __field13 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field14 => {
                                    if _serde::__private::Option::is_some(&__field14) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "venue_details",
                                            ),
                                        );
                                    }
                                    __field14 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field15 => {
                                    if _serde::__private::Option::is_some(&__field15) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "latitude_degrees",
                                            ),
                                        );
                                    }
                                    __field15 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<f64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field16 => {
                                    if _serde::__private::Option::is_some(&__field16) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "longitude_degrees",
                                            ),
                                        );
                                    }
                                    __field16 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<f64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field17 => {
                                    if _serde::__private::Option::is_some(&__field17) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "country_iso2",
                                            ),
                                        );
                                    }
                                    __field17 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field18 => {
                                    if _serde::__private::Option::is_some(&__field18) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "event_ids",
                                            ),
                                        );
                                    }
                                    __field18 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field19 => {
                                    if _serde::__private::Option::is_some(&__field19) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "delegates",
                                            ),
                                        );
                                    }
                                    __field19 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<serde_json::Value>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field20 => {
                                    if _serde::__private::Option::is_some(&__field20) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "organizers",
                                            ),
                                        );
                                    }
                                    __field20 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<serde_json::Value>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "registration_open",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "registration_close",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("announced_at") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("start_date") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("end_date") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "competitor_limit",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("cancelled_at") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("url") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::__private::Some(__field10) => __field10,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("website") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::__private::Some(__field11) => __field11,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("short_name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field12 = match __field12 {
                            _serde::__private::Some(__field12) => __field12,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("city") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field13 = match __field13 {
                            _serde::__private::Some(__field13) => __field13,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "venue_address",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field14 = match __field14 {
                            _serde::__private::Some(__field14) => __field14,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "venue_details",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field15 = match __field15 {
                            _serde::__private::Some(__field15) => __field15,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "latitude_degrees",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field16 = match __field16 {
                            _serde::__private::Some(__field16) => __field16,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "longitude_degrees",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field17 = match __field17 {
                            _serde::__private::Some(__field17) => __field17,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("country_iso2") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field18 = match __field18 {
                            _serde::__private::Some(__field18) => __field18,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("event_ids") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field19 = match __field19 {
                            _serde::__private::Some(__field19) => __field19,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("delegates") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field20 = match __field20 {
                            _serde::__private::Some(__field20) => __field20,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("organizers") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Competition {
                            id: __field0,
                            name: __field1,
                            registration_open: __field2,
                            registration_close: __field3,
                            announced_at: __field4,
                            start_date: __field5,
                            end_date: __field6,
                            competitor_limit: __field7,
                            cancelled_at: __field8,
                            url: __field9,
                            website: __field10,
                            short_name: __field11,
                            city: __field12,
                            venue_address: __field13,
                            venue_details: __field14,
                            latitude_degrees: __field15,
                            longitude_degrees: __field16,
                            country_iso2: __field17,
                            event_ids: __field18,
                            delegates: __field19,
                            organizers: __field20,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "name",
                    "registration_open",
                    "registration_close",
                    "announced_at",
                    "start_date",
                    "end_date",
                    "competitor_limit",
                    "cancelled_at",
                    "url",
                    "website",
                    "short_name",
                    "city",
                    "venue_address",
                    "venue_details",
                    "latitude_degrees",
                    "longitude_degrees",
                    "country_iso2",
                    "event_ids",
                    "delegates",
                    "organizers",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Competition",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Competition>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Competition {
        pub fn from_json(json: &str) -> Vec<Competition> {
            serde_json::from_str(json).unwrap()
        }
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn id(&self) -> &str {
            &self.id
        }
    }
}
mod wcif_oauth {
    use std::ops::{Deref, DerefMut};
    use crate::{WcifContainer, OAuth};
    pub struct WcifOAuth {
        pub(crate) cont: WcifContainer,
        pub(crate) oauth: OAuth,
    }
    impl Deref for WcifOAuth {
        type Target = WcifContainer;
        fn deref(&self) -> &Self::Target {
            &self.cont
        }
    }
    impl DerefMut for WcifOAuth {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.cont
        }
    }
    impl WcifOAuth {
        pub async fn patch(&self) -> String {
            self.cont.patch(&self.oauth).await
        }
        pub fn disassemble(self) -> (WcifContainer, OAuth) {
            (self.cont, self.oauth)
        }
    }
}
pub mod noauth {
    mod traits {
        use async_trait::async_trait;
        use super::*;
        /// The special WCA loopback uri, useful in early development
        pub const LOOPBACK_URI: &str = "urn:ietf:wg:oauth:2.0:oob";
        /// Main trait for an oauth instance
        /// An implementation of this can be obtained through an OAuthBuilder
        pub trait OAuth {
            type Email: Email;
            type ManageCompetitions: ManageCompetitions;
            type DateOfBirth: DateOfBirth;
            #[must_use]
            #[allow(
                clippy::async_yields_async,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn me<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = String,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: ::core::marker::Sync + 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<String> {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: String = { __self.custom_route("me").await };
                    #[allow(unreachable_code)] __ret
                })
            }
            fn prefix(&self) -> &str;
            #[must_use]
            #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn custom_route<'life0, 'life1, 'async_trait>(
                &'life0 self,
                suffix: &'life1 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = String,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                Self: 'async_trait;
        }
        pub trait Refreshable {
            #[must_use]
            #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn refresh<'life0, 'async_trait>(
                &'life0 mut self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<(), String>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub trait Email {}
        impl Email for () {}
        pub trait ManageCompetitions {}
        impl ManageCompetitions for () {}
        pub trait DateOfBirth {}
        impl DateOfBirth for () {}
        /// Builder trait for building an oauth instance
        pub trait OAuthBuilder: Sized {
            type ImplicitOAuth;
            fn with_secret(
                self,
                client_id: String,
                secret: String,
                redirect_uri: String,
            ) -> WithSecret<Self> {
                WithSecret {
                    client_id,
                    secret,
                    redirect_uri,
                    inner: self,
                }
            }
            fn with_manage_competition_scope(self) -> WithManageCompetition<Self> {
                WithManageCompetition(self)
            }
            fn scopes(&self) -> Vec<&str>;
            fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth;
        }
        pub trait OAuthBuilderWithSecret: Sized {
            type ExplicitOAuth;
            #[must_use]
            #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn authenticate_explicit<'async_trait>(
                self,
                access_code: String,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Self::ExplicitOAuth, String>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                Self: 'async_trait;
        }
    }
    mod manage_competitions {
        use async_trait::async_trait;
        use super::*;
        pub struct WithManageCompetition<T>(pub(super) T);
        impl<T> OAuthBuilder for WithManageCompetition<T>
        where
            T: OAuthBuilder,
        {
            type ImplicitOAuth = ManageCompetitionsScope<T::ImplicitOAuth>;
            fn scopes(&self) -> Vec<&str> {
                let mut result = self.0.scopes();
                result.push("manage_competitions");
                result
            }
            fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth {
                ManageCompetitionsScope(self.0.authenticate_implicit(access_token))
            }
        }
        impl<T> OAuthBuilderWithSecret for WithManageCompetition<T>
        where
            T: OAuthBuilderWithSecret + Send,
        {
            type ExplicitOAuth = ManageCompetitionsScope<T::ExplicitOAuth>;
            #[allow(
                clippy::async_yields_async,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn authenticate_explicit<'async_trait>(
                self,
                access_code: String,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Self::ExplicitOAuth, String>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<
                            Result<Self::ExplicitOAuth, String>,
                        > {
                        return __ret;
                    }
                    let __self = self;
                    let access_code = access_code;
                    let __ret: Result<Self::ExplicitOAuth, String> = {
                        __self
                            .0
                            .authenticate_explicit(access_code)
                            .await
                            .map(|inner| ManageCompetitionsScope(inner))
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
        pub struct ManageCompetitionsScope<T>(T);
        pub struct ManageCompetitionTypes;
        impl ManageCompetitions for ManageCompetitionTypes {}
        impl<T> OAuth for ManageCompetitionsScope<T>
        where
            T: OAuth + Sync,
        {
            type Email = T::Email;
            type ManageCompetitions = ManageCompetitionTypes;
            type DateOfBirth = T::DateOfBirth;
            fn prefix(&self) -> &str {
                self.0.prefix()
            }
            #[allow(
                clippy::async_yields_async,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn custom_route<'life0, 'life1, 'async_trait>(
                &'life0 self,
                suffix: &'life1 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = String,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<String> {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: String = {
                        let result = __self.0.custom_route(suffix);
                        result.await
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
        impl<T> Refreshable for ManageCompetitionsScope<T>
        where
            T: Refreshable + Send + Sync,
        {
            fn refresh(&mut self) {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    mod secret {
        use super::*;
        use async_trait::async_trait;
        /// Inidicates an oauth which can be refreshed, i.e. is acquired through regular flow.
        pub trait Refreshable: OAuth {
            fn refresh(&mut self);
        }
        pub trait OAuthBuilderWithSecret {
            type ExplicitOAuth;
            #[must_use]
            #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn authenticate_explicit<'life0, 'async_trait>(
                self,
                auth_code: &'life0 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Self::ExplicitOAuth, String>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub struct WithSecret<T> {
            pub(super) client_id: String,
            pub(super) secret: String,
            pub(super) redirect_uri: String,
            pub(super) inner: T,
        }
        pub struct ExplicitOauth<T> {
            client_id: String,
            secret: String,
            redirect_uri: String,
            inner: T,
        }
    }
    mod base {
        use async_trait::async_trait;
        use reqwest::Client;
        use super::*;
        pub struct BaseOAuthBuilder;
        impl OAuthBuilder for BaseOAuthBuilder {
            type ImplicitOAuth = ImplicitOAuth;
            fn scopes(&self) -> Vec<&str> {
                ::alloc::vec::Vec::new()
            }
            fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth {
                ImplicitOAuth {
                    access_token,
                    prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned(),
                    client: Client::new(),
                }
            }
        }
        pub struct ImplicitOAuth {
            access_token: String,
            prefix: String,
            client: Client,
        }
        impl OAuth for ImplicitOAuth {
            type Email = ();
            type ManageCompetitions = ();
            type DateOfBirth = ();
            fn prefix(&self) -> &str {
                &self.prefix
            }
            #[allow(
                clippy::async_yields_async,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn custom_route<'life0, 'life1, 'async_trait>(
                &'life0 self,
                suffix: &'life1 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = String,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<String> {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: String = {
                        let url = {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}{1}", __self.prefix, suffix),
                            );
                            res
                        };
                        __self
                            .client
                            .get(&url)
                            .header(
                                "Authorization",
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Bearer {0}", __self.access_token),
                                    );
                                    res
                                },
                            )
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap()
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub use traits::*;
    pub use manage_competitions::*;
    pub use secret::*;
    pub use base::*;
}
use serde::{Deserializer, Deserialize, Serializer};
use serde::de::Error;
pub use wcif::*;
pub use oauth::*;
pub use wcif_oauth::*;
pub use competition::*;
pub use serde_with::chrono::{
    NaiveDateTime as DateTime, NaiveDate as Date, NaiveTime as Time, Datelike,
};
fn de_date_time<'de, D>(deserializer: D) -> std::result::Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.chars().last().unwrap() == 'Z' {
        Ok(
            serde_json::from_str(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("\"{0}\"", & s[0..s.len() - 1]),
                        );
                        res
                    },
                )
                .unwrap(),
        )
    } else {
        Err(D::Error::custom(s))
    }
}
fn ser_date_time<S>(
    date_time: &DateTime,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str = serde_json::to_string(&date_time).unwrap();
    serializer
        .serialize_str(
            &{
                let res = ::alloc::fmt::format(
                    format_args!("{0}Z", & str[1..str.len() - 1]),
                );
                res
            },
        )
}
