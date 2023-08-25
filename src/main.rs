use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    rc::Rc,
};

use gtk::{
    ffi::GTK_INVALID_LIST_POSITION,
    gdk::Key,
    gio::{self, SimpleAction},
    glib::{self, clone},
    AboutDialog, DropDown, HeaderBar, Label, StringList, Window, Builder, MenuButton,
};
use gtk::{prelude::*, EventControllerFocus, EventControllerKey, Inhibit};
use gtk::{Application, ApplicationWindow};
use midir::{MidiOutput, MidiOutputConnection};
use midly::{
    live::LiveEvent,
    num::{u4, u7},
    MidiMessage,
};
mod config;

type Note = u7;

fn build_ui(state: Rc<State>, app: &Application) {
    let builder = Builder::from_resource("/eu/muehml/cba-midi/window.blp");
    let window: ApplicationWindow = builder.object("window").expect("Coudln't get window");
    window.set_application(Some(app));


    let keyboard_listener = EventControllerKey::new();

    let about: AboutDialog = builder.object("about").expect("Coudln't get about");
    about.set_authors(&["Lars Mühmel <lars@muehml.eu>"]);

    let action_about = SimpleAction::new("about", None);
    action_about.connect_activate(move |_, _| about.show());
    app.add_action(&action_about);

    let octave_switcher: DropDown = builder.object("octave_switcher").expect("Coudln't get octave-switcher");

    octave_switcher.connect_selected_notify(clone!(@strong state => move |dropdown| {
        state.borrow_mut().octave = dropdown.selected() as u8;
    }));

    keyboard_listener.connect_key_pressed(
        clone!(@strong state => move |_controller, key, _keycode, _modifiers| {
            if key == Key::space {
                state.borrow_mut().midi_panic();
                return Inhibit(true)
            }
            let key = key.to_lower();
            Inhibit(state.borrow_mut().press_key(key.name().unwrap().as_str()))
        }),
    );
    keyboard_listener.connect_key_released(
        clone!(@strong state => move |_controller, key, _keycode, _modifiers| {
            let key = key.to_lower();
            state.borrow_mut().release_key(key.name().unwrap().as_str());
        }),
    );

    let focus_listener = EventControllerFocus::new();

    focus_listener.connect_leave(clone!(@strong state => move |_controller| {
        state.borrow_mut().midi_panic();
    }));
    window.add_controller(keyboard_listener);
    window.add_controller(focus_listener);


    window.show();
    build_connection_window(state.clone()).expect("Couldn't create connection window");
}

fn main() -> glib::ExitCode {
    let res = gio::Resource::load(config::RESOURCE_FILE).expect("Coudln't load resource file");
    gio::resources_register(&res);
    let mut map_path = PathBuf::from(config::PKGDATADIR);
    map_path.push("map.txt");
    dbg!(&map_path);
    let state = Rc::new(State::new(map_path).expect("Couldn't initialise State"));
    let app = Application::builder()
        .application_id(config::APP_ID)
        .build();

    app.connect_activate(clone!(@strong state => move |app| build_ui(state.clone(), app)));

    app.run()
}

fn build_connection_window(state: Rc<State>) -> Result<(), Box<dyn std::error::Error>> {
    let builder = Builder::from_resource("/eu/muehml/cba-midi/window.blp");
    let window: Window = builder.object("connect_window").expect("Coudln't get connect_window");
    let output = MidiOutput::new("Chromatic Keyboard")?;

    let available_ports = output.ports();

    // Otherwise this gives a warning on windows
    #[cfg_attr(windows, allow(unused_mut))]
    let mut ports_with_names: Vec<_> = available_ports
        .into_iter()
        .filter_map(|port| match output.port_name(&port) {
            Ok(name) => Some((Some(port), name)),
            Err(_) => None,
        })
        .collect();

    #[cfg(not(windows))]
    ports_with_names.push((None, String::from("Virtual Output")));

    let names: Vec<&str> = ports_with_names
        .iter()
        .map(|(_, name)| name.as_str())
        .collect();

    let output = Cell::new(Some(output));
    let names = StringList::new(&names);
    let dropdown: DropDown = builder.object("output_dropdown").expect("Coudln't get output_dropdown");
    dropdown.set_model(Some(&names));
    dropdown.set_selected(GTK_INVALID_LIST_POSITION);
    dropdown.connect_selected_notify(clone!(@strong window => move |dropdown| {
        if let Some(out) = output.replace(None).take() {
            let idx = dropdown.selected();
            let port = &ports_with_names[idx as usize].0;
            let connection = match port {
                Some(port) => out.connect(port, "cba").expect("Connection failed"),
                None => {
                    #[cfg(not(windows))]
                    {
                        midir::os::unix::VirtualOutput::create_virtual(out, "cba virtual").expect("Couldn't create a virtual output")
                    }
                    #[cfg(windows)]
                    {
                        unreachable!("port should never be None on windows")
                    }
                }
            };
            state.borrow_mut().midi_port = Some(connection);
            window.close();
        }
    }));

    window.show();

    Ok(())
}

