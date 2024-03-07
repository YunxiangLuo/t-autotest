use std::sync::mpsc;

use t_binding::{JSEngine, LuaEngine, ScriptEngine};

pub enum Msg {
    Stop(mpsc::Sender<()>),
    Script(String),
}

pub struct EngineClient {
    msg_tx: mpsc::Sender<Msg>,
}
impl EngineClient {
    pub fn stop(&self) {
        let (tx, rx) = mpsc::channel();
        self.msg_tx.send(Msg::Stop(tx)).unwrap();
        rx.recv().unwrap();
    }

    pub fn run(&self, script: &str) {
        self.msg_tx.send(Msg::Script(script.to_string())).unwrap();
    }
}

pub struct Engine {
    ext: String,
    script_rx: mpsc::Receiver<Msg>,
}

impl Engine {
    pub fn new(ext: &str) -> (Self, EngineClient) {
        let (tx, rx) = mpsc::channel();
        (
            Self {
                ext: ext.to_string(),
                script_rx: rx,
            },
            EngineClient { msg_tx: tx },
        )
    }

    pub fn start(&mut self) {
        let _e: Box<dyn ScriptEngine> = match self.ext.as_str() {
            "js" => Box::new(JSEngine::new()),
            "lua" => Box::new(LuaEngine::new()),
            _ => unimplemented!(),
        };

        while let Ok(msg) = self.script_rx.recv() {
            match msg {
                Msg::Stop(tx) => {
                    tx.send(()).unwrap();
                    break;
                }
                Msg::Script(script) => {
                    self.run(&script);
                }
            }
        }
    }

    fn run(&mut self, script: &str) {
        let mut e: Box<dyn ScriptEngine> = match self.ext.as_str() {
            "js" => Box::new(JSEngine::new()),
            "lua" => Box::new(LuaEngine::new()),
            _ => unimplemented!(),
        };
        e.run(script);
    }
}