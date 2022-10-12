use std::collections::HashMap;
use log::{error};
use tera::Tera;
use tera::Context;
use glob::Pattern;

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("assets/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                error!("Tera template parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

pub struct Route {
    name: String,
    pattern: String,
}

impl Route {
    pub fn new(name: &str, pattern: &str) -> Route {
        Route {
            name: String::from(name),
            pattern: String::from(pattern)
        }
    }
}

pub struct Router {
    routes: HashMap<String, Pattern>
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Router {
        let mut router = Router {
            routes: HashMap::new()
        };

        for r in &routes {
            router.routes.insert(r.name.to_string(), Pattern::new(&r.pattern).unwrap());
        }

        router
    }

    pub fn route(&self, route: &str) -> Result<&str, &'static str> {
        for (name, pattern) in &self.routes {
            if pattern.matches(route) {
                return Ok(name)
            }
        }

        return Err("Not found")
    }
}

pub fn html_from_template(template: &str, context: Context) -> String {
    TEMPLATES.render(&template, &context).unwrap()
}
