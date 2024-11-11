use daocker_system::app::app;

#[tokio::main]
async fn main() {
    let app_ = app();
    println!("盗🉑 er 启动！");
    app_.await;
}