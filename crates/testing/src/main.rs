use steamstacks::{callbacks::EncryptedAppTicketResponse, result::SteamResult, steam_api};

fn main() {
    steam_api::init().unwrap();

    let user = steam_api::user();
    let friends = steam_api::friends();
    let apps = steam_api::apps();
    // let utils = steam_api::utils();

    let api_call = user.request_encrypted_app_ticket();

    steam_api::register_call_result::<EncryptedAppTicketResponse, _>(api_call, move |r, _| {
        let result: SteamResult = r.result.into();
        match result {
            SteamResult::Ok => {
                let ticket = user.get_encrypted_app_ticket().unwrap();
                println!("Ticket: {:?}", ticket);
            }
            _ => println!("Error: {:?}", result),
        }
    });

    // println!("{}", utils.get_app_id());

    println!("Hello, {}", friends.get_persona_name());
    // println!("Steam ID: {}", user.get_steam_id());

    println!("Owns app: {}", apps.is_subscribed());

    println!("Owns Subnautica: {}", apps.is_subscribed_app(848450.into()));
    println!("Owns Rust: {}", apps.is_subscribed_app(252490.into()));

    loop {
        steam_api::run_callbacks();
    }

    steam_api::shutdown();

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
}
