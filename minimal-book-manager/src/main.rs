// Imports 
use gsk4;
use gtk4::Stack;
use gtk4::CssProvider;
use gtk4::StyleContext;
use std::error::Error;
use sqlx::Row;
use gtk4::prelude::*;
use gtk4::glib::clone;
use gtk4::glib;
use gtk4::prelude::WidgetExt;
use gtk4::gdk;




// Book structure
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

// Function for create a book
async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    // Check if the book with the same ISBN already exists
    let verify_query = "SELECT COUNT(*) FROM book WHERE isbn = $1";
    let count: i64 = sqlx::query_scalar(verify_query)
        .bind(&book.isbn)
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Err("A book with the same ISBN already exists".into());
    }

    // Insert the book into the database
    let insert_query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(insert_query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}
// Function for remove book by isbn
async fn remove_book_by_isbn(pool: &sqlx::PgPool, isbn: &str) -> Result<(), Box<dyn Error>> {
    // Utiliser une requête DELETE pour retirer le livre par ISBN de la base de données
    let remove_isbn_query = "DELETE FROM book WHERE isbn = $1";
    sqlx::query(remove_isbn_query).bind(isbn).execute(pool).await?;

    Ok(())
}
// Function for remove book by name
async fn remove_book_by_title(pool: &sqlx::PgPool, title: &str) -> Result<(), Box<dyn Error>> {
    // Utiliser une requête DELETE pour retirer le livre par ISBN de la base de données
    let remove_book_query = "DELETE FROM book WHERE title = $1";
    sqlx::query(remove_book_query).bind(title).execute(pool).await?;

    Ok(())
}
// Function for read data
async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let show_query = "SELECT title, author, isbn FROM book";
    let show_query_result = sqlx::query(show_query).fetch_all(conn).await?;

    let books: Result<Vec<Book>, Box<dyn Error>> = show_query_result
        .into_iter()
        .map(|row| {
            let title_result = row.try_get("title").map_err(Into::<Box<dyn Error>>::into);
            let author_result = row.try_get("author").map_err(Into::<Box<dyn Error>>::into);
            let isbn_result = row.try_get("isbn").map_err(Into::<Box<dyn Error>>::into);

            let book = Book {
                title: title_result?,
                author: author_result?,
                isbn: isbn_result?,
            };
            
            Ok(book)
        })
        .collect();

    books
}

