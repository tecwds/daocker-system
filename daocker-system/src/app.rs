use salvo::{conn::TcpListener, cors::Cors, http::Method, Listener, Router, Server, Service};
use crate::{router::{login, student_route}, setting::SETTINGS};

pub async fn app() {
    tracing_subscriber::fmt().init();

    let addr = SETTINGS.get_server().get_addr();
    let port = SETTINGS.get_server().get_port();

    let cors = Cors::new()
        .allow_origin("**")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    let acceptor = TcpListener::new(format!("{}:{}", addr, port)).bind().await;
    let server = Server::new(acceptor);
    
    let mut router = Router::new();
    router.routers_mut().push(Router::with_path("/api/login").post(login::login));
    router.routers_mut().push(Router::with_path("/index").post(login::index));
    router.routers_mut().push(Router::with_path("/api/student/list").get(student_route::get_student));
    router.routers_mut().push(Router::with_path("/api/student/mod").post(student_route::post_stu));
    router.routers_mut().push(Router::with_path("/api/student/del").post(student_route::del_edu));
    
    let service = Service::new(router).hoop(cors);

    server.serve(service).await;
}
