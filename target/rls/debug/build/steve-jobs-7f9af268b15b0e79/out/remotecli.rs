/// Command input
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandInput {
    #[prost(string, tag = "1")]
    pub command: ::prost::alloc::string::String,
}
/// Command output
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandOutput {
    #[prost(string, tag = "1")]
    pub output: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod remote_cli_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service definition"]
    pub struct RemoteCliClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RemoteCliClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RemoteCliClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn shell(
            &mut self,
            request: impl tonic::IntoRequest<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/remotecli.RemoteCLI/Shell");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/remotecli.RemoteCLI/Stop");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn streamer(
            &mut self,
            request: impl tonic::IntoRequest<super::CommandInput>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::CommandOutput>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/remotecli.RemoteCLI/Streamer");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/remotecli.RemoteCLI/Status");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn result(
            &mut self,
            request: impl tonic::IntoRequest<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/remotecli.RemoteCLI/Result");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RemoteCliClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RemoteCliClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RemoteCliClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod remote_cli_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RemoteCliServer."]
    #[async_trait]
    pub trait RemoteCli: Send + Sync + 'static {
        async fn shell(
            &self,
            request: tonic::Request<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status>;
        async fn stop(
            &self,
            request: tonic::Request<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status>;
        #[doc = "Server streaming response type for the Streamer method."]
        type StreamerStream: Stream<Item = Result<super::CommandOutput, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn streamer(
            &self,
            request: tonic::Request<super::CommandInput>,
        ) -> Result<tonic::Response<Self::StreamerStream>, tonic::Status>;
        async fn status(
            &self,
            request: tonic::Request<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status>;
        async fn result(
            &self,
            request: tonic::Request<super::CommandInput>,
        ) -> Result<tonic::Response<super::CommandOutput>, tonic::Status>;
    }
    #[doc = " Service definition"]
    #[derive(Debug)]
    pub struct RemoteCliServer<T: RemoteCli> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RemoteCli> RemoteCliServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for RemoteCliServer<T>
    where
        T: RemoteCli,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/remotecli.RemoteCLI/Shell" => {
                    #[allow(non_camel_case_types)]
                    struct ShellSvc<T: RemoteCli>(pub Arc<T>);
                    impl<T: RemoteCli> tonic::server::UnaryService<super::CommandInput> for ShellSvc<T> {
                        type Response = super::CommandOutput;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommandInput>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).shell(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ShellSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/remotecli.RemoteCLI/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: RemoteCli>(pub Arc<T>);
                    impl<T: RemoteCli> tonic::server::UnaryService<super::CommandInput> for StopSvc<T> {
                        type Response = super::CommandOutput;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommandInput>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/remotecli.RemoteCLI/Streamer" => {
                    #[allow(non_camel_case_types)]
                    struct StreamerSvc<T: RemoteCli>(pub Arc<T>);
                    impl<T: RemoteCli> tonic::server::ServerStreamingService<super::CommandInput> for StreamerSvc<T> {
                        type Response = super::CommandOutput;
                        type ResponseStream = T::StreamerStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommandInput>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).streamer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = StreamerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/remotecli.RemoteCLI/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: RemoteCli>(pub Arc<T>);
                    impl<T: RemoteCli> tonic::server::UnaryService<super::CommandInput> for StatusSvc<T> {
                        type Response = super::CommandOutput;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommandInput>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/remotecli.RemoteCLI/Result" => {
                    #[allow(non_camel_case_types)]
                    struct ResultSvc<T: RemoteCli>(pub Arc<T>);
                    impl<T: RemoteCli> tonic::server::UnaryService<super::CommandInput> for ResultSvc<T> {
                        type Response = super::CommandOutput;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommandInput>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).result(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ResultSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RemoteCli> Clone for RemoteCliServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RemoteCli> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RemoteCli> tonic::transport::NamedService for RemoteCliServer<T> {
        const NAME: &'static str = "remotecli.RemoteCLI";
    }
}
