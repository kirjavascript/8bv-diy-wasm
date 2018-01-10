pub const STYLESHEET: &str = r#"

    html {
        color: #111111;
        font-size: 24px;
    }

    h1 {
        transform: rotate(180deg);
        color: #aac;
        margin-top: 0;
        margin-right: 10px;
    }

    .row {
        display: flex;
        justify-content: space-between;
        align-items: stretch;
        padding: 10px;
        margin: 10px;
        border-radius: 10px;
        background-color: #aac;
        box-shadow: 3px 3px 3px #ccc;
    }

    button {
        padding: 5px 10px;
        font-size: 24px;
        border-radius: 15px;
        margin-left: 20px;
    }
    input {
        font-size: 24px;
        padding: 5px;
        text-align: center;
        border-radius: 15px;
    }
    input[type="number"] {
        margin: 0 20px;
        width: 60px;
    }
    input[type="checkbox"] {
        width: initial;
        transform: scale(1.5);
        padding: 5px;
    }

    @media only screen and (max-width: 1000px) {
        html, input, button {
            font-size: 18px;
        }
    }

    @media only screen and (max-width: 900px) {
        html, input, button {
            font-size: 12px;
        }
    }

    @media only screen and (max-width: 770px) {
        .row {
            flex-direction: column;
            align-items: initial;
            > div {
                margin: 2px 0;
            }
        }
    }

"#;
