use steamstacks::{friends::ImageSize, steam_api, FriendFlags};

fn main() {
    steam_api::init().unwrap();

    let user = steam_api::user();
    let friends = steam_api::friends();
    let apps = steam_api::apps();
    let utils = steam_api::utils();

    // user.request_encrypted_app_ticket();
    user.get_encrypted_app_ticket();

    println!("{}", utils.get_app_id());

    println!("Hello, {}", friends.get_persona_name());
    println!("Steam ID: {}", user.get_steam_id());
    println!("Owns app: {}", apps.is_subscribed());

    println!("Owns Subnautica: {}", apps.is_subscribed_app(848450.into()));
    println!("Owns Rust: {}", apps.is_subscribed_app(252490.into()));

    loop {
        steam_api::run_callbacks();
    }

    // println!(
    //     "TESTING: {:?}",
    //     friends.get_avatar(user.get_steam_id(), ImageSize::SMALL)
    // );

    // let list = friends.get_friends(FriendFlags::IMMEDIATE);
    // for friend in list {
    //     println!(
    //         "Friend: {}, id: {}",
    //         friend.get_persona_name(),
    //         friend.get_steam_id()
    //     );
    // }

    steam_api::shutdown();
}
