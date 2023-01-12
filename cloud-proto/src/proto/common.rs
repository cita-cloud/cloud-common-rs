#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateRoot {
    #[prost(bytes = "vec", tag = "1")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proof {
    #[prost(bytes = "vec", tag = "1")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hashes {
    #[prost(message, repeated, tag = "1")]
    pub hashes: ::prost::alloc::vec::Vec<Hash>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalWithProof {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BftProposal {
    #[prost(bytes = "vec", tag = "1")]
    pub pre_state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub pre_proof: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proposal: ::core::option::Option<super::blockchain::Block>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalEnum {
    #[prost(oneof = "proposal_enum::Proposal", tags = "1")]
    pub proposal: ::core::option::Option<proposal_enum::Proposal>,
}
/// Nested message and enum types in `ProposalEnum`.
pub mod proposal_enum {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Proposal {
        #[prost(message, tag = "1")]
        BftProposal(super::BftProposal),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusConfiguration {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub block_interval: u32,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusCode {
    #[prost(uint32, tag = "1")]
    pub code: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<StatusCode>,
    #[prost(message, optional, tag = "2")]
    pub hash: ::core::option::Option<Hash>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<StatusCode>,
    #[prost(message, optional, tag = "2")]
    pub proposal: ::core::option::Option<Proposal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusConfigurationResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<StatusCode>,
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<ConsensusConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeNetInfo {
    #[prost(string, tag = "1")]
    pub multi_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub origin: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalNodeNetInfo {
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<NodeNetInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerStatus {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub node_net_info: ::core::option::Option<NodeNetInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeStatus {
    #[prost(bool, tag = "1")]
    pub is_sync: bool,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub self_status: ::core::option::Option<PeerStatus>,
    #[prost(uint64, tag = "4")]
    pub peers_count: u64,
    #[prost(message, repeated, tag = "5")]
    pub peers_status: ::prost::alloc::vec::Vec<PeerStatus>,
    #[prost(bool, tag = "6")]
    pub is_danger: bool,
}
