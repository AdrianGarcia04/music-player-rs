extern crate music_player_rs;
extern crate simplelog;
extern crate clap;
extern crate gtk;
extern crate glib;

use simplelog::{Level, LevelFilter, WriteLogger, Config};
use std::{path::Path, fs::File, cell::RefCell, sync::mpsc};
use clap::{Arg, App};
use music_player_rs::music_manager::{miner::{Miner, MinerEvent}, music_database::MusicDatabase};
use gtk::prelude::*;
use gtk::{WidgetExt, Inhibit, GtkWindowExt, ImageExt, TreeViewExt, TreeViewColumnExt,
    TreeViewColumn, GtkListStoreExtManual};

use gtk::Type::String as GTKString;

thread_local!(
    static GLOBAL: RefCell<Option<(gtk::Label, mpsc::Receiver<MinerEvent>)>> = RefCell::new(None);
    static DB: RefCell<Option<(gtk::ListStore, gtk::TreeView, MusicDatabase)>> = RefCell::new(None);
);

fn receive_percentage() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref label, ref rx)) = *global.borrow() {
            if let Ok(event) = rx.try_recv() {
                match event {
                    MinerEvent::Percentage(percentage) => {
                        let text = format!("Mining: {:.2}%", percentage*100.0);
                        label.set_text(&text);
                        if percentage >= 100.0 {
                            label.set_text("");
                        }
                    },
                    _ => {},
                }
            }
        }
    });
    glib::Continue(false)
}

fn database() -> glib::Continue {
    DB.with(|db| {
        if let Some((ref list_store, ref tree_view, ref database)) = *db.borrow() {
            for song in database.songs() {
                let title = song.get("title").unwrap().to_value();
                let artist = song.get("performer").unwrap().to_value();
                let album = song.get("album").unwrap().to_value();
                let genre = song.get("genre").unwrap().to_value();
                let data = [&title as &ToValue, &artist as &ToValue, &album as &ToValue, &genre as &ToValue];
                list_store.insert_with_values(None, &[0, 1, 2, 3], &data);
            }
            tree_view.set_model(list_store);
        }
    });
    glib::Continue(false)
}

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
    let status_label: gtk::Label = builder.get_object("StatusLabel").unwrap();
    let _footer_box: gtk::Box = builder.get_object("FooterBox").unwrap();
    let _navbar_box: gtk::Box = builder.get_object("NavbarBox").unwrap();
    let _prev_button: gtk::Button = builder.get_object("PrevButton").unwrap();
    let _play_button: gtk::Button = builder.get_object("PlayButton").unwrap();
    let _next_button: gtk::Button = builder.get_object("NextButton").unwrap();
    let _album_button: gtk::Button = builder.get_object("AlbumButton").unwrap();
    let _performer_button: gtk::Button = builder.get_object("PerformerButton").unwrap();
    let _mine_button: gtk::Button = builder.get_object("MineButton").unwrap();
    let _search_entry: gtk::SearchEntry = builder.get_object("SearchBar").unwrap();

    let mut miner = Miner::new();
    let listener = miner.get_listener();
    let listener_2 = miner.get_listener();
    std::thread::spawn(move || {
        miner.mine().unwrap();
    });

    GLOBAL.with(|global| {
        *global.borrow_mut() = Some((status_label, listener))
    });

    let (tx_db, rx_db) = mpsc::channel();
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = listener_2.recv() {
                match event {
                    MinerEvent::Percentage(_) => {
                        glib::idle_add(receive_percentage);
                    },
                    MinerEvent::Finished => {
                        tx_db.send(true).unwrap();
                        glib::idle_add(database);
                    },
                    _ => {},
                }
            }
        }
    });

    album_image.set_from_file(Path::new("./src/ui/music_album.png"));
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.maximize();
    window.show_all();

    let list_store: gtk::ListStore = gtk::ListStore::new(&[GTKString, GTKString, GTKString, GTKString]);

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

    let tree_view_ = tree_view.clone();
    let list_store_ = list_store.clone();
    let mut music_database = MusicDatabase::new();
    music_database.connect().unwrap();
    DB.with(|db| {
        *db.borrow_mut() = Some((list_store_, tree_view_, music_database))
    });
    // std::thread::spawn(move || {
    //     loop {
    //         if let Ok(miner_finished) = rx_db.recv() {
    //             if miner_finished {
    //                 println!("Miner Finished");
    //             }
    //         }
    //     }
    // });
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
