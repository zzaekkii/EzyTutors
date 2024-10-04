/* 테이블이 존재하면 삭제 */
drop table if exists ezy_course_ch5;

/* 테이블 생성 */
create table ezy_course_ch5
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);


/* 테스트용 시드 데이터 로드 */
insert into ezy_course_ch5 (course_id, tutor_id, course_name, posted_time)
values (1, 1, '스프링 MVC 1편 - 백엔드 웹 개발 핵심 기술', '2024-09-06 21:07:08');

insert into ezy_course_ch5 (course_id, tutor_id, course_name, posted_time)
values (2, 1, '스프링 DB 1편 - 데이터 접근 핵심 원리', '2024-09-06 14:07:17');