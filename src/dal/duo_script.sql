drop database if exists duo;
create database duo default character set utf8;
use duo;

create table users (
    id int not null auto_increment,
    username varchar(16) not null,
    email varchar(64) not null,
    password varchar(64) not null,
    total_wins int not null default 0,
    picture_id int not null default 0,
    primary key (id)
);


