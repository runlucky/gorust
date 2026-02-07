

pub struct User {
    pub name: String,
    pub age: u32,
}



impl User {
    pub fn print(&self) {
        println!("名前: {}, 年齢: {}", self.name, self.age);
    }
}

