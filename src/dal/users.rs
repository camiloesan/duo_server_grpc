use crate::dal::data_access;
use crate::duo::UserDetails;
use mysql::{params, prelude::Queryable, Row};

pub fn add_user(username: String, email: String, password: String) -> bool {
    let query = "INSERT INTO users (username, email, password) 
    VALUES (:username, :email, SHA2(:password, 256))";

    let mut conn = data_access::get_connection();
    let result = conn.exec_iter(
        query, 
        params! { "username" => username, "email" => email, "password" => password }
    )
    .expect("failed inserting a user")
    .affected_rows();

    result == 1
}

pub fn get_user_by_credentials(username: String, password: String) -> UserDetails {
    let query = "SELECT id, username, email, total_wins, picture_id 
        FROM users WHERE username = :username AND password = SHA2(:password, 256)";

    let mut user_details = UserDetails::default();
    let mut conn = data_access::get_connection();
    conn.exec_map(
        query, 
        params! { "username" => username, "password" => password },
        |mut row: Row| {
            user_details = UserDetails {
                id: row.take("id").unwrap(),
                username: row.take("username").unwrap(),
                email: row.take("email").unwrap(),
                total_wins: row.take("total_wins").unwrap(),
                picture_id: row.take("picture_id").unwrap(),
            }
        },
    )
    .expect("failed getting user information");

    user_details
}

pub fn is_username_available(username: String) -> bool {
    let query = "SELECT COUNT(*) as total 
        FROM users WHERE username = :username";

    let mut result = 0;
    let mut conn = data_access::get_connection();
    conn.exec_map(
        query, 
        params! { "username" => username },
        |mut row: Row| {
            result = row.take("total").unwrap();
        }
    )
    .expect("failed getting user");

    result == 0
}

pub fn is_email_available(email: String) -> bool {
    let query = "SELECT COUNT(*) as total 
        FROM users WHERE email = :email";
    
    let mut result = 0;
    let mut conn = data_access::get_connection();
    conn.exec_map(
        query, 
        params! { "email" => email }, 
        |mut row: Row| {
            result = row.take("total").unwrap();
        }
    )
    .expect("failed getting email");

    result == 0
}

fn _delete_user_by_username(username: String) {
    let query = "DELETE FROM users WHERE username = :username";

    let mut conn = data_access::get_connection();
    conn.exec_iter(
        query, 
        params! { "username" => username }
    )
    .expect("failure deleting a user")
    .affected_rows();
}

#[cfg(test)]
mod tests {
    use crate::dal::users::{add_user, _delete_user_by_username, get_user_by_credentials, is_username_available, is_email_available};

    #[test]
    fn add_user_works_test() {
        let username = String::from("camilo");
        let email = String::from("camiloesan@gmail.com");
        let password = String::from("allison");

        let result = add_user(username.clone(), email, password);
        let expected = true;
        _delete_user_by_username(username);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_user_by_credentials_test() {
        let username = String::from("pau");
        let email = String::from("pau@gmail.com");
        let password = String::from("allison");
        add_user(username.clone(), email, password.clone());

        let result = get_user_by_credentials(username.clone(), password);
        let id = result.id.clone();

        _delete_user_by_username(username);
        assert_ne!(id, 0);
    }

    #[test]
    fn is_username_available_test() {
        let result = is_username_available(String::from("value"));
        assert_eq!(result, true);
    }

    #[test]
    fn is_username_not_available_test() {
        let result = is_username_available(String::from("x"));
        assert_eq!(result, false);
    }

    #[test]
    fn is_email_available_test() {
        let result = is_email_available(String::from("value"));
        assert_eq!(result, true);
    }

    #[test]
    fn is_email_not_available_test() {
        let result = is_email_available(String::from("x"));
        assert_eq!(result, false);
    }
}