struct State(RefCell<StateInner>);

impl State {
    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, StateInner> {
        self.0.borrow_mut()
    }
}

impl State {
    fn new<P: AsRef<Path>>(map_file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(State(RefCell::new(StateInner::new(map_file_path)?)))
    }
}

struct StateInner {
    midi_port: Option<MidiOutputConnection>,
    octave: u8,
    held_keys: HashSet<Key>,
    held_notes: HashMap<Note, usize>,
    key_to_note: HashMap<Key, Note>,
}

impl StateInner {
    fn new<P: AsRef<Path>>(map_file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let key_to_note = Self::read_map_file(map_file_path)?;

        Ok(Self {
            midi_port: None,
            held_keys: HashSet::new(),
            held_notes: HashMap::new(),
            key_to_note,
            octave: 4,
        })
    }

    fn read_map_file<P: AsRef<Path>>(
        map_file_path: P,
    ) -> Result<HashMap<Key, Note>, Box<dyn std::error::Error>> {
        let file = BufReader::new(File::open(map_file_path)?);
        let mut key_to_note = HashMap::new();
        for (i, line) in file.lines().enumerate() {
            let line = line?;
            let note = Note::new(i as u8);
            let keys: Result<HashSet<Key>, KeyParseError> = line
                .split_whitespace()
                .map(|name| Key::from_name(name).ok_or_else(|| KeyParseError(name.to_string())))
                .collect();
            let keys = keys?;

            for key in &keys {
                key_to_note.insert(*key, note);
            }
        }
        Ok(key_to_note)
    }

    fn press_key(&mut self, key_name: &str) -> bool {
        let key = Key::from_name(key_name).unwrap();
        let Some(&note) = self.key_to_note.get(&key) else {
            println!("Unmapped Key: {}", key_name);
            return true;
        };
        let note = note + u7::new(self.octave * 12);
        if self.held_keys.contains(&key) {
            return false;
        }
        self.held_keys.insert(key);
        let count = self.held_notes.entry(note).or_insert(0);
        *count += 1;
        if *count == 1 {
            self.note_on(note);
        }
        true
    }
    fn release_key(&mut self, key_name: &str) {
        let key = Key::from_name(key_name).unwrap();
        if !self.held_keys.remove(&key) {
            return;
        }
        let Some(&note) = self.key_to_note.get(&key) else {
            return;
        };
        let note = note + u7::new(self.octave * 12);
        let count = self.held_notes.entry(note).or_insert(1);
        *count -= 1;
        if *count == 0 {
            self.note_off(note);
        }
    }

    fn note_on(&mut self, note: Note) {
        if let Some(ref mut conn) = self.midi_port {
            println!("sending note_on {}", note);
            let event = LiveEvent::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOn {
                    key: note,
                    vel: Note::new(100),
                },
            };

            let mut buf = Vec::new();
            event
                .write(&mut buf)
                .expect("Couldn't serialize midi message");

            conn.send(&buf).expect("Couldn't send midi message");
        }
    }
    fn note_off(&mut self, note: Note) {
        if let Some(ref mut conn) = self.midi_port {
            println!("sending note_off {}", note);
            let event = LiveEvent::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOff {
                    key: note,
                    vel: Note::new(127),
                },
            };

            let mut buf = Vec::new();
            event
                .write(&mut buf)
                .expect("Couldn't serialize midi message");
            conn.send(&buf).expect("Couldn't send midi message");
        }
    }

    fn midi_panic(&mut self) {
        if let Some(ref mut conn) = self.midi_port {
            println!("sending All Notes Off");

            self.held_keys.clear();
            self.held_notes.clear();

            let event1 = LiveEvent::Midi {
                channel: u4::new(0),
                message: MidiMessage::Controller {
                    controller: Note::new(123),
                    value: Note::new(127),
                },
            };
            let event2 = LiveEvent::Midi {
                channel: u4::new(0),
                message: MidiMessage::Controller {
                    controller: Note::new(123),
                    value: Note::new(0),
                },
            };

            let mut buf = Vec::new();
            event1
                .write(&mut buf)
                .expect("Couldn't serialize midi message");
            event2
                .write(&mut buf)
                .expect("Couldn't serialize midi message");
            conn.send(&buf).expect("Couldn't send midi message");
        }
    }
}

#[derive(Debug)]
struct KeyParseError(String);

impl fmt::Display for KeyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Didn't recognise key: {:?}", self.0)
    }
}

impl std::error::Error for KeyParseError {}
