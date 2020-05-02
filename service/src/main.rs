use warp::Filter;


static ASSETS_PATH: Option<&str> = option_env!("ASSETS_PATH");

#[tokio::main]
async fn main() {
    let assets_path: &str = ASSETS_PATH.unwrap_or("../client");

    let home = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", assets_path)));

    let assets = warp::get().and(warp::path("assets"))
        .and(warp::fs::dir(format!("{}/assets", assets_path)));

    let routes = home.or(assets);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}