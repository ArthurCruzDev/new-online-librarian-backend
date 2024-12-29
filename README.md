docker run --name mysql-server -e MYSQL_DATABASE=newonlinelibrarian -e MYSQL_ROOT_PASSWORD=mysql -p 3306:3306 -d mysql:8.4.0 --character-set-server=utf8mb4 --collation-server=utf8mb4_unicode_ci


docker build --tag new-online-librarian-backend --file Dockerfile .