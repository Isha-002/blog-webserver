use std::time::Duration;

use crate::{
    error::Error,
    types::{blog::{Blog, BlogID, NewBlog, Pagination, Text}, comment::{Comment, NewComment}},
};
use sqlx::Row;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool,
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("coundln't establish a database connection: {e:?}"),
        };
        Store {
            connection: db_pool,
        }
    }

    pub async fn blogs(&self, page: Pagination) -> Result<Vec<Blog>, Error> {
        let total_items = match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM blogs;")
            .fetch_one(&self.connection)
            .await
        {
            Ok(t) => t,
            Err(e) => return Err(Error::db_query_error(e)),
        };
        let pagination = match page.calculate_items(total_items) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        match sqlx::query("SELECT * from blogs LIMIT $1 OFFSET $2")
            .bind(pagination.1)
            .bind(pagination.0)
            .map(|row: PgRow| Blog {
                id: BlogID(row.get("id")),
                image: row.get("image"),
                author: row.get("author"),
                date: row.get("date"),
                likes: row.get("likes"),
                bookmarks: row.get("bookmarks"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(blogs) => Ok(blogs),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    pub async fn get_single_blog(&self, blog_id: i64) -> Result<Blog, Error> {
        match sqlx::query("SELECT * from blogs WHERE id = $1")
            .bind(blog_id)
            .map(|row: PgRow| Blog {
                id: BlogID(row.get("id")),
                image: row.get("image"),
                author: row.get("author"),
                date: row.get("date"),
                likes: row.get("likes"),
                bookmarks: row.get("bookmarks"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(blog) => Ok(blog),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    pub async fn post_blog(&self, blog: NewBlog) -> Result<Blog, Error> {
        let blog_row = sqlx::query(
            "INSERT INTO blogs (image, author, likes, bookmarks) 
            VALUES ($1, $2, 0, 0) 
            RETURNING id, image, author, date, likes, bookmarks",
        )
        .bind(blog.image)
        .bind(blog.author)
        .fetch_one(&self.connection)
        .await
        .map_err(Error::db_query_error)?;

        let blog_id: i64 = blog_row.get("id");
        if !blog.text.is_empty() {
            sqlx::query("INSERT INTO texts (blog_id, text) VALUES ($1, $2)")
                .bind(blog_id)
                .bind(blog.text)
                .execute(&self.connection)
                .await
                .map_err(Error::db_query_error)?;
        }

        Ok(Blog {
            id: BlogID(blog_id),
            image: blog_row.get("image"),
            author: blog_row.get("author"),
            date: blog_row.get("date"),
            likes: blog_row.get("likes"),
            bookmarks: blog_row.get("bookmarks")
        })
    }

    pub async fn put_blog(&self, blog: Blog, blog_id: i64) -> Result<Blog, Error> {
        match sqlx::query(
            "UPDATE blogs
            SET image = $1, author = $2, date = NOW(), likes = $3, bookmarks = $4
            WHERE id = $5
            RETURNING *"
        )
        .bind(blog.author)
        .bind(blog.image)
        .bind(blog.likes)
        .bind(blog.bookmarks)
        .bind(blog_id)
        .map(|row: PgRow| Blog {
            id: BlogID(blog_id),
            image: row.get("image"),
            author: row.get("author"),
            date: row.get("date"),
            likes: row.get("likes"),
            bookmarks: row.get("bookmarks")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(blog) => Ok(blog),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    pub async fn delete_blog(&self, blog_id: i64) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM blogs WHERE id = $1")
            .bind(blog_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    pub async fn blog_text(&self, blog_id: i64) -> Result<Text, Error> {
        match sqlx::query(
            "SELECT * FROM texts
            WHERE blog_id = $1"
        )
        .bind(blog_id)
        .map(|row: PgRow| Text {
            blog_id,
            text: row.get("text")
        })
        .fetch_one(&self.connection)
        .await {
            Ok(text) => Ok(text),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    pub async fn put_blog_text(&self, text: Text, blog_id: i64) -> Result<Text, Error> {
        match sqlx::query(
            "UPDATE texts 
            SET text = $1
            WHERE blog_id = $2"
        )
        .bind(text.text)
        .bind(blog_id)
        .map(|row: PgRow| Text {
            blog_id,
            text: row.get("text")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(text) => Ok(text),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    // you can post blog text directly using post_blog handler this is just in case if you get silly :P
    pub async fn post_blog_text(&self, text: Text, blog_id: i64) -> Result<Text, Error> {
        match sqlx::query(
            "INSERT INTO texts (blog_id, text) VALUES ($1, $2)
            RETURNING *"
        )
        .bind(blog_id)
        .bind(text.text)
        .map({|row: PgRow| Text {
            blog_id,
            text: row.get("text")
        }})
        .fetch_one(&self.connection)
        .await
        {
            Ok(text) => Ok(text),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    pub async fn get_blog_comments(&self, blog_id: i64) -> Result<Vec<Comment>, Error> {
        match sqlx::query(
            "SELECT * FROM comments
            WHERE blog_id = $1")
            .bind(blog_id)
            .map(|row: PgRow| Comment {
                id: row.get("id"),
                blog_id,
                author: row.get("author"),
                text: row.get("text"),
                likes: row.get("likes"),
                date: row.get("date")
            })
            .fetch_all(&self.connection)
            .await
            {
                Ok(comments) => Ok(comments),
                Err(e) => Err(Error::db_query_error(e)),
            }
    }

    pub async fn post_blog_comments(&self, comment: NewComment , blog_id: i64) -> Result<Comment, Error> {
        match sqlx::query(
        "INSERT INTO comments 
        (blog_id, author, text, likes)
        VALUES ($1, $2, $3, 0)
        RETURNING *")
        .bind(blog_id)
        .bind(comment.author)
        .bind(comment.text)
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            blog_id,
            author: row.get("author"),
            text: row.get("text"),
            likes: row.get("likes"),
            date: row.get("date")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(comment) => Ok(comment),
            Err(e) => Err(Error::db_query_error(e)),
        }
    }

    pub async fn delete_blog_comment(&self, blog_id: i64, comment_id: i64) -> Result<bool, Error> {
        match sqlx::query(
            "DELETE FROM comments 
            WHERE id = $1 AND blog_id = $2")
            .bind(comment_id)
            .bind(blog_id)
            .execute(&self.connection)
            .await
            {
                Ok(_) => Ok(true),
                Err(e) => Err(Error::db_query_error(e)),
            }
    }

}
