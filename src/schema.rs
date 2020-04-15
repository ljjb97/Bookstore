table! {
    addrss_postal_code (addrss) {
        addrss -> Varchar,
        postal_code -> Nullable<Varchar>,
    }
}

table! {
    book (isbn) {
        isbn -> Varchar,
        author_fname -> Varchar,
        author_lname -> Varchar,
        title -> Varchar,
        genre -> Varchar,
        page_count -> Int4,
        price -> Float4,
        stock -> Int2,
        prcnt_of_sale -> Float4,
        pub_name -> Varchar,
    }
}

table! {
    cart (email, isbn) {
        isbn -> Varchar,
        email -> Varchar,
        quantity -> Int4,
    }
}

table! {
    oder (oder_id) {
        oder_id -> Int4,
        addrss -> Varchar,
        card_num -> Varchar,
        tracking_num -> Int4,
        sttus -> Nullable<Varchar>,
        email -> Varchar,
    }
}

table! {
    order_contents (order_id, isbn, direction) {
        order_id -> Int4,
        isbn -> Varchar,
        direction -> Varchar,
        quantity -> Int4,
    }
}

table! {
    order_from_pub (order_id) {
        order_id -> Int4,
        cost -> Int4,
        tracking_number -> Int4,
        sttus -> Nullable<Varchar>,
        pub_name -> Nullable<Varchar>,
    }
}

table! {
    pub_contact (email) {
        email -> Varchar,
        fname -> Varchar,
        lname -> Varchar,
        phone_num -> Varchar,
        role -> Varchar,
        pub_name -> Varchar,
    }
}

table! {
    publisher (pub_name) {
        pub_name -> Varchar,
        addrss -> Varchar,
        bank_account -> Varchar,
    }
}

table! {
    usr (email) {
        email -> Varchar,
        passwrd -> Varchar,
        addrss -> Varchar,
        fname -> Varchar,
        lname -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    addrss_postal_code,
    book,
    cart,
    oder,
    order_contents,
    order_from_pub,
    pub_contact,
    publisher,
    usr,
);
