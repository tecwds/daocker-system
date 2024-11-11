
use lombok::{Getter, Setter};
use salvo::{handler, writing::Json, Request, Response};
use serde::Deserialize;
use tracing::info;

use crate::{entity::{result::ResultEntity, student::Student}, service::student_service::{del_student, get_list, post_student}};

#[handler]
pub async fn get_student(_: &mut Request, res: &mut Response) {
    
    let vec_stu: Vec<Student> = get_list().await.unwrap_or(Vec::new());

    if vec_stu.len() == 0 {
        res.render(Json(ResultEntity::<String>::error("未知错误")));
        return;
    }

    res.render(Json(ResultEntity::<Vec<Student>>::success(Some(vec_stu))));
}

#[handler]
pub async fn post_stu(req: &mut Request, res: &mut Response) {
    info!("test");
    let body = req.parse_json::<Student>().await;

    if body.is_err() {
        res.render(Json(ResultEntity::<String>::error("参数传递错误")));
        return;
    }

    let body = body.unwrap();
    let stu = post_student(body).await.unwrap();

    if stu.is_none() {
        res.render(Json(ResultEntity::<String>::error("未知错误")));
        return;
    }

    res.render(Json(ResultEntity::<Student>::success(Some(stu.unwrap()))));
}

#[derive(Debug, Getter, Setter, Deserialize)]
struct List {
    list: Vec<u64>,
}

#[handler]
pub async fn del_edu(req: &mut Request, res: &mut Response) {
    let list = req.parse_json::<List>().await;

    info!("test list -- {:?}", list);

    if list.is_err() {
        res.render(Json(ResultEntity::<String>::error("未知错误，请检查传递参数")));
        return;
    }

    let vec_stu = list.unwrap().get_list().clone();

    let mut re = 0_u32;
    for id in &vec_stu {
        del_student(*id).await.unwrap();
    }

    // if re as usize == vec_stu.len() {
    //     res.render(Json(ResultEntity::<String>::error("未知错误，请检查传递参数")));
    //     return;
    // }
    res.render(Json(ResultEntity::<String>::success(None)))
}