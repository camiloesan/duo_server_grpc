use mysql::{PooledConn, Pool};

pub fn get_connection() -> PooledConn {
    let url = "mysql://duo_user:apple@localhost:3306/duo";
    let pool = Pool::new(url).expect("wrong");
    let conn = pool.get_conn().expect("wrong"); 
    conn
}