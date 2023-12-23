#[derive(Debug)]
pub enum UserType {
    Free, 
    Basic,
    Pro,
    Premium
}

#[derive(Debug)]
pub struct User {
    pub name:String, 
    pub pass:String,
    pub mail:Option<String>,
    pub phone:Option<String>,
    pub address:Option<String>,
    pub user_type:UserType
}

pub struct UserBuilder {
    pub name:Option<String>, 
    pub pass:Option<String>, 
    pub mail:Option<String>,
    pub phone:Option<String>,
    pub address:Option<String>,
    pub user_type:UserType
}

impl UserBuilder {
    pub fn new() -> Self {
        UserBuilder { name: None, pass: None, mail: None, phone:None, address:None, user_type:UserType::Free }
    }

    pub fn name(mut self, n:&str) -> Self {
        self.name = Some(n.to_string());
        self
    }

    pub fn pass(mut self, p:&str) -> Self {
        self.pass = Some(p.to_string());
        self
    }

    pub fn mail(mut self, m:&str) -> Self {
        self.mail = Some(m.to_string());
        self
    }

    pub fn phone(mut self, p:&str) -> Self {
        self.phone = Some(p.to_string());
        self
    }

    pub fn address(mut self, a:&str) -> Self {
        self.address = Some(a.to_string());
        self
    }

    pub fn user_type(mut self, ut:UserType) -> Self {
        self.user_type = ut;
        self
    }

    pub fn build(self) -> User {
        User { name: self.name.expect("Name Required"), 
            pass: self.pass.expect("Password Required"), 
            mail: self.mail,
            phone: self.phone,
            address:self.address,
            user_type:self.user_type
        }
    }

}

impl User {
    pub fn change_subscription(&mut self, ut:UserType) {
        self.user_type = ut;
    }
}

