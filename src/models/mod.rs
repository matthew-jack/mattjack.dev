pub mod posts {
    use comrak::{markdown_to_html, ComrakOptions};
    use serde::Serialize;
    use std::io::Read;
    use std::fs;

    #[derive(Serialize)]
    pub struct PostContent {
        content: String,
    }

    #[derive(Serialize, Clone)]
    pub struct Post {
        year: String,
        month: String,
        day: String,
        slug: String,
        link_text: String,
    }

    #[derive(Serialize)]
    pub struct PostIndex {
        post_list: Vec<Post>,
    }

    pub fn post_index() -> PostIndex {
        let mut index = PostIndex { post_list: vec![] };
        for path in fs::read_dir("posts/").unwrap() {
            let filename = path.unwrap().path();
            let filename = filename.file_stem().unwrap();
            let filename = filename.to_str().unwrap();
            let split_name: Vec<&str> = filename.split('-').collect();
            let post_title: Vec<&str> = split_name[3].split('_').collect();
            let post_title = post_title.join(" ");
            let mut filename = String::new();
            filename.push_str(split_name[1]);
            filename.push_str("/");
            filename.push_str(split_name[2]);
            filename.push_str("/");
            filename.push_str(split_name[0]);
            filename.push_str(" — ");
            filename.push_str(&post_title);
            let post = Post {
                year: String::from(split_name[0]),
                month: String::from(split_name[1]),
                day: String::from(split_name[2]),
                slug: String::from(split_name[3]),
                link_text: String::from(filename),
            };
            index.post_list.push(post);
        }
        sort_index(&mut index);
        index
    }

    fn sort_index(post_index: &mut PostIndex) -> () {
        let length = post_index.post_list.len() - 1;
        let mut i: usize = length;
        for _ in 0..length {
            while (i > 0) && (post_index.post_list[i].year.parse::<usize>().unwrap() > post_index.post_list[i-1].year.parse::<usize>().unwrap()) {
                let tmp = post_index.post_list[i-1].clone();
                post_index.post_list[i-1] = post_index.post_list[i].clone();
                post_index.post_list[i] = tmp;
                i -= 1;
            }
        }

        i = length;
        for _ in 0..length {
            while (i > 0) && (post_index.post_list[i].month.parse::<usize>().unwrap() > post_index.post_list[i-1].month.parse::<usize>().unwrap()) {
                if post_index.post_list[i].year.parse::<usize>().unwrap() >= post_index.post_list[i-1].year.parse::<usize>().unwrap() {
                    let tmp = post_index.post_list[i-1].clone();
                    post_index.post_list[i-1] = post_index.post_list[i].clone();
                    post_index.post_list[i] = tmp;
                }
                i -= 1;
            }
        }

        i = length;
        for _ in 0..length {
            while (i > 0) && (post_index.post_list[i].day.parse::<usize>().unwrap() > post_index.post_list[i-1].day.parse::<usize>().unwrap()) {
                if post_index.post_list[i].year.parse::<usize>().unwrap() >= post_index.post_list[i-1].year.parse::<usize>().unwrap() {
                    if post_index.post_list[i].month.parse::<usize>().unwrap() >= post_index.post_list[i-1].month.parse::<usize>().unwrap() {
                        let tmp = post_index.post_list[i-1].clone();
                        post_index.post_list[i-1] = post_index.post_list[i].clone();
                        post_index.post_list[i] = tmp;
                    }
                }
                i -= 1;
            }
        }
        ()
    }

    pub fn year_index(year: String) -> PostIndex {
        let mut year_index = PostIndex { post_list: vec![] };
        let post_index = post_index();
        for post in post_index.post_list {
            if post.year == year {
                year_index.post_list.push(post);
            }
        }
        year_index
    }

    pub fn month_index(year: String, month: String) -> PostIndex {
        let mut month_index = PostIndex { post_list: vec![] };
        let post_index = post_index();
        for post in post_index.post_list {
            if post.year == year {
                if post.month == month {
                    month_index.post_list.push(post);
                }
            }
        }
        month_index
    }

    pub fn day_index(year: String, month: String, day: String) -> PostIndex {
        let mut day_index = PostIndex { post_list: vec![] };
        let post_index = post_index();
        for post in post_index.post_list {
            if post.year == year {
                if post.month == month {
                    if post.day == day {
                        day_index.post_list.push(post);
                    }
                }
            }
        }
        day_index
    }

    pub fn parse_markdown(filename: String) -> PostContent {
        let mut f = fs::File::open(filename).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        let markdown = String::from(markdown_to_html(buffer.as_str(), &ComrakOptions::default()));
        PostContent { content: markdown }
    }
}
