CREATE DATABASE `demo` /*!40100 DEFAULT CHARACTER SET utf8mb4 */;
USE `demo`;

CREATE TABLE `users` (
  `id` char(36) NOT NULL,
  `username` varchar(128) NOT NULL,
  `email` varchar(128) NOT NULL,
  `password` varchar(128) NOT NULL,
  `fullname` varchar(128) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id_UNIQUE` (`id`),
  UNIQUE KEY `username_UNIQUE` (`username`),
  UNIQUE KEY `email_UNIQUE` (`email`)
) DEFAULT CHARSET=utf8mb4;

INSERT INTO `demo`.`users` (
  `id`,
  `username`,
  `email`,
  `password`,
  `fullname`
) VALUES (
  '28345084-0bc6-4cef-b846-8d7f8fa7a193',
  'kresna',
  'aditya.kresna@outlook.co.id',
  'SomeInsecurePassword2021',
  'Aditya Kresna'
);
