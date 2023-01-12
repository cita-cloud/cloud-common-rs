#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flag {
    #[prost(bool, tag = "1")]
    pub flag: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockNumber {
    #[prost(uint64, tag = "1")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemConfig {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub chain_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub block_interval: u32,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, tag = "6")]
    pub emergency_brake: bool,
    #[prost(bytes = "vec", tag = "7")]
    pub version_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub chain_id_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "9")]
    pub admin_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "10")]
    pub block_interval_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "11")]
    pub validators_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "12")]
    pub emergency_brake_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "13")]
    pub quota_limit: u32,
    #[prost(bytes = "vec", tag = "14")]
    pub quota_limit_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "15")]
    pub block_limit: u32,
    #[prost(bytes = "vec", tag = "16")]
    pub block_limit_pre_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionIndex {
    #[prost(uint64, tag = "1")]
    pub tx_index: u64,
}
/// Generated client implementations.
pub mod rpc_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct RpcServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RpcServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RpcServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RpcServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RpcServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// flag means latest or pending.
        /// true means pending, false means latest.
        pub async fn get_block_number(
            &mut self,
            request: impl tonic::IntoRequest<super::Flag>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockNumber");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_raw_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::blockchain::RawTransaction>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/SendRawTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_raw_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::super::blockchain::RawTransactions>,
        ) -> Result<tonic::Response<super::super::common::Hashes>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/SendRawTransactions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_by_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockByHash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_height_by_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetHeightByHash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockByNumber");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_state_root_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::StateRoot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetStateRootByNumber");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_proof_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::Proof>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetProofByNumber");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_detail_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::blockchain::Block>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.RPCService/GetBlockDetailByNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::RawTransaction>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_system_config(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::SystemConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetSystemConfig");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_system_config_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::SystemConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.RPCService/GetSystemConfigByNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockHash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_block_number(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.RPCService/GetTransactionBlockNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_index(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::TransactionIndex>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetTransactionIndex");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// add new node
        pub async fn add_node(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::NodeNetInfo>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/controller.RPCService/AddNode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_node_status(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::super::common::NodeStatus>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/controller.RPCService/GetNodeStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod consensus2_controller_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct Consensus2ControllerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl Consensus2ControllerServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> Consensus2ControllerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> Consensus2ControllerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            Consensus2ControllerServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Consensus request a Proposal to start consensus
        /// ret: proposal
        pub async fn get_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::super::common::ProposalResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.Consensus2ControllerService/GetProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// when Consensus received a new proposal from other nodes, it will ask controller to check it
        /// args: proposal hash
        /// ret: ok or not
        pub async fn check_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Proposal>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.Consensus2ControllerService/CheckProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// after Consensus, tell controller a proposal has committed
        pub async fn commit_block(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::ProposalWithProof>,
        ) -> Result<
            tonic::Response<super::super::common::ConsensusConfigurationResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.Consensus2ControllerService/CommitBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod rpc_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RpcServiceServer.
    #[async_trait]
    pub trait RpcService: Send + Sync + 'static {
        /// flag means latest or pending.
        /// true means pending, false means latest.
        async fn get_block_number(
            &self,
            request: tonic::Request<super::Flag>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status>;
        async fn send_raw_transaction(
            &self,
            request: tonic::Request<super::super::blockchain::RawTransaction>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status>;
        async fn send_raw_transactions(
            &self,
            request: tonic::Request<super::super::blockchain::RawTransactions>,
        ) -> Result<tonic::Response<super::super::common::Hashes>, tonic::Status>;
        async fn get_block_by_hash(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>;
        async fn get_height_by_hash(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status>;
        async fn get_block_by_number(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>;
        async fn get_state_root_by_number(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::StateRoot>, tonic::Status>;
        async fn get_proof_by_number(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::Proof>, tonic::Status>;
        async fn get_block_detail_by_number(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::blockchain::Block>, tonic::Status>;
        async fn get_transaction(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::RawTransaction>, tonic::Status>;
        async fn get_system_config(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::SystemConfig>, tonic::Status>;
        async fn get_system_config_by_number(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::SystemConfig>, tonic::Status>;
        async fn get_block_hash(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status>;
        async fn get_transaction_block_number(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status>;
        async fn get_transaction_index(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::TransactionIndex>, tonic::Status>;
        /// add new node
        async fn add_node(
            &self,
            request: tonic::Request<super::super::common::NodeNetInfo>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status>;
        async fn get_node_status(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::super::common::NodeStatus>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RpcServiceServer<T: RpcService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RpcService> RpcServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RpcServiceServer<T>
    where
        T: RpcService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/controller.RPCService/GetBlockNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::Flag> for GetBlockNumberSvc<T> {
                        type Response = super::BlockNumber;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Flag>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/SendRawTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendRawTransactionSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService>
                        tonic::server::UnaryService<super::super::blockchain::RawTransaction>
                        for SendRawTransactionSvc<T>
                    {
                        type Response = super::super::common::Hash;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::blockchain::RawTransaction>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_raw_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendRawTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/SendRawTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct SendRawTransactionsSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService>
                        tonic::server::UnaryService<super::super::blockchain::RawTransactions>
                        for SendRawTransactionsSvc<T>
                    {
                        type Response = super::super::common::Hashes;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::blockchain::RawTransactions>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_raw_transactions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendRawTransactionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetBlockByHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByHashSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetBlockByHashSvc<T>
                    {
                        type Response = super::super::blockchain::CompactBlock;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_by_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetHeightByHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetHeightByHashSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetHeightByHashSvc<T>
                    {
                        type Response = super::BlockNumber;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_height_by_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetHeightByHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetBlockByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber> for GetBlockByNumberSvc<T> {
                        type Response = super::super::blockchain::CompactBlock;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_by_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetStateRootByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetStateRootByNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber> for GetStateRootByNumberSvc<T> {
                        type Response = super::super::common::StateRoot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_state_root_by_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStateRootByNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetProofByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetProofByNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber> for GetProofByNumberSvc<T> {
                        type Response = super::super::common::Proof;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_proof_by_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProofByNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetBlockDetailByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockDetailByNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber>
                        for GetBlockDetailByNumberSvc<T>
                    {
                        type Response = super::super::blockchain::Block;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_block_detail_by_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockDetailByNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetTransactionSvc<T>
                    {
                        type Response = super::super::blockchain::RawTransaction;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetSystemConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetSystemConfigSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Empty>
                        for GetSystemConfigSvc<T>
                    {
                        type Response = super::SystemConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_system_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSystemConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetSystemConfigByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetSystemConfigByNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber>
                        for GetSystemConfigByNumberSvc<T>
                    {
                        type Response = super::SystemConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_system_config_by_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSystemConfigByNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetBlockHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHashSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber> for GetBlockHashSvc<T> {
                        type Response = super::super::common::Hash;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetTransactionBlockNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionBlockNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetTransactionBlockNumberSvc<T>
                    {
                        type Response = super::BlockNumber;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_transaction_block_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionBlockNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetTransactionIndex" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionIndexSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetTransactionIndexSvc<T>
                    {
                        type Response = super::TransactionIndex;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction_index(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/AddNode" => {
                    #[allow(non_camel_case_types)]
                    struct AddNodeSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService>
                        tonic::server::UnaryService<super::super::common::NodeNetInfo>
                        for AddNodeSvc<T>
                    {
                        type Response = super::super::common::StatusCode;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::NodeNetInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_node(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddNodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetNodeStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeStatusSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Empty>
                        for GetNodeStatusSvc<T>
                    {
                        type Response = super::super::common::NodeStatus;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_node_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodeStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RpcService> Clone for RpcServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RpcService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RpcService> tonic::server::NamedService for RpcServiceServer<T> {
        const NAME: &'static str = "controller.RPCService";
    }
}
/// Generated server implementations.
pub mod consensus2_controller_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with Consensus2ControllerServiceServer.
    #[async_trait]
    pub trait Consensus2ControllerService: Send + Sync + 'static {
        /// Consensus request a Proposal to start consensus
        /// ret: proposal
        async fn get_proposal(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::super::common::ProposalResponse>, tonic::Status>;
        /// when Consensus received a new proposal from other nodes, it will ask controller to check it
        /// args: proposal hash
        /// ret: ok or not
        async fn check_proposal(
            &self,
            request: tonic::Request<super::super::common::Proposal>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status>;
        /// after Consensus, tell controller a proposal has committed
        async fn commit_block(
            &self,
            request: tonic::Request<super::super::common::ProposalWithProof>,
        ) -> Result<
            tonic::Response<super::super::common::ConsensusConfigurationResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct Consensus2ControllerServiceServer<T: Consensus2ControllerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Consensus2ControllerService> Consensus2ControllerServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for Consensus2ControllerServiceServer<T>
    where
        T: Consensus2ControllerService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/controller.Consensus2ControllerService/GetProposal" => {
                    #[allow(non_camel_case_types)]
                    struct GetProposalSvc<T: Consensus2ControllerService>(pub Arc<T>);
                    impl<T: Consensus2ControllerService>
                        tonic::server::UnaryService<super::super::common::Empty>
                        for GetProposalSvc<T>
                    {
                        type Response = super::super::common::ProposalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_proposal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.Consensus2ControllerService/CheckProposal" => {
                    #[allow(non_camel_case_types)]
                    struct CheckProposalSvc<T: Consensus2ControllerService>(pub Arc<T>);
                    impl<T: Consensus2ControllerService>
                        tonic::server::UnaryService<super::super::common::Proposal>
                        for CheckProposalSvc<T>
                    {
                        type Response = super::super::common::StatusCode;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Proposal>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check_proposal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.Consensus2ControllerService/CommitBlock" => {
                    #[allow(non_camel_case_types)]
                    struct CommitBlockSvc<T: Consensus2ControllerService>(pub Arc<T>);
                    impl<T: Consensus2ControllerService>
                        tonic::server::UnaryService<super::super::common::ProposalWithProof>
                        for CommitBlockSvc<T>
                    {
                        type Response = super::super::common::ConsensusConfigurationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::ProposalWithProof>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).commit_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CommitBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Consensus2ControllerService> Clone for Consensus2ControllerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Consensus2ControllerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Consensus2ControllerService> tonic::server::NamedService
        for Consensus2ControllerServiceServer<T>
    {
        const NAME: &'static str = "controller.Consensus2ControllerService";
    }
}
