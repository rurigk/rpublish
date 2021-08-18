use std::sync::Mutex;
use std::task::{Context, Poll};

use actix_web::{HttpMessage, HttpResponse, http, web};
use actix_web::{dev::ServiceRequest, dev::Service, dev::Transform, dev::ServiceResponse, Error};
use futures::future::Either;
use futures::{future::{ok, Ready}};

use crate::rpublish;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct LoggedIn;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for LoggedIn
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggedInMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggedInMiddleware { service })
    }
}

pub struct LoggedInMiddleware<S> {
    service: S,
}

impl<S, B> Service for LoggedInMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    
    #[allow(clippy::type_complexity)]
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Some(sessid_cookie) = req.cookie("SESSID") {
            let app = req.app_data::<web::Data<Mutex<rpublish::RPublishApp>>>()
                .unwrap()
                .lock()
                .unwrap();
            
            let valid_session = app.identity_manager.sessions.validate(&sessid_cookie.value().to_string());
            drop(app);

            if valid_session
            {
                Either::Left(self.service.call(req))
            } else {
                Either::Right(ok(req.into_response(
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/auth/login")
                        .finish()
                        .into_body(),
                )))
            }
        } else {
            Either::Right(ok(req.into_response(
                HttpResponse::Found()
                    .header(http::header::LOCATION, "/auth/login")
                    .finish()
                    .into_body(),
            )))
        }
    }
}
