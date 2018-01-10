mod styles;

use super::yew::html::*;
use super::{NicType, Msg, Model, Item};
use super::util::{get_ml, get_num, get_pct, get_grams, Material};

pub fn view(model: &Model) -> Html<Msg> {

    // nicotine
    let nicPct = (&model.nicMg / &model.nicBase) * 100.0;
    let nicInputPct = &model.nicMg / 10.0;
    let nicStrength = get_grams(nicInputPct, Material::Nic);
    let nicMaterialType = match model.nicType {
        NicType::VG => Material::VG,
        NicType::PG => Material::PG,
    };
    let nicGrams = (nicStrength + ((100.0 - nicInputPct) * get_grams(1.0, nicMaterialType))) / 100.0;
    let nicWeight = get_ml(nicPct, &model) * nicGrams;


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
        match item.flavType {
            NicType::VG => {
                base_vg -= get_pct(item.size, &model);
            },
            NicType::PG => {
                base_pg -= get_pct(item.size, &model);
            },
        }
    }

    // PG
    let pg_ml = get_ml(base_pg, &model);
    let pg_grams = get_grams(pg_ml, Material::PG);
    // VG
    let vg_ml = get_ml(base_vg, &model);
    let vg_grams = get_grams(vg_ml, Material::VG);

    html! {
        <div>

            <h1>{"mixer calc"}</h1>

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
                    <input
                        type="number",
                        min="0",
                        max="100",
                        step="10",
                        value=&model.pg,
                        oninput=|e: InputData| Msg::PG(e.value),
                    />
                    { "PG" }
                    <input
                        type="number",
                        min="0",
                        max="100",
                        step="10",
                        value=&model.vg,
                        oninput=|e: InputData| Msg::VG(e.value),
                    />
                    { "VG" }
                    <button
                        class="add",
                        onclick=|_| Msg::ItemNew,
                    >
                        {"Add Flavour"}
                    </button>
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
                            value=nicInputPct,
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
                            value=&model.nicBase,
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

            <div class="row",>
                { "Nicotine " }
                <div>
                    { format!("{:.2}% - ", nicPct) }
                    { format!("{:.2}ml - ", get_ml(nicPct, &model)) }
                    { format!("{:.2}g", nicWeight) }
                </div>
            </div>
            <div class="row",>
                { "PG " }
                <div>
                    { format!("{:.2}% - ", base_pg) }
                    { format!("{:.2}ml - ", pg_ml) }
                    { format!("{:.2}g", pg_grams) }
                </div>
            </div>
            <div class="row",>
                { "VG " }
                <div>
                    { format!("{:.2}% - ", base_vg) }
                    { format!("{:.2}ml - ", vg_ml) }
                    { format!("{:.2}g", vg_grams) }
                </div>
            </div>

            { for model.items.iter().enumerate().map(|(i, item)| {
                view_item((i, item), &model)
            }) }

            // <pre>
            //     { format!("{:#?}", &model) }
            // </pre>
        </div>
    }

}

fn view_item((i, item): (usize, &Item), model: &Model) -> Html<Msg> {
    let materialType = match item.flavType {
        NicType::VG => Material::VG,
        NicType::PG => Material::PG,
    };
    html! {
        <div class="row",>
            <div>
                {"Flavour"}
                <input
                    type="number",
                    min="0",
                    max="500",
                    step="any",
                    value=item.size,
                    oninput=move |e: InputData| Msg::ItemSize(i, get_num(e.value)),
                    />
                    {"ml "}

                <input
                    type="number",
                    min="0",
                    max="500",
                    step="any",
                    value=format!("{:.2}", get_pct(item.size, &model)),
                    oninput=move |e: InputData| Msg::ItemPct(i, get_num(e.value)),
                    />
                    {"% "}
                {format!("{:.2}g ", get_grams(item.size, materialType))}
            </div>
            <div>
                {" (VG"}
                <input
                    type="checkbox",
                    onclick=move |_| Msg::ItemFlavType(i, NicType::VG),
                    checked={ item.flavType == NicType::VG },
                    />
                    {"PG"}
                <input
                    type="checkbox",
                    onclick=move |_| Msg::ItemFlavType(i, NicType::PG),
                    checked={ item.flavType == NicType::PG },
                    />
                    {")"}
                <button class="delete", onclick=move |_| Msg::ItemDel(i),>
                    {"delete"}
                </button>
            </div>
        </div>
    }
}
