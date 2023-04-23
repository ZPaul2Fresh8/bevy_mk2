pub mod mod_fighter{
    use bevy::prelude::Image;

    pub struct StructFighter{
        // basic info
        name: String,
        locked: bool,
        
        // bio
        bio: String,
        bio_image: Image,
    
        // ending
        ending1: String,
        ending2: String,
        ending1_image: Image,
        ending2_image: Image,
    
        // walk attributes
        walk_vel_fwd: f32,
        walk_vel_bwd: f32,
        walk_ani_speed_fwd: f32,
        walk_ani_speed_bwd: f32,
    }
    impl StructFighter{
        pub fn walk_forward(){
            
        }
    }
}