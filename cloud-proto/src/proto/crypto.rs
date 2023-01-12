#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCryptoInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::StatusCode>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub hash_len: u32,
    #[prost(uint32, tag = "4")]
    pub signature_len: u32,
    #[prost(uint32, tag = "5")]
    pub address_len: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashDataRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyDataHashRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageRequest {
    #[prost(bytes = "vec", tag = "2")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::StatusCode>,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverSignatureRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverSignatureResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::common::StatusCode>,
    #[prost(bytes = "vec", tag = "2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod crypto_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CryptoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CryptoServiceClient<tonic::transport::Channel> {
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
    impl<T> CryptoServiceClient<T>
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
        ) -> CryptoServiceClient<InterceptedService<T, F>>
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
            CryptoServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get crypto info
        pub async fn get_crypto_info(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::GetCryptoInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/crypto.CryptoService/GetCryptoInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Hash data
        pub async fn hash_data(
            &mut self,
            request: impl tonic::IntoRequest<super::HashDataRequest>,
        ) -> Result<tonic::Response<super::super::common::HashResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/crypto.CryptoService/HashData");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Verify hash of data
        pub async fn verify_data_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyDataHashRequest>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/crypto.CryptoService/VerifyDataHash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sign a message
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageRequest>,
        ) -> Result<tonic::Response<super::SignMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/crypto.CryptoService/SignMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Recover signature
        pub async fn recover_signature(
            &mut self,
            request: impl tonic::IntoRequest<super::RecoverSignatureRequest>,
        ) -> Result<tonic::Response<super::RecoverSignatureResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/crypto.CryptoService/RecoverSignature");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// check transactions
        pub async fn check_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::super::blockchain::RawTransactions>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/crypto.CryptoService/CheckTransactions");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod crypto_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CryptoServiceServer.
    #[async_trait]
    pub trait CryptoService: Send + Sync + 'static {
        /// Get crypto info
        async fn get_crypto_info(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::GetCryptoInfoResponse>, tonic::Status>;
        /// Hash data
        async fn hash_data(
            &self,
            request: tonic::Request<super::HashDataRequest>,
        ) -> Result<tonic::Response<super::super::common::HashResponse>, tonic::Status>;
        /// Verify hash of data
        async fn verify_data_hash(
            &self,
            request: tonic::Request<super::VerifyDataHashRequest>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status>;
        /// Sign a message
        async fn sign_message(
            &self,
            request: tonic::Request<super::SignMessageRequest>,
        ) -> Result<tonic::Response<super::SignMessageResponse>, tonic::Status>;
        /// Recover signature
        async fn recover_signature(
            &self,
            request: tonic::Request<super::RecoverSignatureRequest>,
        ) -> Result<tonic::Response<super::RecoverSignatureResponse>, tonic::Status>;
        /// check transactions
        async fn check_transactions(
            &self,
            request: tonic::Request<super::super::blockchain::RawTransactions>,
        ) -> Result<tonic::Response<super::super::common::StatusCode>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CryptoServiceServer<T: CryptoService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CryptoService> CryptoServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CryptoServiceServer<T>
    where
        T: CryptoService,
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
                "/crypto.CryptoService/GetCryptoInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetCryptoInfoSvc<T: CryptoService>(pub Arc<T>);
                    impl<T: CryptoService> tonic::server::UnaryService<super::super::common::Empty>
                        for GetCryptoInfoSvc<T>
                    {
                        type Response = super::GetCryptoInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_crypto_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCryptoInfoSvc(inner);
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
                "/crypto.CryptoService/HashData" => {
                    #[allow(non_camel_case_types)]
                    struct HashDataSvc<T: CryptoService>(pub Arc<T>);
                    impl<T: CryptoService> tonic::server::UnaryService<super::HashDataRequest> for HashDataSvc<T> {
                        type Response = super::super::common::HashResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HashDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).hash_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HashDataSvc(inner);
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
                "/crypto.CryptoService/VerifyDataHash" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyDataHashSvc<T: CryptoService>(pub Arc<T>);
                    impl<T: CryptoService> tonic::server::UnaryService<super::VerifyDataHashRequest>
                        for VerifyDataHashSvc<T>
                    {
                        type Response = super::super::common::StatusCode;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyDataHashRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).verify_data_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyDataHashSvc(inner);
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
                "/crypto.CryptoService/SignMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SignMessageSvc<T: CryptoService>(pub Arc<T>);
                    impl<T: CryptoService> tonic::server::UnaryService<super::SignMessageRequest>
                        for SignMessageSvc<T>
                    {
                        type Response = super::SignMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignMessageSvc(inner);
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
                "/crypto.CryptoService/RecoverSignature" => {
                    #[allow(non_camel_case_types)]
                    struct RecoverSignatureSvc<T: CryptoService>(pub Arc<T>);
                    impl<T: CryptoService>
                        tonic::server::UnaryService<super::RecoverSignatureRequest>
                        for RecoverSignatureSvc<T>
                    {
                        type Response = super::RecoverSignatureResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecoverSignatureRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).recover_signature(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecoverSignatureSvc(inner);
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
                "/crypto.CryptoService/CheckTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct CheckTransactionsSvc<T: CryptoService>(pub Arc<T>);
                    impl<T: CryptoService>
                        tonic::server::UnaryService<super::super::blockchain::RawTransactions>
                        for CheckTransactionsSvc<T>
                    {
                        type Response = super::super::common::StatusCode;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::blockchain::RawTransactions>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check_transactions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckTransactionsSvc(inner);
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
    impl<T: CryptoService> Clone for CryptoServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CryptoService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CryptoService> tonic::server::NamedService for CryptoServiceServer<T> {
        const NAME: &'static str = "crypto.CryptoService";
    }
}
