use crate::{fs::JoshutoDirList, io::FileOperationProgress, preview::preview_file::FilePreview};
use crossterm::event::{poll, read, Event};
use signal_hook::{
    consts::signal,
    iterator::{exfiltrator::SignalOnly, SignalsInfo},
};
use std::{io, path, sync::mpsc, thread, time::Duration};
use uuid::Uuid;

#[derive(Debug)]
pub enum AppEvent {
    // User input events
    Crossterm(Event),

    // background IO worker events
    IoWorkerCreate,
    FileOperationProgress(FileOperationProgress),
    IoWorkerResult(io::Result<FileOperationProgress>),

    // forked process events
    ChildProcessComplete(u32),

    // preview thread events
    PreviewDir {
        id: Uuid,
        path: path::PathBuf,
        res: Box<io::Result<JoshutoDirList>>,
    },
    PreviewFile {
        path: path::PathBuf,
        res: Box<io::Result<FilePreview>>,
    },
    // terminal size change events
    Signal(i32),
    // filesystem change events
    Filesystem(notify::Event),
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Config {}

/// A small event handler that wrap Terminal input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    pub event_tx: mpsc::Sender<AppEvent>,
    event_rx: mpsc::Receiver<AppEvent>,
    //pub input_tx: mpsc::SyncSender<()>,
}

impl Events {
    pub fn new() -> Self {
        Self::default()
    }

    // We need a next() and a flush() so we don't continuously consume
    // input from the console. Sometimes, other applications need to
    // read terminal inputs while joshuto is in the background
    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        Ok(event)
    }

    pub fn flush(&self) {
        //let _ = self.input_tx.send(());
    }
}

impl std::default::Default for Events {
    fn default() -> Self {
        let (event_tx, event_rx) = mpsc::channel();

        // signal thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            let sigs = vec![signal::SIGWINCH];
            let mut signals = SignalsInfo::<SignalOnly>::new(sigs).unwrap();
            for signal in &mut signals {
                if let Err(e) = event_tx2.send(AppEvent::Signal(signal)) {
                    eprintln!("Signal thread send err: {:#?}", e);
                    return;
                }
            }
        });

        // input thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            // check for events ever .1 seconds. This should probably be even lower. Ask
            // maintainers if there is a better way.
            // should probably find a better way than unwrap
            if poll(Duration::from_millis(100)).unwrap() {
                if let Ok(event) = read() {
                    let _ = event_tx2.send(AppEvent::Crossterm(event));
                }
            }
        });

        Events { event_tx, event_rx }
    }
}
