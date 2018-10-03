extern crate music_player_rs;
extern crate simplelog;
extern crate clap;
extern crate gtk;
extern crate glib;

use simplelog::{Level, LevelFilter, WriteLogger, Config};
use std::{path::Path, fs::File};
use clap::{Arg, App};
use music_player_rs::music_manager::{music_database::MusicDatabase};
use gtk::prelude::*;
use gtk::{WidgetExt, Inhibit, GtkWindowExt, ImageExt, TreeViewExt, TreeViewColumnExt,
    TreeViewColumn, GtkListStoreExtManual};

use gtk::Type::String as GTKString;

fn main() {
    let matches = App::new("music player")
                    .version("0.1")
                    .author("Adrián G. <adrian.garcia04@ciencias.unam.mx>")
                    .about("A music player written in Rust")
                    .arg(Arg::with_name("log output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Log file")
                        .takes_value(true))
                    .arg(Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Verbosity level"))
                    .get_matches();

    let log_file = matches.value_of("output").unwrap_or("music_player.log");

    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Warn,
        3 | _ => LevelFilter::max(),
    };

    let config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Error),
        location: Some(Level::Trace),
        time_format: Some("%r"),
    };

    let archivo_log = File::create(log_file).unwrap();
    WriteLogger::init(log_level, config, archivo_log).unwrap();

    let mut music_database = MusicDatabase::new();
    music_database.connect().unwrap();
    match music_database.mine() {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    }

    if gtk::init().is_err() {
        println!("Error initialiazing GTK");
        return;
    }

    gtk::Window::set_default_icon_from_file(Path::new("./src/ui/rust_logo.png")).unwrap();
    let music_player_glade = include_str!("ui/MusicPlayer.glade");
    let builder = gtk::Builder::new_from_string(music_player_glade);
    let window: gtk::Window = builder.get_object("MPWindow").unwrap();
    let album_image: gtk::Image = builder.get_object("AlbumImage").unwrap();
    let tree_view: gtk::TreeView = builder.get_object("TreeView").unwrap();
    let title_label: gtk::Label = builder.get_object("Title").unwrap();
    let album_label: gtk::Label = builder.get_object("Album").unwrap();
    let artist_label: gtk::Label = builder.get_object("Artist").unwrap();

    album_image.set_from_file(Path::new("./src/ui/music_album.png"));
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.maximize();
    window.show_all();

    let list_store: gtk::ListStore = gtk::ListStore::new(&[GTKString, GTKString, GTKString, GTKString]);
    for song in music_database.songs() {
        let title = song.title().to_value();
        let artist = song.artist().to_value();
        let album = song.album().to_value();
        let genre = song.genre().to_value();
        let data = [&title as &ToValue, &artist as &ToValue, &album as &ToValue, &genre as &ToValue];
        list_store.insert_with_values(None, &[0, 1, 2, 3], &data);
    }

    tree_view.append_column(&create_treeview_column("Title", 0));
    tree_view.append_column(&create_treeview_column("Artist", 1));
    tree_view.append_column(&create_treeview_column("Album", 2));
    tree_view.append_column(&create_treeview_column("Genre", 3));

    tree_view.expand_all();
    tree_view.set_model(&list_store);

    let tree_view_clone = tree_view.clone();
    tree_view.connect_cursor_changed(move |_| {
        let tree_selection: gtk::TreeSelection = tree_view_clone.get_selection();
        let (tree_model, tree_iter) = tree_selection.get_selected().unwrap();
        let title_value = tree_model.get_value(&tree_iter, 0);
        match title_value.get() {
            Some(title) => {
                title_label.set_text(title);
            },
            None => title_label.set_text("Unknown"),
        };
        let album_value = tree_model.get_value(&tree_iter, 1);
        match album_value.get() {
            Some(album) => {
                album_label.set_text(album);
            },
            None => album_label.set_text("Unknown"),
        };
        let artist_value = tree_model.get_value(&tree_iter, 2);
        match artist_value.get() {
            Some(artist) => {
                artist_label.set_text(artist);
            },
            None => artist_label.set_text("Unknown"),
        };
    });
    gtk::main();
}

fn create_treeview_column(title: &str, num_column: i32) -> TreeViewColumn {
    let cell_renderer: gtk::CellRendererText = gtk::CellRendererText::new();
    cell_renderer.set_visible(true);
    let view_column = TreeViewColumn::new();
    view_column.set_expand(true);
    view_column.set_visible(true);
    view_column.set_title(title);
    view_column.pack_start(&cell_renderer, true);
    view_column.add_attribute(&cell_renderer, "text", num_column);
    view_column.set_sort_column_id(num_column);
    view_column
}
