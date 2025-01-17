macro_rules! def_modules {
    (
        $repository:ident, $pub:ident $struct:ident $name:ident
        { $($variable:ident: $usecase:ident,)* }
    ) => {
        $pub $struct $name {
            $($variable: $usecase<$repository>,)*
        }

        impl $name {
            $pub fn new(db: DbPool) -> Self {
                let repository = $repository::new(db);
                Self {
                    $($variable: $usecase::new(repository.clone()),)*
                }
            }

            $(
                $pub fn $variable(&self) -> &$usecase<$repository> {
                    &self.$variable
                }
            )*
        }
    };
}

pub(crate) use def_modules;
