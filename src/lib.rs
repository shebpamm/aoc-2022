pub mod puzzle {

    use reqwest::{cookie::Jar, Url};
    use std::path::Path;

    use std::error::Error;
    use std::fs::{self, File};
    use std::io;
    use std::io::prelude::*;

    use std::sync::Arc;

    pub fn get_puzzle_input(day: i32) -> Result<io::BufReader<File>, Box<dyn Error>> {
        let path_string = format!("inputs/{day}/input.txt");
        let path = Path::new(&path_string);

        let prefix = path.parent().unwrap();

        if !prefix.is_dir() {
            fs::create_dir_all(prefix).unwrap();
        };

        if !path.is_file() {
            // Download inputs, save to file, return
            let data = download_puzzle_input(day)?;

            let mut file = fs::File::create(&path)?;
            file.write_all(data.as_bytes())?;
        }

        let file = fs::File::open(&path)?;
        Ok(io::BufReader::new(file))
    }

    fn get_session_cookie() -> Result<String, Box<dyn Error>> {
        let cookie: String = fs::read_to_string(".session")?.parse()?;
        return Ok(format!("session={cookie}"));
    }

    fn download_puzzle_input(day: i32) -> Result<String, Box<dyn Error>> {
        let cookie = get_session_cookie()?;
        let url = format!("https://adventofcode.com/2022/day/{day}/input")
            .parse::<Url>()
            .unwrap();

        let jar = Arc::new(Jar::default());

        jar.add_cookie_str(&cookie, &url);

        let client = reqwest::blocking::Client::builder()
            .cookie_provider(jar)
            .build()?;

        let resp = client.get(url).send()?;
        let data = resp.text()?;

        Ok(data)
    }
}
