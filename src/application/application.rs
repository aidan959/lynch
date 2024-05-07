

struct ApplicationBuilder {
    name: String,
    width: u32,
    height: u32,
    initialize_base_services  : bool
}
impl ApplicationBuilder {
    fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            name: "Frost Application".to_string(),
            width: 800,
            height: 600,
            initialize_base_services: true
        }
    }
    fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
    fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }
    fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
    fn initialize_base_services(&mut self, initialize_base_services: bool) -> &mut Self {
        self.initialize_base_services = initialize_base_services;
        self
    }

}

trait Application {
    fn create(&mut self, builder : &ApplicationBuilder);
    fn run(&mut self, builder : &ApplicationBuilder);
    
}