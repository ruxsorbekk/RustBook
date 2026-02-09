struct AppModel {
    counter: u8,
}

enum AppMsg {
    Increment, 
    Decrement,
}

enum Inbox {
    GetEmail(Email),
}

fn update(&mut self, message: Self::Input, ...) {
    match message {
        Inbox::GetEmail(email) => self.emails.push(email)
    }
}


fn main() {
    println!("Hello, world!");
}