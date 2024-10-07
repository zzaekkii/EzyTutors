/* 테이블이 존재하면 삭제 */
drop table if exists ezy_course_ch6;

/* 테이블 생성 */
create table ezy_course_ch6
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price INT,
    course_language varchar(30),
    course_level varchar(30),
    posted_time TIMESTAMP default now()
);


/* 테스트용 시드 데이터 로드 */
insert into ezy_course_ch6 (course_id, tutor_id, course_name, posted_time)
values (1, 1, '액틱스 MVC 1편 - 백엔드 웹 개발 핵심 기술', '2024-10-05 20:14:03');

insert into ezy_course_ch6 (course_id, tutor_id, course_name, posted_time)
values (2, 1, '액틱스 DB 1편 - 데이터 접근 핵심 원리', '2024-10-05 20:14:15');