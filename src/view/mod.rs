mod styles;

use super::yew::html::*;
use super::{NicType, Msg, Model, Item};
use super::util::{get_ml, get_num, get_pct, get_grams, Material};

pub fn view(model: &Model) -> Html<Msg> {
    let nicPct = (&model.nicMg / &model.nicBase) * 100.0;
    let nicGrams = get_grams(get_ml(nicPct, &model), Material::Nic);
    // TODO: check js version


    let mut base_pg = model.pg;
    let mut base_vg = model.vg;

    // remove nic
    match model.nicType {
        NicType::VG => {
            base_vg -= nicPct;
        },
        NicType::PG => {
            base_pg -= nicPct;
        },
    }

    // remove liquids from base
    for item in &model.items {
        match item.nicType {
            NicType::VG => {
                base_vg -= get_pct(item.size, &model);
            },
            NicType::PG => {
                base_pg -= get_pct(item.size, &model);
            },
        }
    }

    html! {
        <div>
            <h1>{ "mixer calc" }</h1>

            <style>
                { styles::STYLESHEET }
            </style>

            <div class="row",>
                <div>
                    { "Bottle Size" }
                    <input
                        type="number",
                        min="0",
                        max="500",
                        step="10",
                        value=&model.bottleSize,
                        oninput=|e: InputData| Msg::BottleSize(e.value),
                        />
                        { "ml" }
                </div>

                <div >
                    { "PG" }
                    <input
                        type="number",
                        min="0",
                        max="100",
                        step="10",
                        value=&model.pg,
                        oninput=|e: InputData| Msg::PG(e.value),
                    />
                    { "VG" }
                    <input
                        type="number",
                        min="0",
                        max="100",
                        step="10",
                        value=&model.vg,
                        oninput=|e: InputData| Msg::VG(e.value),
                    />
                </div>
            </div>
            <div class="row",>

                    <div>
                        { "Nicotine" }
                        <input
                            type="number",
                            min="0",
                            max="100",
                            step="2",
                            value=&model.nicMg,
                            oninput=|e: InputData| Msg::NicMg(get_num(e.value)),
                            />
                            { "mg/ml" }
                        { " (Or" }
                        <input
                            type="number",
                            min="0",
                            max="10",
                            step="0.2",
                            value=&model.nicMg / 10.0,
                            oninput=|e: InputData| Msg::NicMg(get_num(e.value) * 10.0),
                            />
                            { "%)" }
                    </div>

                    <div>
                        { "Nicotine Base" }
                        <input
                            type="number",
                            min="0",
                            max="100",
                            step="2",
                            value="72",
                            oninput=|e: InputData| Msg::NicBase(e.value),
                            />
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
                </div>

            <div class="box",>
                { "Nicotine " }
                { format!("{:.2}% ", nicPct) }
                { format!("{:.2}ml", get_ml(nicPct, &model)) }
                { format!("{:.2}g (wrong)", nicGrams) }
            </div>
            <div class="box",>
                { "PG " }
                { format!("{:.2}% ", base_pg) }
                { format!("{:.2}ml", get_ml(base_pg, &model)) }
            </div>
            <div class="box",>
                { "VG " }
                { format!("{:.2}% ", base_vg) }
                { format!("{:.2}ml", get_ml(base_vg, &model)) }
            </div>

            { for model.items.iter().enumerate().map(|(i, item)| {
                view_item((i, item), &model)
            }) }

            <button
                onclick=|_| Msg::ItemNew,
            >
                {"Add Flavour"}
            </button>

            // <pre>
            //     { format!("{:#?}", &model) }
            // </pre>
        </div>
    }

}

fn view_item((i, item): (usize, &Item), model: &Model) -> Html<Msg> {
    html! {
        <div>
            <input
                type="number",
                min="0",
                max="500",
                step="1",
                value=item.size,
                oninput=move |e: InputData| Msg::ItemSize(i, get_num(e.value)),
                />
            {"ml "}

            <input
                type="number",
                min="0",
                max="500",
                step="1",
                value=format!("{:.2}", get_pct(item.size, &model)),
                oninput=move |e: InputData| Msg::ItemPct(i, get_num(e.value)),
                />
            {"% "}
            {format!("{:?}", item.nicType)}
            {" (VG"}
            <input
                type="checkbox",
                onclick=move |_| Msg::ItemNicType(i, NicType::VG),
                checked={ item.nicType == NicType::VG },
                />
                {"PG"}
            <input
                type="checkbox",
                onclick=move |_| Msg::ItemNicType(i, NicType::PG),
                checked={ item.nicType == NicType::PG },
                />
                {")"}
            <button onclick=move |_| Msg::ItemDel(i),>
                {"delete"}
            </button>
        </div>
    }
}
