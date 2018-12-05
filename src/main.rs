
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
    pub fn new() -> Download {
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

    // TODO: this function should belong to a different module, like a url module
    fn is_valid_url(url: &str) -> bool {
        if url.starts_with("http://") ||
            url.starts_with("https://") ||
            url.starts_with("http2://") {
                return true;
        }
        else {
            return false;
        }
    }

    pub fn set_url(&mut self, url: &str) -> Option<bool>{
        if url.len() < 128 && Download::is_valid_url(url) {
            self.url.push_str(url);
            return Some(true);
        } else {
            return None;
        }
    }

    pub fn get_url(&self) -> Option<String> {
        if !self.url.isEmpty() {
            return Some(self.url);
        } else {
            return None;
        }
    }

    // TODO: this function should belong to a different module
    fn is_valid_playlist(url: &String) -> bool{

    }

    pub fn set_playlist(&mut self, pl: bool){
        match self.get_url() {
            Some(url) => {

            }
            None => panic!("Downloader: no url assigned.");
        }
    }

    pub fn add_title(&mut self, title: &str){
        assert!(title.len() < 32);
        //self.title.push_str(title);
    }

    pub fn add_format(&mut self, format: &str){
        // TODO: add formats to vector
    }

    pub fn set_artist(&mut self, artist: &str){
        assert!(artist.len() < 32);
        self.artist.push_str(artist);
    }

    pub fn set_genre(&mut self, genre: &str){
        assert!(genre.len() < 32);
        self.genre.push_str(genre);
    }

    pub fn set_year(&mut self, year: u64){
        // TODO: should accept some kind of tuple or option of a string or an int
    }
}

struct Retspan {
    dls: Vec<Download>,
}

fn main() {
    println!("===== Welcome to ¡RetspaN! =====");

    let mut dl = Download::new();
    match dl.set_url("https://github.com/rg3/youtube-dl") {
        Some(result) => {
            assert!(result == true);
            println!("Donwload: {} is valid!", dl.url);
        }
        None => {
            println!("Download: invalid url!");
        }
    }
}
