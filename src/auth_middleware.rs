use actix_web::http::header::Header;
use actix_web_httpauth::headers::authorization::{Authorization as ActixAuthorization, Basic};
use std::borrow::Cow;
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures_util::future::LocalBoxFuture;

pub struct Authorization;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match check_auth(&req) {
            Err(_) => Box::pin(async { Err(ErrorUnauthorized("Unauthorised")) }),
            Ok(_) => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
        }
    }
}

fn check_auth(req: &ServiceRequest) -> Result<(), String> {
    match ActixAuthorization::<Basic>::parse(req) {
        Err(e) => Err(e.to_string()),
        Ok(auth) => {
            if auth.as_ref().user_id() != "username"
                || auth.as_ref().password().unwrap_or(&Cow::from("")) != "password"
            {
                return Err("Unauthorised user".to_string());
            }
            Ok(())
        }
    }
}
