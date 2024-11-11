use tracing::info;

use crate::{entity::student::Student, setting::get_pool};

pub async fn get_student_by_id(student_id: String) -> Result<Student, Box<dyn std::error::Error>> {
    Ok(sqlx::query_as("select * from lb_user where student_id = ?").bind(student_id).fetch_one(&get_pool().await.unwrap()).await.unwrap())
}

pub async fn get_student_by_email(email: String) -> Result<Student, Box<dyn std::error::Error>> {
    Ok(sqlx::query_as("select * from lb_user where email = ?").bind(email).fetch_one(&get_pool().await.unwrap()).await.unwrap())
}

pub async fn get_list() -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    Ok(sqlx::query_as("select * from lb_user")
        .fetch_all(&get_pool().await.unwrap())
        .await
        .unwrap())
}

pub async fn del_student(id: u64) -> Result<u32, Box<dyn std::error::Error>> {
    let result = sqlx::query("delete from lb_user where id = ?")
        .bind(id)
        .execute(&get_pool().await.unwrap())
        .await;

    match result {
        Err(e) => {
            println!("err info {:?}", e);
            Ok(0)
        }
        Ok(res) => {
            println!("ok info {:?}", res);
            Ok(1)
        }
    }
}

pub async fn post_student(student: Student) -> Result<Option<Student>, Box<sqlx::Error>> {
    info!("post_student -- student value -- {:?}", student);
    if student.id == 0 {
        // new student
        let student = student.clone();
        let res = sqlx::query(
            "insert into lb_user (
                    student_id,
                    email,
                    full_name,
                    age,
                    gender,
                    college,
                    major,
                    grade,
                    class,
                    password
                ) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(student.student_id)
        .bind(student.email)
        .bind(student.full_name)
        .bind(student.age)
        .bind(student.gender)
        .bind(student.college)
        .bind(student.major)
        .bind(student.grade)
        .bind(student.class)
        .bind(student.password)
        .execute(&get_pool().await.unwrap())
        .await;
        
        info!("log -- test insert result -- {:?}", res);

        // return Ok(None);
    } else {
        let student = student.clone();
        let res = sqlx::query(
            "update lb_user set
                        student_id = ?,
                        email = ?,
                        password = ?,
                        full_name = ?,
                        age = ?,
                        gender = ?,
                        college = ?,
                        major = ?,
                        grade = ?,
                        class = ?    
                    where id = ?
            ",
        )
        .bind(student.student_id)
        .bind(student.email)
        .bind(student.password)
        .bind(student.full_name)
        .bind(student.age)
        .bind(student.gender)
        .bind(student.college)
        .bind(student.major)
        .bind(student.grade)
        .bind(student.class)
        .bind(student.id)
        .execute(&get_pool().await.unwrap())
        .await;

        match res {
            Ok(res) => {
                println!("ok info {:?}", res);
            }
            Err(e) => {
                println!("err info {:?}", e);
                return Ok(None)
            }
        }
    }

    let new_student: Result<Student, sqlx::Error> = sqlx::query_as("select * from lb_user where student_id = ?")
        .bind(student.student_id)
        .fetch_one(&get_pool().await.unwrap())
        .await;

    if let Err(e) = new_student {
        info!("err {}", e);
        return Ok(None);
    }

    let new_student = new_student.unwrap();

    Ok(Some(new_student))
    // update student
}

#[cfg(test)]
mod test {
    use crate::{entity::student::Student, setting::POOL};

    async fn test_insert_support() {
        let student: Vec<Student> = sqlx::query_as("select * from lb_user")
            .fetch_all(POOL.get_pool())
            .await
            .unwrap();

        println!("{student:?}");
    }

    #[test]
    fn test_insert() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            test_insert_support().await;
        });
    }
}
