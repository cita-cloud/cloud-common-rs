#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(bytes = "vec", tag = "1")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub transaction_index: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub block_number: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub cumulative_quota_used: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub quota_used: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "8")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    #[prost(bytes = "vec", tag = "9")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "10")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "11")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "5")]
    pub block_number: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "7")]
    pub transaction_index: u64,
    #[prost(uint64, tag = "8")]
    pub log_index: u64,
    #[prost(uint64, tag = "9")]
    pub transaction_log_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteCode {
    #[prost(bytes = "vec", tag = "1")]
    pub byte_code: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nonce {
    #[prost(bytes = "vec", tag = "1")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteAbi {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes_abi: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteQuota {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes_quota: ::prost::alloc::vec::Vec<u8>,
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
        pub async fn get_transaction_receipt(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::Receipt>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/evm.RPCService/GetTransactionReceipt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Address>,
        ) -> Result<tonic::Response<super::ByteCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/evm.RPCService/GetCode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Address>,
        ) -> Result<tonic::Response<super::Balance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/evm.RPCService/GetBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_count(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Address>,
        ) -> Result<tonic::Response<super::Nonce>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/evm.RPCService/GetTransactionCount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_abi(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Address>,
        ) -> Result<tonic::Response<super::ByteAbi>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/evm.RPCService/GetAbi");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn estimate_quota(
            &mut self,
            request: impl tonic::IntoRequest<super::super::executor::CallRequest>,
        ) -> Result<tonic::Response<super::ByteQuota>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/evm.RPCService/EstimateQuota");
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
        async fn get_transaction_receipt(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::Receipt>, tonic::Status>;
        async fn get_code(
            &self,
            request: tonic::Request<super::super::common::Address>,
        ) -> Result<tonic::Response<super::ByteCode>, tonic::Status>;
        async fn get_balance(
            &self,
            request: tonic::Request<super::super::common::Address>,
        ) -> Result<tonic::Response<super::Balance>, tonic::Status>;
        async fn get_transaction_count(
            &self,
            request: tonic::Request<super::super::common::Address>,
        ) -> Result<tonic::Response<super::Nonce>, tonic::Status>;
        async fn get_abi(
            &self,
            request: tonic::Request<super::super::common::Address>,
        ) -> Result<tonic::Response<super::ByteAbi>, tonic::Status>;
        async fn estimate_quota(
            &self,
            request: tonic::Request<super::super::executor::CallRequest>,
        ) -> Result<tonic::Response<super::ByteQuota>, tonic::Status>;
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
                "/evm.RPCService/GetTransactionReceipt" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionReceiptSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetTransactionReceiptSvc<T>
                    {
                        type Response = super::Receipt;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_transaction_receipt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionReceiptSvc(inner);
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
                "/evm.RPCService/GetCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetCodeSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Address> for GetCodeSvc<T> {
                        type Response = super::ByteCode;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Address>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCodeSvc(inner);
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
                "/evm.RPCService/GetBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetBalanceSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Address>
                        for GetBalanceSvc<T>
                    {
                        type Response = super::Balance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Address>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBalanceSvc(inner);
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
                "/evm.RPCService/GetTransactionCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionCountSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Address>
                        for GetTransactionCountSvc<T>
                    {
                        type Response = super::Nonce;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Address>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction_count(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionCountSvc(inner);
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
                "/evm.RPCService/GetAbi" => {
                    #[allow(non_camel_case_types)]
                    struct GetAbiSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Address> for GetAbiSvc<T> {
                        type Response = super::ByteAbi;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Address>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_abi(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAbiSvc(inner);
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
                "/evm.RPCService/EstimateQuota" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateQuotaSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService>
                        tonic::server::UnaryService<super::super::executor::CallRequest>
                        for EstimateQuotaSvc<T>
                    {
                        type Response = super::ByteQuota;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::executor::CallRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).estimate_quota(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EstimateQuotaSvc(inner);
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
        const NAME: &'static str = "evm.RPCService";
    }
}
