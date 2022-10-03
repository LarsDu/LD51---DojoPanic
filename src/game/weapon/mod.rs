use bevy::prelude::*;

mod bullet;
use bullet::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugin(BulletPlugin);
    }
}