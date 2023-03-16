#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(uint32, tag = "1")]
    pub region: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtKey {
    #[prost(uint32, tag = "1")]
    pub region: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::StatusCode>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Regions {
    Global = 0,
    Transactions = 1,
    Headers = 2,
    Bodies = 3,
    BlockHash = 4,
    Proof = 5,
    Result = 6,
    TransactionHash2blockHeight = 7,
    /// In SQL db, reuse 4
    BlockHash2blockHeight = 8,
    TransactionIndex = 9,
    CompactBlock = 10,
    FullBlock = 11,
    AllBlockData = 12,
    Button = 13,
}
impl Regions {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Regions::Global => "GLOBAL",
            Regions::Transactions => "TRANSACTIONS",
            Regions::Headers => "HEADERS",
            Regions::Bodies => "BODIES",
            Regions::BlockHash => "BLOCK_HASH",
            Regions::Proof => "PROOF",
            Regions::Result => "RESULT",
            Regions::TransactionHash2blockHeight => "TRANSACTION_HASH2BLOCK_HEIGHT",
            Regions::BlockHash2blockHeight => "BLOCK_HASH2BLOCK_HEIGHT",
            Regions::TransactionIndex => "TRANSACTION_INDEX",
            Regions::CompactBlock => "COMPACT_BLOCK",
            Regions::FullBlock => "FULL_BLOCK",
            Regions::AllBlockData => "All_BLOCK_DATA",
            Regions::Button => "BUTTON",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GLOBAL" => Some(Self::Global),
            "TRANSACTIONS" => Some(Self::Transactions),
            "HEADERS" => Some(Self::Headers),
            "BODIES" => Some(Self::Bodies),
            "BLOCK_HASH" => Some(Self::BlockHash),
            "PROOF" => Some(Self::Proof),
            "RESULT" => Some(Self::Result),
            "TRANSACTION_HASH2BLOCK_HEIGHT" => Some(Self::TransactionHash2blockHeight),
            "BLOCK_HASH2BLOCK_HEIGHT" => Some(Self::BlockHash2blockHeight),
            "TRANSACTION_INDEX" => Some(Self::TransactionIndex),
            "COMPACT_BLOCK" => Some(Self::CompactBlock),
            "FULL_BLOCK" => Some(Self::FullBlock),
            "All_BLOCK_DATA" => Some(Self::AllBlockData),
            "BUTTON" => Some(Self::Button),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod storage_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StorageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageServiceClient<tonic::transport::Channel> {
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
    impl<T> StorageServiceClient<T>
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
        ) -> StorageServiceClient<InterceptedService<T, F>>
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
            StorageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// store key/value
        pub async fn store(
            &mut self,
            request: impl tonic::IntoRequest<super::Content>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.StorageService/Store");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// given a ext key return value
        pub async fn load(
            &mut self,
            request: impl tonic::IntoRequest<super::ExtKey>,
        ) -> Result<tonic::Response<super::Value>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.StorageService/Load");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// given a ext key delete it
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ExtKey>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.StorageService/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod storage_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StorageServiceServer.
    #[async_trait]
    pub trait StorageService: Send + Sync + 'static {
        /// store key/value
        async fn store(
            &self,
            request: tonic::Request<super::Content>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status>;
        /// given a ext key return value
        async fn load(
            &self,
            request: tonic::Request<super::ExtKey>,
        ) -> Result<tonic::Response<super::Value>, tonic::Status>;
        /// given a ext key delete it
        async fn delete(
            &self,
            request: tonic::Request<super::ExtKey>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StorageServiceServer<T: StorageService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StorageService> StorageServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageServiceServer<T>
    where
        T: StorageService,
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
                "/storage.StorageService/Store" => {
                    #[allow(non_camel_case_types)]
                    struct StoreSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::UnaryService<super::Content> for StoreSvc<T> {
                        type Response = super::super::common::StatusCode;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Content>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).store(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StoreSvc(inner);
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
                "/storage.StorageService/Load" => {
                    #[allow(non_camel_case_types)]
                    struct LoadSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::UnaryService<super::ExtKey> for LoadSvc<T> {
                        type Response = super::Value;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ExtKey>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).load(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoadSvc(inner);
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
                "/storage.StorageService/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::UnaryService<super::ExtKey> for DeleteSvc<T> {
                        type Response = super::super::common::StatusCode;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ExtKey>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
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
    impl<T: StorageService> Clone for StorageServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: StorageService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StorageService> tonic::server::NamedService for StorageServiceServer<T> {
        const NAME: &'static str = "storage.StorageService";
    }
}
