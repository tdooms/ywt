#[macro_export]
macro_rules! spawn {
    ( $y:expr ) => {
        wasm_bindgen_futures::spawn_local($y)
    };
    ( $( $x:ident ),*; $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            wasm_bindgen_futures::spawn_local($y)
        }
    };
}

#[macro_export]
macro_rules! callback {
    ( $y:expr ) => {
        Callback::from($y)
    };
    ( $( $x:ident ),*; $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            Callback::from($y)
        }
    };
}

#[macro_export]
macro_rules! clone(
    ( $( $x:ident ),* ) => {
        $(let $x = $x.clone();)*
    }
);