## There are six small projects in this repo, all programmed in Rust: 

### Hangman : 
A simple hangman game in which one person enters a word and another has to divide it by entering letters. This was my very first project in rust which required me to think a bit.

### Minimal-Book-Manager : 
A book manager with a graphical interface in which you can add and delete books in a database. I used Sqlx as the database and gtk4 for the interface. This was my first use of a database in Rust.

Depedencies for Minimal-Book-Manager :

* Install docker (Ubuntu/Debian) : 
```
sudo snap install docker
```
Run Minimal-Book-Manager : 

* Run docker with user and password:
```
sudo docker run -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_USER=dbuser -e POSTGRES_DB=bookstore -p 5432:5432 postgres
```
If you'd like to change passsword and user you have to change the pools variables on src/main.rs


### Rock-Paper-Scissors : 
A simple paper rock scissors game. This is my first project in Rust.

### The-Price-Is-Right : 
A simple the price is high game with a graphical interface (gtk4). This is my firt use of a graphical interface.

### TicTacToe : 
A simple tic-tac-toe game in the terminal. 

### Webassembly-Counter : 
Is a website with a counter that can be incremented and decremented. It uses Webassembly compiled from Rust. This was my first use of Webassembly.

Depedencies for Wabassembly-Counter :

* Install WebAssembly target : 
```
rustup target add wasm32-unknown-unknown
```
* Install trunk and target :
```
cargo install trunk target
```
Run Wabassembly-Counter :

```
trunk serve
```

