#[derive(Debug, Clone)]
pub struct UserInfo {
    pub name: String,
}

impl UserInfo {
    pub fn new(name: String) -> Self {
        UserInfo {
            name
        }
    }
}

fn main() {
    let name = String::from("mhb");
    let user_info = UserInfo::new(name);
    println!("user_info: {:?}", user_info);

    let copied_user_info = user_info.clone();
    println!("copied_user_info: {:?}", copied_user_info);
    println!("user_info: {:?}", user_info);

    println!("Hello, world!");
}
