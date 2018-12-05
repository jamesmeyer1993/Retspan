
struct Download {
    url: String,
    playlist: bool,
    titles: Vec<String>,
    artist: String,
    album: String,
    genre: String,
    year: u64,
    formats: Vec<String>,
}

impl Download {
    fn new() -> Download {
        let dl = Download {
            url: String::with_capacity(128),
            playlist: false,
            titles: vec![String::with_capacity(32)],
            artist: String::with_capacity(32),
            album: String::with_capacity(32),
            genre: String::with_capacity(32),
            year: 0,
            formats: vec![String::with_capacity(4)],
        };
        return dl;
    }

    fn set_url(&mut self, url: &str){
        assert!(url.len() < 128);
        self.url.push_str(url);
    }

    fn set_playlist(&mut self, pl: bool){
        // TODO: check the url to make sure this actually is a playlist
        self.playlist = pl;
    }

    fn add_title(&mut self, title: &str){
        // TODO: push title to vector
    }

    fn set_artist(&mut self, artist: &str){
        // TODO:
    }

    fn set_genre(&mut self, genre: &str){
        // TODO:
    }

    fn set_year(&mut self, year: u64){
        // TODO: should accept some kind of tuple or option of a string or an int
    }

    fn add_format(&mut self, format: &str){
        // TODO: add formats to vector
    }
}

struct Retspan {
    dls: Vec<Download>,
}

fn main() {
    println!("===== Welcome to ¡RetspaN! =====");

    let mut dl = Download::new();

}
