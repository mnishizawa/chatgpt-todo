
create database todos;

create table todos(
  id integer generated always as identity primary KEY,
  title varchar not null,
  completed bool default false
);