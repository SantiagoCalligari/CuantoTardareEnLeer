CREATE TABLE books {
  id int(32) primary key,
  title text not null,
  author text not null,
  url_cover not null,
  word_count i32 not null,
  language text not null
}
