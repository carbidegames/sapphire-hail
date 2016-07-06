mod home;

use clockwork::Routes;

pub fn register(routes: &mut Routes) {
    home::register(routes);
}
