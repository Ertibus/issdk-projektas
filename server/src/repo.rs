use crate::models::Article;
use crate::models::User;
use crate::models::SlimUser;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;


pub fn get_user(conn: Connection, username: String) -> Result<SlimUser, String> {
    match conn.query_row("SELECT username, password FROM user WHERE username=$1", &[&username], |row| {
        Ok(SlimUser{
            username: row.get(0)?,
            password: row.get(1)?,
        })
    }) {
        Ok(user) => Ok(user),
        Err(_) => Err(format!("User '{}' was not found", &username))
    }
}


pub fn del_user(conn: Connection, id: i32) -> Result<(), String> {
    conn.execute("DELETE FROM user WHERE id=$1", &[&id]).unwrap();
    Ok(())
}

pub fn promote_user(conn: Connection, id: i32) -> Result<(), String> {
    conn.execute("UPDATE user SET is_admin=1 WHERE id=$1", &[&id]).unwrap();
    Ok(())
}

pub fn demote_user(conn: Connection, id: i32) -> Result<(), String> {
    conn.execute("UPDATE user SET is_admin=0 WHERE id=$1", &[&id]).unwrap();
    Ok(())
}
pub fn get_users(conn: Connection, ) -> Result<Vec<User>, String> {
    let mut stmt = conn.prepare("SELECT id, username, is_admin FROM user").unwrap();
    let results = stmt.query_map([], |row| {
        Ok(User{
            id: row.get(0)?,
            username: row.get(1)?,
            password: "#foo".to_string(),
            is_admin: row.get(2)?,
        })
    });

    let mut retval: Vec<User> = Vec::new();
    for row in results.unwrap() {
        retval.push(row.unwrap());
    }
    Ok(retval)
}

pub fn register_user(conn: Connection, data: SlimUser) -> Result<String, String> {
    match conn.query_row("SELECT username FROM user WHERE username=$1", &[&data.username], |_| {Ok(" ")}) {
        Ok(_) => { return Err(format!("User '{}' already exists", &data.username)) }
        Err(_) => {  }
    };

    match conn.execute(
        "INSERT INTO user (username, password, is_admin) VALUES ($0, $1, $2)",
        &[&data.username, &data.password, &"0".to_string()]
    ) {
        Ok(_) => Ok("".to_string()),
        Err(err) => Err(format!("Failed to insert user {:?}", err.to_string())),
    }
}

pub fn check_permissions(conn: Connection, username: String) -> Result<bool, String> {
    match conn.query_row("SELECT is_admin FROM user WHERE username=$1", &[&username], |row| {
        Ok(row.get(0)?)
    }) {
        Ok(admin) => Ok(admin),
        Err(_) => Err(format!("User '{}' was not found", &username))
    }
}


pub fn get_all_articles(conn: Connection, ) -> Result<Vec<Article>, String> {
    let mut stmt = conn.prepare("SELECT * FROM article").unwrap();
    let results = stmt.query_map([], |row| {
        Ok(Article{
            id: row.get(0)?,
            owner: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
        })
    });

    let mut retval: Vec<Article> = Vec::new();
    for row in results.unwrap() {
        retval.push(row.unwrap());
    }
    Ok(retval)
}

pub fn get_articles(conn: Connection, id: String ) -> Result<Vec<Article>, String> {
    let mut stmt = conn.prepare("SELECT * FROM article WHERE owner=$1").unwrap();
    let results = stmt.query_map(&[&id], |row| {
        Ok(Article{
            id: row.get(0)?,
            owner: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
        })
    });

    let mut retval: Vec<Article> = Vec::new();
    for row in results.unwrap() {
        retval.push(row.unwrap());
    }
    Ok(retval)
}


pub fn get_article(conn: Connection, id: i32) -> Result<Article, String> {
    match conn.query_row("SELECT * FROM article WHERE id=$1", &[&id], |row| {
        Ok(Article{
            id: row.get(0)?,
            owner: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
        })
    }) {
        Ok(res) => Ok(res),
        Err(_) => Err(format!("Article '{}' was not found", &id))
    }
}

pub fn post_article(conn: Connection, data: Article) -> Result<String, String> {
    if data.id != -1 {
        match conn.query_row("SELECT * FROM article WHERE id=$1", &[&data.id], |_| {Ok(" ")}) {
            Ok(_) => {
                conn.execute("UPDATE article SET title=$1, description=$2 WHERE id=$3", &[&data.title, &data.description, &data.id.to_string()]).unwrap();
                return Ok("".to_string())
            }
            Err(_) => {  }
        };
    }

    match conn.execute(
        "INSERT INTO article (owner, title, description) VALUES ($0, $1, $2)",
        &[&data.owner, &data.title, &data.description]
    ) {
        Ok(_) => Ok("".to_string()),
        Err(err) => Err(format!("Failed to insert article {:?}", err.to_string())),
    }
}

pub fn del_article(conn: Connection, id: i32) -> Result<(), String> {
    conn.execute("DELETE FROM article WHERE id=$1", &[&id]).unwrap();
    Ok(())
}
