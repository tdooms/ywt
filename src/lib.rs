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
macro_rules! use_effect {
    ( $y:expr ) => {
        yew::use_effect(move || {
            $y();
            || ()
        })
    };
    ( $y:expr; $z:expr ) => {
        yew::use_effect_with_deps(move |$(($z),)*| {
            $y();
            || ()
        }, $($z.clone();)*)
    };
    ( $( $x:ident ),*; $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            yew::use_effect(move || {
                $y();
                || ()
            })
        }
    };
    ( $( $x:ident ),*; $y:expr; $( $z:ident ),*) => {
        {
            $(let $x = $x.clone();)*
            yew::use_effect_with_deps(move |$(($z),)*| {
                $y();
                || ()
            }, $($z.clone();)*)
        }
    };
}