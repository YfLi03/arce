pub struct Picture{
    pub hash_old: String,
    pub hash_new: String,
    pub filename: PathBuf,

    pub photography: bool, 
    pub selected: bool,

    pub title: String,
    pub params: String,
    pub date: String,
    pub camera: String,

    pub article_linked: bool,
    pub article_link: String,
}