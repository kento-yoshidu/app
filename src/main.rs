use actix_web::{web, App, HttpServer, HttpResponse};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::http::playground_source;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &str {
        "Hello World"
    }
}

/// ハンドラ
async fn graphql_handler(
    schema: web::Data<MySchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Playground UI を表示
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

/// スキーマの型
type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish();

    println!("🚀 サーバー起動: http://localhost:8000");
    println!("🚀 http://localhost:8000/playground で Playground を開く");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/", web::post().to(graphql_handler))
            .route("/playground", web::get().to(graphql_playground))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
