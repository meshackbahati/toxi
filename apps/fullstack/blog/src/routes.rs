//! Route table. Controllers stay in `controllers/`, wiring lives here.

use toxi::prelude::*;

use crate::controllers;

/// Register every route on the application router.
pub fn register(router: &mut Router) {
    router.get("/", controllers::pages::index);
    router.get("/posts/:id", controllers::pages::show);
    router.post("/auth/register", controllers::write::register);
    router.post("/auth/login", controllers::write::login);
    router.post("/posts", controllers::write::publish);
    router.get("/write", controllers::write::form);
    router.post("/write", controllers::write::submit);
    router.get("/api/status", controllers::status::api_status);
    router.get("/health", controllers::status::health_check);
    // Static assets last: specific routes match first, everything else
    // falls through to the template engine file server.
    router.get("/*", toxi_template::serve_static);
}
