use crate::proto_generated::common::v1::Uuid;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(message, required, tag = "1")]
    pub id: Uuid,
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub password_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub avatar_url: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub email_verified: bool,
    #[prost(message, required, tag = "7")]
    pub created_at: ::prost_types::Timestamp,
    #[prost(message, required, tag = "8")]
    pub updated_at: ::prost_types::Timestamp,
    #[prost(message, optional, tag = "9")]
    pub deleted_at: ::core::option::Option<::prost_types::Timestamp>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserRequest {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginUserRequest {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginUserResponse {
    #[prost(string, tag = "1")]
    pub session_token: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub user: ::core::option::Option<User>,
}
