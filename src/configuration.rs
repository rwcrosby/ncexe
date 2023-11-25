#[allow(dead_code)]
#[derive(Debug)]
pub struct Configuration<'a> {
    theme: &'a str,
}

impl<'a> Configuration<'a> {
    pub fn new() -> Box<Configuration<'a>> {
        Box::new(Configuration{theme: "Something"})
    }
}
