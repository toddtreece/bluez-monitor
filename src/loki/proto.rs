#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushRequest {
    #[prost(message, repeated, tag = "1")]
    pub streams: ::prost::alloc::vec::Vec<StreamAdapter>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAdapter {
    #[prost(string, tag = "1")]
    pub labels: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<EntryAdapter>,
    #[prost(uint64, tag = "3")]
    pub hash: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryAdapter {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "2")]
    pub line: ::prost::alloc::string::String,
}
