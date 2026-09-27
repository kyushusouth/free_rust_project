use crate::request::Request;
use crate::response::Response;
use crate::stages::{explanation, reranking, retrieval};

pub fn run(request: &Request) -> Response {
    // request は参照のため、関数を呼び出した後も有効なままである。

    // 所有権を渡す実装
    // let items = retrieval::search(request);
    // let items = reranking::rerank(request, items);
    // let items = explanation::annotate(request, items);

    // 所有権を渡さない実装
    let mut items = retrieval::search(request);
    reranking::rerank_ref(request, &mut items);
    explanation::annotate_ref(request, &mut items);

    Response { items }
} // request の所有権が drop される。
