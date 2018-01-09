#![recursion_limit="256"]
#![allow(non_snake_case)]
#[macro_use]
extern crate yew;

mod view;
mod util;

use util::{get_num, get_ml};

use yew::html::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

#[derive(Debug)]
pub struct Model {
    bottleSize: f64,
    pg: f64,
    vg: f64,
    nicBase: f64,
    nicMg: f64,
    nicType: NicType,
    items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    size: f64, // (ml)
    nicType: NicType,
}

#[derive(Debug, PartialEq)]
pub enum NicType {
    VG, PG,
}

pub enum Msg {
    Log(String),
    BottleSize(String),
    PG(String),
    VG(String),
    NicMg(f64),
    NicType(NicType),
    NicBase(String),
    ItemNew,
    ItemDel(usize),
    ItemSize(usize, f64),
    ItemPct(usize, f64),
    ItemNicType(usize, NicType),
}



fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Log(str) => {
            context.console.log(&str);
        },
        Msg::BottleSize(str) => {
            model.bottleSize = get_num(str);
        },
        Msg::PG(str) => {
            let v = get_num(str);
            model.pg = v;
            model.vg = 100.0 - v;
        },
        Msg::VG(str) => {
            let v = get_num(str);
            model.vg = v;
            model.pg = 100.0 - v;
        },
        Msg::NicType(t) => {
            model.nicType = t;
        },
        Msg::NicMg(v) => {
            model.nicMg = v;
        },
        Msg::NicBase(str) => {
            let v = get_num(str);
            model.nicBase = v;
        },
        Msg::ItemNew => {
            model.items.push(Item {
                size: 3.0,
                nicType: NicType::PG,
            });
        },
        Msg::ItemDel(i) => {
            model.items.remove(i);
        },
        Msg::ItemSize(i, v) => {
            model.items[i].size = v;
        },
        Msg::ItemPct(i, v) => {
            model.items[i].size = get_ml(v, &model);
        },
        Msg::ItemNicType(i, t) => {
            model.items[i].nicType = t;
        },
    }
}



fn main() {
    yew::initialize();
    let mut app = App::new();
    let context = Context {
        console: ConsoleService,
    };
    let model = Model {
        bottleSize: 30.0,
        pg: 20.0,
        vg: 80.0,
        nicBase: 72.0,
        nicType: NicType::VG,
        nicMg: 10.0,
        items: Vec::new(),
    };
    app.mount(context, model, update, view::view);

    yew::run_loop();
}
