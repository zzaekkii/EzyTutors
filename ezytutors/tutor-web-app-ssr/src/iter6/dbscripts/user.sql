DROP TABLE IF EXISTS ezyweb_user;

CREATE TABLE ezyweb_user (
    user_id VARCHAR(20) PRIMARY KEY,
    tutor_id INT,
    user_password CHAR(100) NOT NULL
)