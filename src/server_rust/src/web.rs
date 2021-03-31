use rocket_contrib::serve::StaticFiles;

pub fn routes() -> Vec<rocket::Route> {
    StaticFiles::from("./site/").into()
}
