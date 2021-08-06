-- user_status
CREATE TABLE site_user_status (
    id serial PRIMARY KEY,
    name VARCHAR(25) UNIQUE NOT NULL
);

INSERT INTO public.site_user_status(
	id, name)
	VALUES 
	(0, 'inactive')
	, (1, 'active')
	, (2, 'suspended')
	, (3, 'banned');

-- site_user
CREATE TABLE site_user (
    id serial PRIMARY KEY,
    site_user_status_id INT NOT NULL,
    username VARCHAR(64) UNIQUE NOT NULL,
    email VARCHAR(320) UNIQUE NOT NULL,
    password VARCHAR(60) NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    updated TIMESTAMPTZ NOT NULL,

    FOREIGN KEY(site_user_status_id)
        REFERENCES site_user_status(id)
);

-- post_group
CREATE TABLE post_group (
    id serial PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

-- post_category
CREATE TABLE post_category (
    id serial PRIMARY KEY,
    name VARCHAR(25) UNIQUE NOT NULL
);

INSERT INTO public.post_category(
	id, name)
	VALUES 
    (0, 'blog')
	, (1, 'git')
	, (2, 'contact');

-- post
CREATE TABLE post (
    id serial PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    label TEXT NOT NULL,
    post_group_id INT NOT NULL,
    post_category_id INT NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
    embedded BOOLEAN NOT NULL,
    created TIMESTAMPTZ NOT NULL, 
    updated TIMESTAMPTZ NOT NULL,

    FOREIGN KEY(post_group_id)
        REFERENCES post_group(id),
            
    FOREIGN KEY(post_category_id)
        REFERENCES post_category(id)
);

-- comment
CREATE TABLE comment (
    id serial PRIMARY KEY,
    content VARCHAR(500) NOT NULL,
    post_id INT NOT NULL,
    site_user_id INT NOT NULL,
    parent_comment_id INT,
    created TIMESTAMPTZ NOT NULL,

    FOREIGN KEY(post_id)
        REFERENCES post(id),

    FOREIGN KEY(site_user_id)
        REFERENCES site_user(id),

    FOREIGN KEY(parent_comment_id)
        REFERENCES comment(id)
);