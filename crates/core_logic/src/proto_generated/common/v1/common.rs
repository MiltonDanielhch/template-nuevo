#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(message, required, tag = "1")]
    pub value: ::prost_types::Timestamp,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalTimestamp {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost_types::Timestamp>,
}
