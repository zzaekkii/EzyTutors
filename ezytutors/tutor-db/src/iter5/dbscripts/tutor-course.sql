/* 잔존 테이블 삭제 */
DROP TABLE IF EXISTS ezy_course_ch6 CASCADE;
DROP TABLE IF EXISTS ezy_tutor_ch6;

/* 테이블 생성 */
CREATE TABLE ezy_tutor_ch6 (
    tutor_id SERIAL PRIMARY KEY,
    tutor_name VARCHAR(200) NOT NULL,
    tutor_picture_url VARCHAR(200) NOT NULL,
    tutor_profile VARCHAR(2000) NOT NULL
);

CREATE TABLE ezy_course_ch6 (
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    course_description VARCHAR(2000),
    course_format VARCHAR(30),
    course_structure VARCHAR(200),
    course_duration VARCHAR(30),
    course_price INT,
    course_language VARCHAR(30),
    course_level VARCHAR(30),
    posted_time TIMESTAMP DEFAULT NOW(),

    CONSTRAINT fk_tutor
        FOREIGN KEY(tutor_id)
        REFERENCES ezy_tutor_ch6(tutor_id)
    ON DELETE CASCADE
);

GRANT ALL PRIVILEGES ON TABLE ezy_tutor_ch6 TO zzaekkii;
GRANT ALL PRIVILEGES ON TABLE ezy_course_ch6 TO zzaekkii;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA PUBLIC TO zzaekkii;

/* 테스팅 시드 */
INSERT INTO ezy_tutor_ch6 (tutor_id, tutor_name, tutor_picture_url, tutor_profile)
    VALUES (1, '김영한', 'https://cdn.inflearn.com/public/users/thumbnails/74366/4c5096a5-6579-4034-b780-b8d1a958055f?w=108?w=108', '진짜 실무에 필요한 제대로 된 개발자가 될 수 있도록, 교육하는 것이 저의 목표입니다.');

INSERT INTO ezy_tutor_ch6 (tutor_id, tutor_name, tutor_picture_url, tutor_profile)
    VALUES (2, 'Jayson Lennon', 'https://img-c.udemycdn.com/user/200_H/136320281_b44d_2.jpg', 'I''m looking forward to sharing my knowledge with you!');

INSERT INTO ezy_course_ch6 (course_id, tutor_id, course_name, course_level, posted_time)
    VALUES (1, 1, '액틱스 MVC 1편 - 백엔드 웹 개발 핵심 기술', 'Beginner', '2024-10-08 21:00:23');

INSERT INTO ezy_course_ch6 (course_id, tutor_id, course_name, course_format, posted_time)
    VALUES (2, 1, '액틱스 DB 1편 - 데이터 접근 핵심 원리', 'Video', '2024-10-08 21:00:45');