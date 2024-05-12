CREATE TABLE users(
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(300) NOT NULL UNIQUE,
    password VARCHAR(200) NOT NULL,
    email_token VARCHAR(300),
    name VARCHAR(300) NOT NULL,
    profile_picture VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE collections(
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(300) NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    CONSTRAINT fk_collections_users FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT uq_collections_users UNIQUE(name, user_id)
);

CREATE TABLE locations(
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(300) NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    CONSTRAINT fk_locations_users FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT uq_locations_users UNIQUE(name, user_id)
);

CREATE TABLE books(
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(500) NOT NULl,
    authors JSON NOT NULL,
    publisher VARCHAR(500) NOT NULL,
    languages JSON NOT NULL,
    edition VARCHAR(50),
    isbn VARCHAR(13),
    year VARCHAR(4),
    genres JSON,
    cover VARCHAR(500),
    collection_id BIGINT UNSIGNED,
    location_id BIGINT UNSIGNED NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    CONSTRAINT fk_books_collections FOREIGN KEY(collection_id) REFERENCES collections(id),
    CONSTRAINT fk_books_locations FOREIGN KEY(location_id) REFERENCES locations(id),
    CONSTRAINT fk_books_users FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);