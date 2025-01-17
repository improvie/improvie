macro_rules! def_repositories_module {
    (
        $module:ident, $struct:ident $name:ident
        { $($variable:ident: $impl:ident = $repository:ident,)* }
    ) => {
        $struct $name {
            $($variable: $impl,)*
        }

        impl $name {
            fn new(db: DbPool) -> Self {
                Self {
                    $($variable: $impl::new(db.clone()),)*
                }
            }
        }

        impl improvie_domain::modules::RepositoriesModule for $module {
            $(
                type $repository = $impl;
                fn $variable(&self) -> &Self::$repository {
                    &self.0.$variable
                }
            )*
        }
    };
}

pub(crate) use def_repositories_module;
