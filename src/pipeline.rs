use crate::request::Request;
use crate::response::Response;
use crate::stages::{retrieval, reranking, explanation};

pub fn run(request: &Request) -> Response {
    let items = retrieval::search(request);
    let items = reranking::rerank(request, items);
    let items = explanation::annotate(request, items);

    Response { items }
}
