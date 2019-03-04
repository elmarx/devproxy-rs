use http::Uri;
use std::collections::HashMap;

pub struct Mapper<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> Mapper<'a> {
    pub fn new(map: HashMap<&'a str, &'a str>) -> Mapper<'a> {
        Mapper { map }
    }

    pub fn uri(&self, host: &str, path: &str) -> Uri {
        let path_and_query = format!("/{}", path);

        let authority = match self.map.get(host) {
            Some(x) => x,
            None => host,
        };

        Uri::builder()
            .scheme("http")
            .authority(authority)
            .path_and_query(path_and_query.as_str())
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::mapper::Mapper;
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn no_mapping() {
        let subject = Mapper::new(HashMap::new());
        assert_eq!(
            subject.uri("example.com", "echo"),
            "http://example.com/echo"
        )
    }

    #[test]
    fn map_example_to_localhost() {
        let map = hashmap! {
            "example.com" => "localhost"
        };
        let subject = Mapper::new(map);

        assert_eq!(subject.uri("example.com", "demo"), "http://localhost/demo")
    }
}
