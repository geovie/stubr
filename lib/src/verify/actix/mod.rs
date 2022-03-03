use std::fmt::Debug;

use actix_http::Request as ActixRequest;
use actix_service::{IntoServiceFactory, Service as ActixService, ServiceFactory};
use actix_web::{dev::{AppConfig, ServiceResponse}, test::TestRequest};
use async_trait::async_trait;

use super::{
    mapping::{req::StdRequest, resp::{RequestAndStub, StdResponse}},
    stub_finder::ProducerStubFinder,
    StubrVerify,
};

mod req;
mod resp;

#[async_trait(? Send)]
impl<A, T> StubrVerify<T> for A where
    A: IntoServiceFactory<T, ActixRequest>,
    T: ServiceFactory<ActixRequest, Config=AppConfig, Response=ServiceResponse>,
    <T as ServiceFactory<ActixRequest>>::InitError: Debug,
{
    async fn verify(self) {
        let srv = self.into_factory();
        let app = srv.new_service(AppConfig::default()).await.unwrap();
        for (stub, name) in ProducerStubFinder::find_stubs() {
            let mut req = StdRequest::from(&stub);
            let test_req = TestRequest::from(&mut req).to_request();
            let resp: StdResponse = app.call(test_req).await
                .unwrap_or_else(|_| panic!("Failed verifying stub {:?}", name))
                .into();
            RequestAndStub { req, stub: stub.response, name }.verify(resp);
        };
    }
}