use super::schema::cart;
use super::schema::usr;
use super::schema::book;


#[derive(Insertable)]
#[table_name="cart"]
pub struct NewCart<'a> {
    pub isbn: &'a str,
    pub email: &'a str,
    pub quantity: &'a i32,
}

#[derive(Insertable)]
#[table_name="usr"]
pub struct NewUsr<'a> {
    pub email: &'a str,
    pub passwrd: &'a str,
    pub addrss: &'a str,
    pub fname: &'a str,
    pub lname: &'a str,
}

#[derive(Insertable)]
#[table_name="book"]
pub struct NewBook<'a> {
    pub isbn: &'a str,
    pub author_fname: &'a str,
    pub author_lname: &'a str, 
    pub title: &'a str,
    pub genre: &'a str, 
    pub page_count: &'a i32,
    pub price: &'a f32, 
    pub stock: &'a i16, 
    pub prcnt_of_sale: &'a f32,
    pub pub_name: &'a str,
}

#[derive(Queryable)]
pub struct Cart {
    pub isbn: String,
    pub email: String,
    pub quantity: i32,
}

#[derive(Queryable)]
pub struct Usr {
    pub email: String,
    pub passwrd: String,
    pub addrss: String,
    pub fname: String,
    pub lname: String,
}

#[derive(Queryable)]
pub struct Book {
    pub isbn: String,
    pub author_fname: String,
    pub author_lname: String, 
    pub title: String,
    pub genre: String, 
    pub page_count: i32,
    pub price: f32, 
    pub stock: i16, 
    pub prcnt_of_sale: f32,
    pub pub_name: String
}



