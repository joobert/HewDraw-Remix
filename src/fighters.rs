macro_rules! install_fighters {
    ($func:ident; $($name:ident = $feature:expr),*) => {{
        $(
            #[cfg(feature = $feature)]
            { $name::$func() }
        )*
    }}
}

pub fn install() {
    #[cfg(not(feature = "main_nro"))]
    {
        common::install();
    }

    install_fighters! {
        install;
    }
}
