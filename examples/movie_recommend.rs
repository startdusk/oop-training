// 设计一个电影推荐系统

use std::collections::HashMap;

/// Movie 电影
#[derive(Debug, Clone)]
pub struct Movie {
    id: u32,
    title: String,
}

impl Movie {
    pub fn new(id: u32, title: String) -> Self {
        Self { id, title }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}

/// User 用户，用户对电影进行评分
#[derive(Debug, Clone)]
pub struct User {
    id: u32,
    name: String,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
pub enum MovieRating {
    NotRated,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl MovieRating {
    pub fn to_num(&self) -> usize {
        match self {
            Self::NotRated => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }
}

type UserId = u32;
type MovieId = u32;

/// RatingRegister 评分注册器
pub struct RatingRegister {
    // 每个用户对应他看过的电影列表
    user_movies: HashMap<UserId, Vec<Movie>>,
    // 每部电影的评分, 对应每个用户对它的评分
    movie_ratings: HashMap<MovieId, HashMap<UserId, MovieRating>>,

    movies: Vec<Movie>,
    users: Vec<User>,
}

impl RatingRegister {
    pub fn new() -> Self {
        Self {
            user_movies: HashMap::new(),
            movie_ratings: HashMap::new(),
            movies: Vec::new(),
            users: Vec::new(),
        }
    }

    pub fn add_rating(&mut self, user: &User, movie: &Movie, rating: MovieRating) {
        // 如果电影不存在，则添加该影片
        if !self.movie_ratings.contains_key(&movie.get_id()) {
            self.movie_ratings.insert(movie.get_id(), HashMap::new());
            self.movies.push(movie.clone());
        }

        // 如果用户不存在，则添加用户
        if !self.user_movies.contains_key(&user.get_id()) {
            self.user_movies.insert(user.get_id(), Vec::new());
            self.users.push(user.clone());
        }

        // 添加用户看过该影片
        if let Some(user_movies) = self.user_movies.get_mut(&user.get_id()) {
            user_movies.push(movie.clone());
        }
        // 添加用户对该影片的评分
        if let Some(movie) = self.movie_ratings.get_mut(&movie.get_id()) {
            if let Some(movie_rating) = movie.get_mut(&user.get_id()) {
                *movie_rating = rating;
            } else {
                movie.insert(user.get_id(), rating);
            }
        }
    }

    // 获取该影片的平均评分
    pub fn get_average_rating(&self, movie: &Movie) -> f64 {
        if !self.movie_ratings.contains_key(&movie.get_id()) {
            return 0.00;
        }

        let mut sum: f64 = 0.00;
        let ratins = self.movie_ratings.get(&movie.get_id()).unwrap();
        for (_user_id, rating) in ratins.iter() {
            sum += rating.to_num() as f64;
        }
        sum / ratins.len() as f64
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn get_movies(&self) -> &Vec<Movie> {
        &self.movies
    }

    pub fn get_user_movies(&self, user: &User) -> Option<&Vec<Movie>> {
        let Some(movies) = self.user_movies.get(&user.get_id()) else {
            return None;
        };
        Some(movies)
    }

    pub fn get_movie_ratings(&self, movie: &Movie) -> Option<&HashMap<MovieId, MovieRating>> {
        let Some(movie_rating) = self.movie_ratings.get(&movie.get_id()) else {
            return None;
        };
        Some(movie_rating)
    }
}

/// MovieRecommendation 电影推荐 主体结构
pub struct MovieRecommendation {
    rating_register: RatingRegister,
}

impl MovieRecommendation {
    pub fn new(rating_register: RatingRegister) -> Self {
        Self { rating_register }
    }

    // 推荐电影
    pub fn recommend_movie(&self, user: &User) -> String {
        if self.rating_register.get_user_movies(user).is_none() {
            // 如果该用户没有看过任何电影
            self.recommend_movie_new_user()
        } else {
            // 如果该用户已经看过一些电影
            self.recommend_movie_existing_user(user)
        }
    }

    // 推荐高分影片给新用户
    fn recommend_movie_new_user(&self) -> String {
        let mut best_move = Movie {
            id: u32::MAX,
            title: "unknown".to_string(),
        };
        let mut best_rating: f64 = 0.0;
        for movie in self.rating_register.get_movies() {
            let rating = self.rating_register.get_average_rating(movie);
            if rating > best_rating {
                best_move = Movie {
                    id: movie.get_id(),
                    title: movie.get_title(),
                };
                best_rating = rating;
            }
        }

        best_move.get_title()
    }

    // 推荐别人打高分的，且用户感兴趣的电影
    fn recommend_movie_existing_user(&self, user: &User) -> String {
        let mut best_movie: Option<Movie> = None;
        let mut similarity_score = usize::MAX;

        for reviewer in self.rating_register.get_users() {
            if reviewer.get_id() == user.get_id() {
                continue;
            }

            let score = self.get_similarity_score(user, reviewer);
            if score < similarity_score {
                similarity_score = score;
                let movie = self.recommend_unwatched_movie(user, reviewer);
                best_movie = movie;
            }
        }

        if best_movie.is_none() {
            return "unknown".to_string();
        }
        best_movie.unwrap().get_title()
    }

    // 推荐用户还没看过高分的电影
    fn recommend_unwatched_movie(&self, user: &User, reviewer: &User) -> Option<Movie> {
        let mut best_movie: Option<Movie> = None;
        let mut best_rating: usize = 0;

        for movie in self.rating_register.get_user_movies(reviewer)?.iter() {
            let Some(rating) = self.rating_register.get_movie_ratings(movie) else {
                continue;
            };
            // 如果user没有看过这部电影，且reviewer给这部电影打了个高分，则推荐它
            if !rating.contains_key(&user.get_id())
                && rating.get(&reviewer.get_id()).unwrap().to_num() > best_rating
            {
                best_movie = Some(Movie {
                    id: movie.get_id(),
                    title: movie.get_title(),
                });
                best_rating = rating.get(&reviewer.get_id()).unwrap().to_num();
            }
        }
        best_movie
    }

    // 获取两个用户对电影相似的评分
    // 评分差的和越小，说明两个用户的品味越相近
    fn get_similarity_score(&self, user1: &User, user2: &User) -> usize {
        let mut score = usize::MAX;
        let user_movies = self.rating_register.get_user_movies(user2);
        if user_movies.is_none() {
            return score;
        }
        for movie in user_movies.unwrap().iter() {
            let Some(ratings) = self.rating_register.get_movie_ratings(movie) else {
                continue;
            };
            // 在user2打分的过的电影列表里 如果user1已经给该电影打分了，则添加打分差异
            // 评分差的和越小，说明两个用户的品味越相近
            if ratings.contains_key(&user1.get_id()) {
                if score == usize::MAX {
                    score = 0;
                }
                let user1_rating = ratings.get(&user1.get_id()).unwrap();
                let user2_rating = ratings.get(&user2.get_id()).unwrap();
                score += user1_rating.to_num().abs_diff(user2_rating.to_num());
            }
        }
        score
    }
}

fn main() {
    let user1 = User::new(1, "User1".to_string());
    let user2 = User::new(2, "User2".to_string());
    let user3 = User::new(3, "User3".to_string());

    let movie1 = Movie::new(1, "Batman Begins".to_string());
    let movie2 = Movie::new(2, "Liar Liar".to_string());
    let movie3 = Movie::new(3, "The Godfather".to_string());

    let mut ratings = RatingRegister::new();
    ratings.add_rating(&user1, &movie1, MovieRating::Five);
    ratings.add_rating(&user1, &movie2, MovieRating::Two);
    ratings.add_rating(&user2, &movie2, MovieRating::Two);
    ratings.add_rating(&user2, &movie3, MovieRating::Four);
    ratings.add_rating(&user3, &movie3, MovieRating::One);

    let recommender = MovieRecommendation::new(ratings);

    println!(
        "recommender user1 movie: {}",
        recommender.recommend_movie(&user1)
    ); // The Godfather
    println!(
        "recommender user2 movie: {}",
        recommender.recommend_movie(&user2)
    ); // Batman Begins
    println!(
        "recommender user3 movie: {}",
        recommender.recommend_movie(&user3)
    ); // Liar Liar
}
