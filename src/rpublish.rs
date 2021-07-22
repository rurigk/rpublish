pub struct RPublishApp
{
    pub shared: String
}

impl RPublishApp {
    pub fn test(&mut self) -> String
    {
        //self.shared.push_str("a");
        format!("{}", self.shared)
    }
}