use clap::{App, app_from_crate};

pub fn build() -> App<'static> {
    app_from_crate!()
}
