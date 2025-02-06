-- Create blogs table
CREATE TABLE IF NOT EXISTS blogs (
    id BIGSERIAL PRIMARY KEY,
    image TEXT,
    author TEXT NOT NULL,
    date TEXT NOT NULL,
    likes BIGINT NOT NULL DEFAULT 0,
    bookmarks INT NOT NULL DEFAULT 0
);

-- Create texts table with a one-to-one relationship to blogs
CREATE TABLE IF NOT EXISTS texts (
    blog_id BIGINT PRIMARY KEY REFERENCES blogs(id) ON DELETE CASCADE,
    text TEXT NOT NULL
);

-- Create comments table with a foreign key to blogs
CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    blog_id BIGINT NOT NULL REFERENCES blogs(id) ON DELETE CASCADE,
    author TEXT NOT NULL,
    text TEXT NOT NULL,
    likes INT NOT NULL DEFAULT 0,
    date TEXT NOT NULL
);
