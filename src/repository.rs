use std::collections::HashMap;
use crate::shortener::Shortener;

pub struct Repository {
    urls: HashMap<String, String>,
    shortener: Shortener,
}

// impl<'a> FromRequest<'a> for State<RwLock<Repository>> {
//     type Error = ();

//     fn from_request(request: &'a Request<'a>) -> request::Outcome<Self, Self::Error> {
//         let state = request.guard::<State<RwLock<Repository>>>()?;
//         Outcome::Success(state)
//     }
// }


impl Repository {
    pub fn new() -> Repository {
        Repository {
            urls: HashMap::new(),
            shortener: Shortener::new(),
        }
    }

    pub fn store(&mut self, url: &str) -> String {
        let id = self.shortener.next_id();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }

    pub fn lookup(&self, id: &str) -> Option<&String> {
        self.urls.get(id)
    }
}