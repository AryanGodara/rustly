use std::sync::RwLock;

use crate::repository::Repository;

use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use std::ops::Deref;


pub struct AppState {
    pub repository: State<RwLock<Repository>>,
}

impl AppState {
    pub fn new(repository: State<RwLock<Repository>>) -> Self {
        Self { repository }
    }

    pub fn get_repository(&self) -> std::sync::RwLockReadGuard<Repository> {
        self.repository.read().unwrap()
    }

    pub fn get_repository_mut(&self) -> std::sync::RwLockWriteGuard<Repository> {
        self.repository.write().unwrap()
    }
}

impl<'a> FromRequest<'a> for &'a AppState {
    type Error = ();

    fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let state = request.guard::<State<AppState>>()?;
        Outcome::Success(state.deref())
    }
}