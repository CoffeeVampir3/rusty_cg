#[macro_export]
macro_rules! register_types_and_components {
    ($app:expr, $($t:ty),+) => {
        $app
            $(.register_type::<$t>())+
            $(.register_component_as::<dyn CardComponent, $t>())+
    }
}
