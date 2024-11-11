use jsonwebtoken::EncodingKey;
use salvo::{handler, hyper::header::AUTHORIZATION, writing::Json, Depot, Request, Response};
use tracing::info;

use crate::{
    entity::{login_dto::LoginDTO, result::ResultEntity, student::Student},
    service::student_service::{get_student_by_email, get_student_by_id},
    setting::SETTINGS,
    util::JwtClaims,
};

#[handler]
pub async fn login(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let body = req.parse_json::<LoginDTO>().await.unwrap();

    let mut student: Option<Student> = None;
    let mut flag: bool = false;
    if let Some(student_id) = body.get_student_id() {
        let stu = get_student_by_id(student_id.to_owned()).await;

        if stu.is_err() {
            info!("err -- stu value = {:?}", stu);
            res.render(Json(ResultEntity::<String>::error("账户名或者密码错误")));
            return;
        }

        let stu = stu.unwrap();

        student = Some(stu);
        flag = true;
    }

    if let Some(email) = body.get_email() {
        if !flag {
            let stu = get_student_by_email(email.to_owned()).await;

            if stu.is_err() {
                info!("err -- stu value = {:?}", stu);
                res.render(Json(ResultEntity::<String>::error("账户名或者密码错误")));
                return;
            }
            student = Some(stu.unwrap());
            flag = true;
        }
    }

    if !flag {
        res.render(Json(ResultEntity::<String>::error("账户名或者密码错误")));
        return;
    }

    let student = student.unwrap();

    if *body.get_password() != student.password {
        res.render(Json(ResultEntity::<String>::error("账户名或者密码错误")));
        return;
    }

    // 加入 cookie

    let claim = JwtClaims {
        username: student.get_student_id().clone(),
        exp: *SETTINGS.get_token().get_expire(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(SETTINGS.get_token().get_secret_key().as_bytes()),
    )
    .unwrap();

    res.render(Json(ResultEntity::<String>::success(Some(String::from(
        token,
    )))));
}

#[handler]
pub async fn index(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let token_val = req.headers().get(AUTHORIZATION);

    if token_val.is_none() {
        res.render(Json(ResultEntity::<String>::error("没有登陆，滚蛋")));
        return;
    }

    // 验证 Token

    //    let get = depot.get(AUTHORIZATION.as_str());
    //    let token : _ = get.unwrap();
}
