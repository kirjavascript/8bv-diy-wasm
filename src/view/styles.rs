pub const STYLESHEET: &str = r#"

    html {
        color: #111111;
        font-size: 24px;
    }

    h1 {
        transform: rotate(180deg);
    }

    .row {
        display: flex;
        justify-content: space-between;
        align-items: stretch;
    }

    input {
        font-size: 24px;
        padding: 5px;
        width: 60px;
        text-align: center;
    }
    input[type="checkbox"] {
        width: initial;
    }

"#;
