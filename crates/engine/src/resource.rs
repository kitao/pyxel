global_instance!(Resource, resource);

pub struct Resource {
    //
}

pub fn init_resource() {
    let resource = Resource {};

    set_instance(resource);
}

impl Resource {
    //
}
