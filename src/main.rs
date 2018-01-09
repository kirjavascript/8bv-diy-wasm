#![recursion_limit="256"]
#![allow(non_snake_case)]
#[macro_use]
extern crate yew;

use yew::html::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

#[derive(Debug)]
struct Model {
    bottleSize: f64,
    pg: f64,
    vg: f64,
    nicBase: f64,
    nicMg: f64,
    nicType: NicType,
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
}

#[derive(Debug, PartialEq)]
enum NicType {
    VG, PG,
}

enum Msg {
    Log(String),
    BottleSize(String),
    PG(String),
    VG(String),
    NicMg(f64),
    NicType(NicType),
    NicBase(String),
}

fn get_num(s: String) -> f64 {
    match s.parse::<f64>() {
        Ok(v) => v,
        _ => 0.0,
    }
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
    }
}

fn get_ml(num: f64, model: &Model) -> f64 {
    (&model.bottleSize * num) / 100.0
}

fn view(model: &Model) -> Html<Msg> {
    let nicPct = (&model.nicMg / &model.nicBase) * 100.0;

    let mut base_pg = model.pg;
    let mut base_vg = model.vg;

    match &model.nicType {
        &NicType::VG => {
            base_vg -= nicPct;
        },
        &NicType::PG => {
            base_pg -= nicPct;
        },
    }

    // TODO:
    // remove flavour liquid from base liquids

    html! {
        <div>
            <h1>{ "mixer calc" }</h1>

            <div class="row",>
                { "Bottle Size" }
                <input
                    type="number",
                    min="0",
                    max="500",
                    step="10",
                    value=&model.bottleSize,
                    oninput=|e: InputData| Msg::BottleSize(e.value),
                    id="qty", />

                { "ml" }

                <div class="right",>
                    { "PG" }
                    <input
                        type="number",
                        min="0",
                        max="100",
                        step="10",
                        value=&model.pg,
                        oninput=|e: InputData| Msg::PG(e.value),
                        id="pg", />
                    { "VG" }
                    <input
                        type="number",
                        min="0",
                        max="100",
                        step="10",
                        value=&model.vg,
                        oninput=|e: InputData| Msg::VG(e.value),
                        id="vg", />
                </div>
            </div>
            <div class="row",>

                { "Nicotine" }
                <input
                    type="number",
                    min="0",
                    max="100",
                    step="2",
                    value=&model.nicMg,
                    oninput=|e: InputData| Msg::NicMg(get_num(e.value)),
                    id="mg", />
                        { "mg/ml" }
                { " (Or" }
                <input
                    type="number",
                    min="0",
                    max="10",
                    step="0.2",
                    value=&model.nicMg / 10.0,
                    oninput=|e: InputData| Msg::NicMg(get_num(e.value) * 10.0),
                    id="pct", />
                        { "%)" }
                <br/>

                { "Nicotine Base" }
                <input
                    type="number",
                    min="0",
                    max="100",
                    step="2",
                    value="72",
                    oninput=|e: InputData| Msg::NicBase(e.value),
                    id="nicBase", />
                    {"mg/ml"}
                    {" (VG"}
                    <input
                        type="checkbox",
                        onclick=|_| Msg::NicType(NicType::VG),
                        checked={ model.nicType == NicType::VG },
                    />
                    {"PG"}
                    <input
                        type="checkbox",
                        onclick=|_| Msg::NicType(NicType::PG),
                        checked={ model.nicType == NicType::PG },
                    />
                    {")"}
            </div>

            <div class="box",>
                { "Nicotine " }
                { format!("{:.2}", nicPct) } {"%"}
                { format!("{:.2}", get_ml(nicPct, &model)) } {"ml"}
            </div>
            <div class="box",>
                { "PG " }
                { format!("{:.2}", base_pg) } {"%"}
                { format!("{:.2}", get_ml(base_pg, &model)) } {"ml"}
            </div>
            <div class="box",>
                { "VG " }
                { format!("{:.2}", base_vg) } {"%"}
                { format!("{:.2}", get_ml(base_vg, &model)) } {"ml"}
            </div>

            <button
                id="add",
                onclick=|_| Msg::Log(String::from("TODO")),
            >
                {"Add Flavour"}
            </button>

            <pre>
                { format!("{:#?}", &model) }
            </pre>
        </div>
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
    app.mount(context, model, update, view);
    yew::run_loop();
}
