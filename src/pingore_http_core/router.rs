use crate::async_core::asyncfn::CallbackHolder;

pub struct AppRouter {
    routes: Vec<route>,
}

impl AppRouter {
    pub fn new() -> AppRouter {
        AppRouter {
            routes: Vec::new(),
        }
    }

   pub  fn add_route(&mut self, route: route) {
        let path = route.path.clone();

        if self.get_route(path).is_some() {
            panic!("Route already exists");
        }

        self.routes.push(route);
    }

    pub fn get_route (&self, path: String) -> Option<&route> {
        for route in self.routes.iter() {
            if route.path == path {
                return Some(route);
            }
        }
        None
    }


}

pub struct route  {
    pub path: String,
    pub get: Option<CallbackHolder>,
    pub post: Option<CallbackHolder>,
    pub put: Option<CallbackHolder>,
    pub delete: Option<CallbackHolder>,
}
impl route {

    pub fn new(path: String) -> route {

        if path.is_empty() || !path.starts_with("/") {
            panic!("Path cannot be empty");
        }

        route {
            path,
            get: None,
            post: None,
            put: None,
            delete: None,
        }
    }

   pub fn get(&mut self, callback: CallbackHolder) {
       self.get = Some(callback);
   }
    pub fn post(&mut self, callback: CallbackHolder) {
         self.post = Some(callback);
    }
    pub fn put(&mut self, callback: CallbackHolder) {
         self.put = Some(callback);
    }
    pub fn delete(&mut self, callback: CallbackHolder) {
         self.delete = Some(callback);
    }
}


