pub trait Service {
    fn init(&mut self, configuration: Option<&mut dyn std::any::Any>) {}
    fn shutdown(&mut self) {}
}

macro_rules! lynch_declare_service {
    ($type:ty) => {
        pub fn instance() -> &'static mut $type {
            use std::sync::Once;
            static mut SINGLETON: Option<$type> = None;
            static INIT: Once = Once::new();

            unsafe {
                INIT.call_once(|| {
                    SINGLETON = Some(<$type>::default());
                });
                SINGLETON.as_mut().unwrap()
            }
        }
    };
}