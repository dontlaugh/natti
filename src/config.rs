/// Args models our command line arguments
pub struct Args {
    pub command: Option<String>,
    pub private_key: Option<String>,
    pub server: Option<String>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub exec: Option<String>,
}
