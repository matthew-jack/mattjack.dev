pub mod home {
    use rocket_contrib::templates::Template;

    use super::posts;

    #[get("/")]
    pub fn index() -> Template {
        posts::index()
    }

    #[get("/about")]
    pub fn about() -> Template {
        Template::render("about", &())
    }
}

pub mod posts {
    use rocket::response::Redirect;
    use rocket_contrib::templates::Template;
    use std::path::PathBuf;

    use models::posts;

    #[get("/")]
    pub fn index() -> Template {
        let index = posts::post_index();
        Template::render("home", &index)
    }

    #[get("/<year>/<month>/<day>/<short_title>")]
    pub fn get_post(year: String, month: String, day: String, short_title: String) -> Template {
        let mut filename = String::from("posts/");
        filename.push_str(&year);
        filename.push_str("-");
        filename.push_str(&month);
        filename.push_str("-");
        filename.push_str(&day);
        filename.push_str("-");
        filename.push_str(&short_title);
        let post_content = posts::parse_markdown(filename);
        Template::render("post", &post_content)
    }

    #[get("/<year>/<month>/<day>", rank = 2)]
    pub fn get_day(year: String, month: String, day: String) -> Template {
        let index = posts::day_index(year, month, day);
        Template::render("home", &index)
    }

    #[get("/<year>/<month>", rank = 3)]
    pub fn get_month(year: String, month: String) -> Template {
        let index = posts::month_index(year, month);
        Template::render("home", &index)
    }

    #[get("/<year>", rank = 4)]
    pub fn get_year(year: String) -> Template {
        let index = posts::year_index(year);
        Template::render("home", &index)
    }

    #[get("/<identifier..>", rank = 5)]
    pub fn get_misc(identifier: PathBuf) -> Redirect {
        println!("Got nonsense query {:?}", identifier);
        Redirect::to("/posts")
    }
}

pub mod assets {
    use rocket_contrib::templates::Template;

    #[get("/")]
    pub fn public_key() -> Template {
        Template::render("public_key", &())
    }
}
