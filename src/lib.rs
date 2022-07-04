//use log::info;

pub mod type_print{
    use log::info;
    pub fn print_type_of<T>(_: &T) {
        info!("concrete-type:{}", std::any::type_name::<T>());
        println!("concrete-type:{}", std::any::type_name::<T>());
    }
}






#[cfg(test)]
mod tests {
    //use crate::type_print::print_type_of;
    use crate::type_print::print_type_of;
    #[test]
    fn print_type(){
        let x = 123;
        print_type_of(&x);
    }
}


