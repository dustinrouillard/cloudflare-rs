use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Modifies a namespace's title.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/update/>
#[derive(Debug)]
pub struct RenameNamespace<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub params: RenameNamespaceParams,
}

impl EndpointSpec for RenameNamespace<'_> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_identifier, self.namespace_identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct RenameNamespaceParams {
    pub title: String,
}
