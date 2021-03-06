#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::dsl::sql;
use self::models::{NewCart, Cart, NewUsr, Usr, NewBook, Book};

use std::collections::HashMap;

use rocket::Request;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

//Functions to create table entries

//Function to create an entry in the table cart
pub fn create_cart<'a>(conn: &PgConnection, isbn: &'a str, email: &'a str, quantity: &'a i32) -> Cart {
    use schema::cart;

    let new_cart = NewCart {
        isbn: isbn,
        email: email,
        quantity: quantity,
    };

    //Inserts the new_cart into the cart table
    diesel::insert_into(cart::table)
        .values(&new_cart)
        .get_result(conn)
        .expect("Error saving new cart")
}

//Function to create an entry in the table usr
pub fn create_usr<'a>(conn: &PgConnection, email: &'a str, passwrd: &'a str, addrss: &'a str, fname: &'a str, lname: &'a str) -> Usr {
    use schema::usr;

    let new_usr = NewUsr {
        email: email,
        passwrd: passwrd,
        addrss: addrss,
        fname: fname,
        lname: lname
    };

    //Inserts the new_usr into the usr table
    diesel::insert_into(usr::table)
        .values(&new_usr)
        .get_result(conn)
        .expect("Error saving new usr")
}

//Function to create an entry in the table book
pub fn create_book<'a>(conn: &PgConnection, isbn: &'a str,
 author_fname: &'a str, author_lname: &'a str, 
 title: &'a str, genre: &'a str, page_count: &'a i32,
 price: &'a f32, stock: &'a i16, prcnt_of_sale: &'a f32,
 pub_name: &'a str) -> Book {
    use schema::book;

    let new_book = NewBook {
        isbn: isbn,
        author_fname: author_fname,
        author_lname: author_lname,
        title: title,
        genre: genre,
        page_count: page_count,
        price: price,
        stock: stock,
        prcnt_of_sale: prcnt_of_sale,
        pub_name: pub_name
    };

    //Insets the new_book into the book table
    diesel::insert_into(book::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error saving new book")
}

//Connecting to the database
pub fn establish_connection() -> PgConnection {
    
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub mod schema;
pub mod models;

//Structs for templates

//Struct for book template
#[derive(Serialize)]
struct TemplateBookContext {
    Title: String,
    Author_fname: String,
    Author_lname: String,
    Price: f32,
    Isbn: String,
    Genre: String,
    Page_count: i32,
    Pub_name: String
}

//Struct for search results template
#[derive(Serialize)]
struct TemplateSearchContext {
    books: Vec<Book>
}

//Struct for empty template contexts
#[derive(Serialize)]
struct TemplateWriteContext {
}

//Structs for parsing form data

//Struct for search form
#[derive(FromForm)]
struct search {
    isbn: String,
    title: String,
    genre: String,
    fname: String,
    lname: String, 
}

//Struct for add book form
#[derive(FromForm)]
struct add {
    isbn: String,
    author_fname: String,
    author_lname: String, 
    title: String,
    genre: String,
    page_count: i32,
    price: f32,
    stock: i16,
    prcnt_of_sale: f32,
    pub_name: String,
}

//Routing

//Default route. Redirects to search.
#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(getsearch))
}

//Route for searching
#[get("/search")]
fn getsearch() -> Template {
    let context = TemplateWriteContext {};
    Template::render("search", &context)
}

//Route for showing search results
#[post("/search/finder", format = "form", data = "<temp>")]
fn postsearch(temp: Form<search>) -> Template {
    let connection = establish_connection();
    println!("{}", temp.isbn);
    //If the isbn is given then we know that there can only be one book
    if temp.isbn != "" {
        let sisbn = temp.isbn.to_string();
        return getbook(sisbn)
    }
    use schema::book::dsl::*;

    let sqlstatement = &format!("
    SELECT *
    FROM book
    WHERE title = {} OR genre = {}
    ", temp.title, temp.genre);
    let books = book.filter(sql(sqlstatement))
        .load::<Book>(&connection)
        .expect("Error loading books");
    
    Template::render("results", &TemplateSearchContext {
        books: books
    })
    //Template::render("results", context)
}

//Route for the add book form
#[get("/admin/addbook")]
fn addbook() -> Template {
    let context = TemplateWriteContext {};
    Template::render("addbook", &context)
}

//Route for data from the add book form
#[post("/admin/addbook/add", format = "form", data = "<temp>")]
fn postadd(temp: Form<add>) -> Redirect {
    let connection = establish_connection();
    
    let book = create_book(&connection, &temp.isbn,
    &temp.author_fname, &temp.author_lname, 
    &temp.title, &temp.genre, &temp.page_count,
    &temp.price, &temp.stock, &temp.prcnt_of_sale,
    &temp.pub_name);

    Redirect::to(uri!(getbook: isbn = &temp.isbn))
}

//Route for displaying book with isbn equal to the isbn in the route
#[get("/book/<isbn>")]
fn getbook(isbn: String) -> Template {
    use schema::book::dsl::*;

    let connection = establish_connection();
    let result = book.filter(isbn.eq(isbn))
        .limit(1)
        .load::<Book>(&connection)
        .expect("Error loading book");
    let Book = &result[0];
    let Title = &Book.title;
    let Title = Title.to_string();
    let Author_fname = &Book.author_fname;
    let Author_fname = Author_fname.to_string();
    let Author_lname = &Book.author_lname;
    let Author_lname = Author_lname.to_string();
    let Genre = &Book.genre;
    let Genre = Genre.to_string();
    let Page_count = Book.page_count;
    let Price = Book.price;
    let Pub_name = &Book.pub_name;
    let Pub_name = Pub_name.to_string();
    let Isbn = &Book.isbn;
    let Isbn = Isbn.to_string();
    let context = TemplateBookContext {Title, Author_fname, Author_lname, Price,
    Isbn, Genre, Page_count, Pub_name};
    Template::render("book", &context)
}

//Page not found route
#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}


//Function for building rockets
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, getsearch, postsearch,
            addbook, postadd, getbook])
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}