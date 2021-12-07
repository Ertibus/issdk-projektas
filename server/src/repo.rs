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


pub fn register_user(conn: Connection, data: SlimUser) -> Result<String, String> {
    match conn.query_row("SELECT username FROM user WHERE username=$1", &[&data.username], |_| {Ok(" ")}) {
        Ok(_) => { return Err(format!("User '{}' already exists", &data.username)) }
        Err(_) => {  }
    };

    match conn.execute(
        "INSERT INTO user (username, password, is_admin) VALUES ($0, $1, $2)",
        &[data.username, data.password, "0".to_string()]
    ) {
        Ok(_) => Ok("".to_string()),
        Err(err) => Err(format!("Failed to insert user {:?}", err.to_string())),
    }
}
