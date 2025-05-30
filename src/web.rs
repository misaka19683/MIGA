//! Web 服务器模块，提供 HTTP 接口以允许其他人下载内容

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use log::{info, warn};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tower_http::services::ServeDir;

/// 表示可以共享的内容
pub struct SharedContent {
    /// 内容的 CID
    pub cid: String,
    /// 内容在本地存储的路径
    pub path: PathBuf,
    /// 内容的描述
    pub description: Option<String>,
}

/// Web 服务器状态
pub struct WebServerState {
    /// 所有可以共享的内容
    shared_contents: Arc<Mutex<Vec<SharedContent>>>,
    /// 服务器根目录路径
    serve_dir: PathBuf,
}

/// 启动 Web 服务器以允许其他人下载内容
/// 
/// # 参数
/// * `port` - 服务器监听的端口
/// * `serve_dir` - 提供下载服务的目录路径
pub async fn run_web_server(port: u16, serve_dir: PathBuf) -> Result<mpsc::Sender<SharedContent>> {
    // 创建一个通道，用于接收新的共享内容
    let (content_sender, mut content_receiver) = mpsc::channel::<SharedContent>(100);

    // 创建共享状态
    let state = Arc::new(WebServerState {
        shared_contents: Arc::new(Mutex::new(Vec::new())),
        serve_dir: serve_dir.clone(),
    });

    // 确保服务目录存在
    if !serve_dir.exists() {
        std::fs::create_dir_all(&serve_dir)?;
    }

    // 克隆状态以供任务使用
    let state_clone = state.clone();

    // 启动后台任务以接收新的共享内容
    tokio::spawn(async move {
        while let Some(content) = content_receiver.recv().await {
            info!("接收到新的共享内容: {}", content.cid);
            let mut contents = state_clone.shared_contents.lock().unwrap();
            contents.push(content);
        }
    });

    // 设置路由
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/list", get(list_handler))
        .nest_service("/files", ServeDir::new(&serve_dir))
        .with_state(state);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    info!("Web 服务器正在监听 {}", addr);

    // 启动服务器
    tokio::spawn(async move {
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => listener,
            Err(err) => {
                warn!("Web 服务器绑定地址失败: {}", err);
                return;
            }
        };

        if let Err(err) = axum::serve(listener, app).await {
            warn!("Web 服务器发生错误: {}", err);
        }
    });

    Ok(content_sender)
}

/// 首页处理函数
async fn index_handler() -> Html<String> {
    Html("\
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>MIGA IPFS 内容共享</title>
        <style>
            body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
            h1 { color: #333; }
            .info { background-color: #f0f0f0; padding: 15px; border-radius: 5px; }
            a { color: #0066cc; text-decoration: none; }
            a:hover { text-decoration: underline; }
        </style>
    </head>
    <body>
        <h1>MIGA IPFS 内容共享</h1>
        <div class=\"info\">
            <p>欢迎使用 MIGA IPFS 内容共享服务!</p>
            <p>这个服务允许您从此节点下载 IPFS 内容。</p>
            <p><a href=\"/list\">查看可用内容列表</a></p>
        </div>
    </body>
    </html>
    ".to_string())
}

/// 列出所有可共享内容的处理函数
async fn list_handler(State(state): State<Arc<WebServerState>>) -> impl IntoResponse {
    let contents = state.shared_contents.lock().unwrap();

    if contents.is_empty() {
        return (StatusCode::OK, Html("<h1>暂无可共享内容</h1><p>当前没有可用的内容。</p><p><a href='/'>返回首页</a></p>".to_string()));
    }

    let mut html = String::from(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>MIGA IPFS 内容列表</title>
        <style>
            body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
            h1 { color: #333; }
            .content-list { margin-top: 20px; }
            .content-item { background-color: #f0f0f0; padding: 15px; border-radius: 5px; margin-bottom: 10px; }
            .content-title { font-weight: bold; margin-bottom: 5px; }
            .content-cid { font-family: monospace; color: #666; }
            .content-description { margin-top: 5px; }
            .download-link { display: inline-block; background-color: #0066cc; color: white; padding: 5px 10px; 
                            border-radius: 3px; text-decoration: none; margin-top: 10px; }
            .download-link:hover { background-color: #0055aa; }
            .back-link { margin-top: 20px; display: block; }
        </style>
    </head>
    <body>
        <h1>可用 IPFS 内容列表</h1>
    "#);

    html.push_str("<div class=\"content-list\">");

    for (index, content) in contents.iter().enumerate() {
        let file_name = content.path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        html.push_str(&format!(r#"
            <div class="content-item">
                <div class="content-title">内容 #{}</div>
                <div class="content-cid">CID: {}</div>
                {}
                <div>文件名: {}</div>
                <a href="/files/{}" class="download-link">下载</a>
            </div>
        "#, 
        index + 1,
        content.cid,
        content.description.as_ref().map_or(String::new(), |desc| format!("<div class=\"content-description\">描述: {}</div>", desc)),
        file_name,
        file_name
        ));
    }

    html.push_str("</div>");
    html.push_str("<a href=\"/\" class=\"back-link\">返回首页</a>");
    html.push_str("</body></html>");

    (StatusCode::OK, Html(html))
}
