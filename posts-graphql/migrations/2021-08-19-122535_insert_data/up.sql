insert into authors(name) values ('J. K. Rowling');
insert into posts(title, description, author_id) values ('Harry Potter 1', 'Description of Harry Potter 1', (select currval('authors_id_seq')));
insert into posts(title, description, author_id) values ('Harry Potter 2', 'Description of Harry Potter 2', (select currval('authors_id_seq')));
insert into authors(name) values ('Jane Austen');
insert into posts(title, description, author_id) values ('Pride and Prejudice', 'Description of Pride and Prejudice', (select currval('authors_id_seq')));
insert into authors(name) values ('Mark Twain');
insert into posts(title, description, author_id) values ('The Adventures of Huckleberry Finn', 'Description of adventures of Huckleberry Finn', (select currval('authors_id_seq')));
