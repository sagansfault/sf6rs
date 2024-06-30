use crate::framedata::load_all;

pub mod framedata;
pub mod character;

pub struct LazyLock<T, F = fn() -> T> {
    data: std::sync::OnceLock<T>,
    f: F,
}

impl<T, F> LazyLock<T, F> {
    pub const fn new(f: F) -> LazyLock<T, F> {
        Self {
            data: std::sync::OnceLock::new(),
            f,
        }
    }
}

impl<T> std::ops::Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.get_or_init(self.f)
    }
}

#[tokio::test]
async fn test() {
    let data = load_all().await;
    let x = data.find_character_frame_data(&character::MBISON).unwrap();
    println!("{:?}", data.find_move("mbison", "5lp"));
    println!("{:?}", x.gifs.iter().next().unwrap());
    println!("{:?}", x.moves.iter().next().unwrap());
}