// When the application is launched
fn on_activate(application: &gtk4::Application) {
    // Create a new window …
    let window = gtk4::ApplicationWindow::new(application);
    
    // Set title to the window
    window.set_title(Some("Bookstore"));
    window.set_default_size(700, 400);

    // Create a stack to hold the pages
     
    // Create a vertical box layout to hold the widgets
    let box_layout = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    let create_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    let remove_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    let show_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    
    // Create stack
    let stack = Stack::new();
    box_layout.append(&stack);  

    // Create page 1/2/3
    stack.add_named(&show_box, Some("main_page"));
    stack.add_named(&create_box, Some("page_add"));
    stack.add_named(&remove_box, Some("page_remove"));

    // Create buttons to switch between pages
    let switch_to_page_main_button = gtk4::Button::with_label("Show Books");
    let switch_to_page_add_button = gtk4::Button::with_label("Create a book");
    let switch_to_page_remove_button = gtk4::Button::with_label("Remove a book");

    // Switch to Page 1 when the first button is clicked
    let stack_clone1 = stack.clone();
    switch_to_page_main_button.connect_clicked(move |_| {
        stack_clone1.set_visible_child_name("main_page");
    });

    // Switch to Page 2 when the first button is clicked
    let stack_clone2 = stack.clone();
    switch_to_page_add_button.connect_clicked(move |_| {
        stack_clone2.set_visible_child_name("page_add");
    });

    // Switch to Page 3 when the second button is clicked
    let stack_clone3 = stack.clone();
    switch_to_page_remove_button.connect_clicked(move |_| {
        stack_clone3.set_visible_child_name("page_remove");
    });

    // Create main title on top of the window
    let main_title = gtk4::Label::new(Some(""));

    // Create input create_title, create_author, create_isbn, remove_title, remove_isbn
    let text_entry_create_title = gtk4::Entry::new();
    let text_entry_create_author = gtk4::Entry::new();
    let text_entry_create_isbn = gtk4::Entry::new();
    let text_entry_remove_title = gtk4::Entry::new();
    let text_entry_remove_isbn = gtk4::Entry::new();

    // Set placeholder text create_title, create_author, create_isbn, remove_title, remove_isbn
    text_entry_create_title.set_placeholder_text(Some("Title"));
    text_entry_create_author.set_placeholder_text(Some("Author"));
    text_entry_create_isbn.set_placeholder_text(Some("Isbn"));    
    text_entry_remove_title.set_placeholder_text(Some("Title"));  
    text_entry_remove_isbn.set_placeholder_text(Some("Isbn"));

    // Create a button
    let show_button = gtk4::Button::with_label("Show Books");
    let create_button = gtk4::Button::with_label("Save Book");
    let remove_button = gtk4::Button::with_label("Remove Book");
    
    // CSS CLASSES

    // Set window css class
    window.add_css_class("back");

    // Set create_box css class
    box_layout.add_css_class("box-layout");
    create_box.add_css_class("create-box");
    remove_box.add_css_class("remove-box");
    show_box.add_css_class("show-box");

    // Set text entry css class
    text_entry_create_title.set_css_classes(&["text-entry", "top"]);
    text_entry_create_author.add_css_class("text-entry");
    text_entry_create_isbn.add_css_class("text-entry");
    text_entry_remove_title.add_css_class("text-entry");
    text_entry_remove_isbn.add_css_class("text-entry");

    // Set buttons css class
    // show_button.add_css_class("change-buttons");
    // create_button.add_css_class("change-buttons");
    // remove_button.add_css_class("change-buttons");

    // When create_button is pressed
    create_button.connect_clicked(clone!(@weak text_entry_create_title, @weak text_entry_create_author, @weak text_entry_create_isbn, @weak main_title => move |_| {    
        // Get text entered into entry
        let text_create_title = text_entry_create_title.text();
        let text_create_author = text_entry_create_author.text();
        let text_create_isbn = text_entry_create_isbn.text();

        if text_create_title.is_empty() || text_create_author.is_empty() || text_create_isbn.is_empty() {
            main_title.set_text("Error: one or more entries are empty");
        } 

        else {
            // Set text in entry at none
            text_entry_create_title.set_text("");
            text_entry_create_author.set_text("");
            text_entry_create_isbn.set_text("");

            let book = Book {
                title: text_create_title.to_string(),
                author: text_create_author.to_string(),
                isbn: text_create_isbn.to_string(),
            };

            // Execute the asynchronous function within the GTK main loop
            gtk4::glib::MainContext::default().spawn_local(async move {
                // Create pool/url
                let create_url = "postgres://dbuser:mysecretpassword@localhost:5432/bookstore";
                let create_pool = sqlx::postgres::PgPool::connect(create_url).await.expect("Failed to connect to the database pool");
                match create(&book, &create_pool).await {
                    Ok(_) => {
                        let main_title_create_content = format!("Book: {:?}, {:?}, {:?}, successfully entered", text_create_title, text_create_author, text_create_isbn); // Debug
                        main_title.set_text(&main_title_create_content);
                    }
                    Err(e) => {
                        let main_title_create_content = format!("Error inserting book: {:?}", e);
                        main_title.set_text(&main_title_create_content);
                    }
                }
            });
        }
    }));

    // When show_button is pressed
    show_button.connect_clicked( |_| {
        gtk4::glib::MainContext::default().spawn_local(async move {
            let show_url = "postgres://dbuser:mysecretpassword@localhost:5432/bookstore";
            let show_pool = sqlx::postgres::PgPool::connect(show_url).await.expect("Failed to connect to the database pool");

            let books_data = match read(&show_pool).await {
                Ok(books_data) => books_data,
                Err(err) => {
                    eprintln!("Error reading books: {}", err);
                    return Err(());
                }
            };
            // Debug now / need to show in window later
            println!("All Books:");
            for (index, book) in books_data.iter().enumerate() {
                println!("Book {}: {}", index + 1, book.title);
                println!("Author: {}", book.author);
                println!("ISBN: {}", book.isbn);
                println!();
            }

            Ok(())
        });
    });

    // When remove_button is pressed
    remove_button.connect_clicked(clone!(@weak text_entry_remove_title, @weak text_entry_remove_isbn, @weak main_title => move |_|{
        gtk4::glib::MainContext::default().spawn_local(async move {
            let url = "postgres://dbuser:mysecretpassword@localhost:5432/bookstore";
            let pool = sqlx::postgres::PgPool::connect(url).await.expect("Failed to connect to the database pool");

            let text_remove_title = text_entry_remove_title.text();
            let text_remove_isbn = text_entry_remove_isbn.text();

            if text_remove_title.is_empty() && text_remove_isbn.is_empty() {
                let main_title_remove_content = format!("You must enter at least one value!");
                main_title.set_text(&main_title_remove_content)
            } else if !text_remove_title.is_empty() && !text_remove_isbn.is_empty() {
                let main_title_remove_content = format!("You cannot remove by both title and ISBN");
                main_title.set_text(&main_title_remove_content)
            } else if !text_remove_title.is_empty() {
                match remove_book_by_title(&pool, &text_remove_title).await {
                    Ok(_) => {
                        let main_title_remove_content = format!("Book removed successfully!");
                        main_title.set_text(&main_title_remove_content);
                        text_entry_remove_title.set_text("")
                    }
                    Err(e) => {
                        let main_title_remove_content = format!("Error removing book : {:?}", e);
                        main_title.set_text(&main_title_remove_content)
                    }
                }
            } else {
                match remove_book_by_isbn(&pool, &text_remove_isbn).await {
                    Ok(_) => {
                        let main_title_remove_content = format!("Book removed successfully!");
                        main_title.set_text(&main_title_remove_content);
                        text_entry_remove_isbn.set_text("")
                    }
                    Err(e) => {
                        let main_title_remove_content = format!("Error removing book : {:?}", e);
                        main_title.set_text(&main_title_remove_content)
                    }
                }
            };
        }); 
    }));




    // Add the widget to the create_box
    create_box.append(&text_entry_create_title);
    create_box.append(&text_entry_create_author);
    create_box.append(&text_entry_create_isbn);
    create_box.append(&create_button);

    // Add the widgets to the remove_box
    remove_box.append(&text_entry_remove_title);
    remove_box.append(&text_entry_remove_isbn);
    remove_box.append(&remove_button);

    // Add the widgets to the show_boy
    show_box.append(&show_button);

    // Add the widgets to the box_layout
    box_layout.append(&main_title);
    box_layout.append(&switch_to_page_main_button);
    box_layout.append(&switch_to_page_add_button);
    box_layout.append(&switch_to_page_remove_button);

    // set animation
    stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(500);

    // Show box on window
    window.set_child(Some(&box_layout));

    // Load css 
    load_css();

    // Show window
    window.present();
}

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("../style/style.css"));
    StyleContext::add_provider_for_display(&display, &provider, priority);
}

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {

    // Connect database 
    let main_url = "postgres://dbuser:mysecretpassword@localhost:5432/bookstore";
    let main_pool = sqlx::postgres::PgPool::connect(main_url).await?;
    sqlx::migrate!("./migrations").run(&main_pool).await?;


    // Create a new application with the builder pattern
    let app = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(move |app| on_activate(app));

    // Run the application
    app.run();
    Ok(())
}
