CREATE TABLE users (
  id VARCHAR NOT NULL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  created_by VARCHAR NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_by VARCHAR NOT NULL,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

insert into users (id, first_name, last_name, email, password, created_by, updated_by) values 
('00000000-0000-0000-0000-000000000000', 'admin', 'user', 'admin@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'),
('1802d2f8-1a18-43c1-9c58-1c3f7100c842', 'test', 'user', 'test@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000');