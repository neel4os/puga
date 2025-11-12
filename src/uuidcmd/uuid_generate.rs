use uuid::Uuid;
pub fn generate_uuid(with_hyphen: bool, version: u8) -> String {
    let uuid = match version {
        4 => Uuid::new_v4().to_string(),
        7 => Uuid::now_v7().to_string(),
        _ => panic!("requested version is not 4 or 7"),
    };
    if with_hyphen {
        return uuid.replace("-", "");
    }
    return uuid;
}
