use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc,
    time::Duration,
};

use glib::{clone, timeout_add_local};
use gtk::gdk::Key;
use gtk::{prelude::*, EventControllerFocus, EventControllerKey, Inhibit};
use gtk::{Application, ApplicationWindow};
use midir::{MidiOutput, MidiOutputConnection};
use midly::{
    live::LiveEvent,
    num::{u4, u7},
    MidiMessage,
};

type Note = u7;

// fn button_event(button: &gtk::Button, state: &Rc<State>) {
//     let key = button.widget_name();

//     match key_to_midi(key.as_str()) {
//         Some(note) => {
//             state.borrow_mut().note_on(note);
//             let state = state.clone();
//             timeout_add_local(Duration::from_millis(500), move || {
//                 state.borrow_mut().note_off(28);
//                 Continue(false)
//             });
//         }
//         None => {}
//     }
// }

// fn key_to_midi(key: &str) -> Option<u8> {
//     Some(
//         48 + match key {
//             "backslash" | "1" => 28,
//             "a" => 29,
//             "w" => 30,
//             "z" => 31,
//             "s" => 32,
//             "e" => 33,
//             "x" => 34,
//             "d" => 35,
//             "r" => 36,
//             "c" => 37,
//             "f" => 38,
//             "t" => 39,
//             "v" => 40,
//             "g" => 41,
//             "y" => 42,
//             "b" => 43,
//             k => {
//                 println!("Unmapped key: {}", k);
//                 return None;
//             }
//         },
//     )
// }

fn main() -> glib::ExitCode {
    let state = Rc::new(State::new("src/map.txt").expect("Couldn't initialise State"));
    let app = Application::builder()
        .application_id("eu.muehml.CBAKeyboard")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(240)
            .title("Chromatic Button Accordion Virtual Keyboard")
            .build();

        let buttonboard = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let first_row = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let button = gtk::Button::builder()
            .can_focus(false)
            .label("\\")
            .name("backslash")
            .build();
        // button.connect_clicked(clone!(@strong state => move|b|
        //     button_event(b, &state);
        // ));

        first_row.append(&button);

        buttonboard.append(&first_row);

        let keyboard_listener = EventControllerKey::new();

        keyboard_listener.connect_key_pressed(
            clone!(@strong state => move |_controller, key, _keycode, modifiers| {
                if key == Key::space {
                    state.borrow_mut().midi_panic();
                    return Inhibit(true)
                }
                if !modifiers.is_empty() {
                    return Inhibit(false);
                }
                Inhibit(state.borrow_mut().press_key(key.name().unwrap().as_str()))
            }),
        );
        keyboard_listener.connect_key_released(
            clone!(@strong state => move |_controller, key, _keycode, modifiers| {
                if !modifiers.is_empty() {
                    return;
                }
                state.borrow_mut().release_key(key.name().unwrap().as_str());
            }),
        );

        let focus_listener = EventControllerFocus::new();

        focus_listener.connect_leave(clone!(@strong state => move |_controller| {
            state.borrow_mut().midi_panic();
        }));
        window.add_controller(keyboard_listener);
        window.add_controller(focus_listener);

        window.set_child(Some(&buttonboard));

        window.show();
    });

    app.run()
}

struct State(RefCell<StateInner>);

impl State {
    pub fn borrow(&self) -> std::cell::Ref<'_, StateInner> {
        self.0.borrow()
    }

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
    midi_connection: MidiOutputConnection,
    held_keys: HashSet<Key>,
    held_notes: HashMap<Note, usize>,
    key_to_note: HashMap<Key, Note>,
    note_to_keys: HashMap<Note, HashSet<Key>>,
}

impl StateInner {
    fn new<P: AsRef<Path>>(map_file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let output = MidiOutput::new("Chromatic Keyboard")?;
        let port = &output.ports()[0];
        let midi_connection = output.connect(port, "cba")?;

        let (key_to_note, note_to_keys) = Self::read_map_file(map_file_path)?;

        Ok(Self {
            midi_connection,
            held_keys: HashSet::new(),
            held_notes: HashMap::new(),
            key_to_note,
            note_to_keys,
        })
    }

    fn read_map_file<P: AsRef<Path>>(
        map_file_path: P,
    ) -> Result<(HashMap<Key, Note>, HashMap<Note, HashSet<Key>>), Box<dyn std::error::Error>> {
        let file = BufReader::new(File::open(map_file_path)?);
        let mut key_to_note = HashMap::new();
        let mut note_to_keys = HashMap::new();
        for (i, line) in file.lines().enumerate() {
            let line = line?;
            let note = Note::new(i as u8 + 41);
            let keys: Result<HashSet<Key>, KeyParseError> = line
                .split_whitespace()
                .map(|name| Key::from_name(name).ok_or_else(|| KeyParseError(name.to_string())))
                .collect();
            let keys = keys?;

            for key in &keys {
                key_to_note.insert(*key, note);
            }

            note_to_keys.insert(note, keys);
        }
        Ok((key_to_note, note_to_keys))
    }

    fn press_key(&mut self, key_name: &str) -> bool {
        let key = Key::from_name(key_name).unwrap();
        let Some(&note) = self.key_to_note.get(&key) else {
            println!("Unmapped Key: {}", key_name);
            return true;
        };
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
        let count = self.held_notes.entry(note).or_insert(1);
        *count -= 1;
        if *count == 0 {
            self.note_off(note);
        }
    }

    fn note_on(&mut self, note: Note) {
        println!("sending note_on {}", note);
        let event = LiveEvent::Midi {
            channel: u4::new(1),
            message: MidiMessage::NoteOn {
                key: note,
                vel: Note::new(64),
            },
        };

        let mut buf = Vec::new();
        event
            .write(&mut buf)
            .expect("Coulnd't serialize midi message");
        self.midi_connection
            .send(&buf)
            .expect("Couldn't send midi message");
    }
    fn note_off(&mut self, note: Note) {
        println!("sending note_off {}", note);
        let event = LiveEvent::Midi {
            channel: u4::new(1),
            message: MidiMessage::NoteOff {
                key: note,
                vel: Note::new(64),
            },
        };

        let mut buf = Vec::new();
        event
            .write(&mut buf)
            .expect("Coulnd't serialize midi message");
        self.midi_connection
            .send(&buf)
            .expect("Couldn't send midi message");
    }

    fn midi_panic(&mut self) {
        println!("sedning All Notes Off");

        self.held_keys.clear();
        self.held_notes.clear();

        let event1 = LiveEvent::Midi {
            channel: u4::new(1),
            message: MidiMessage::Controller {
                controller: Note::new(123),
                value: Note::new(127),
            },
        };
        let event2 = LiveEvent::Midi {
            channel: u4::new(1),
            message: MidiMessage::Controller {
                controller: Note::new(123),
                value: Note::new(0),
            },
        };

        let mut buf = Vec::new();
        event1
            .write(&mut buf)
            .expect("Coulnd't serialize midi message");
        event2
            .write(&mut buf)
            .expect("Coulnd't serialize midi message");
        self.midi_connection
            .send(&buf)
            .expect("Couldn't send midi message");
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
