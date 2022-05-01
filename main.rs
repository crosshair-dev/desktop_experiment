
use std::{ops::Deref, str::FromStr, process::Command};

use reqwest::{blocking::Client, header::{self, HeaderMap, HeaderValue}};
use serde_json::Value;


fn main() {
    let lockfile_path = format!("{}{}", std::env::var("LOCALAPPDATA").unwrap(), r"\Riot Games\Riot Client\Config\lockfile");
    let lockfile_content = std::fs::read_to_string(lockfile_path).unwrap();

    println!("lockfile_content = {:?}", lockfile_content);

    let values = lockfile_content.split(":").collect::<Vec<&str>>();
    let client = values[0];
    let pid = values[1];
    let port = values[2];
    let lockfile_password = values[3];
    let protocol = values[4];

    let base64_encoded_lock_file_password = base64::encode(format!("riot:{lockfile_password}"));
    let authorization_value = format!("Basic {base64_encoded_lock_file_password}");
    let url: String = format!("https://127.0.0.1:{port}/chat/v1/session");

    println!("url = {:?}", url);
    println!("authorization_value = {:?}", authorization_value);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Could not build client");
    
    let body = client
        .get(url)
        .header("Authorization", &authorization_value)
        .send()
        .expect("Request failed");


    let text = body.text().unwrap();
    println!("body = {:?}", text);
    
    let result: Value = serde_json::from_str(text.as_str()).unwrap();

    let puuid = result["puuid"].as_str().unwrap();
    println!("puuid = {:?}", puuid);


    // Entitlement

    let url: String = format!("https://127.0.0.1:{port}/entitlements/v1/token");

    println!("url = {:?}", url);
    
    let body = client
        .get(url)
        .header("Authorization", &authorization_value)
        .send()
        .expect("Request failed");

    let text = body.text().unwrap();
    println!("body = {:?}", text);


    let result: Value = serde_json::from_str(text.as_str()).unwrap();

    let token = result["accessToken"].as_str().unwrap();
    let base64_encoded_token = base64::encode(token);

    let entitlement = result["token"].as_str().unwrap();

    println!("token = {:?}", token);
    println!("base64_token = {:?}", base64_encoded_token);
    println!("entitlement = {:?}", entitlement);

    //

    let client_platform = "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9";
    let region = "na";

    let client_version = "release-04.07-shipping-15-699063";
    let context: Context = Context::new(token.to_string(), entitlement.to_string(), client_version.to_string(), client_platform.to_string(), region.to_string());

    // Get player MMR

    let tier = get_player_tier(&client, &context, puuid);

    println!("tier = {:?}", tier);

    // let url: String = format!("https://pd.{region}.a.pvp.net/mmr/v1/players/{puuid}");
    // println!("url = {:?}", url);

    let auth_value = format!("Bearer {token}");
    // println!("auth_value = {:?}", auth_value);

    // let body = client
    //     .get(url)
    //     .header("Authorization", &auth_value)
    //     .header("X-Riot-Entitlements-JWT", entitlement)
    //     .header("X-Riot-ClientVersion", "release-04.07-shipping-15-699063")
    //     .header("X-Riot-ClientPlatform", client_platform)
    //     .send()
    //     .expect("Request failed");

    // let text = body.text().unwrap();
    // println!("body = {:?}", text);


    // Get current match

    let url: String = format!("https://glz-{region}-1.{region}.a.pvp.net/core-game/v1/players/{puuid}");
    println!("url = {:?}", url);

    let body = client
        .get(url)
        .header("Authorization", &auth_value)
        .header("X-Riot-Entitlements-JWT", entitlement)
        .send()
        .expect("Request failed");


    let text = body.text().unwrap();
    println!("body = {:?}", text);

    let result: Value = serde_json::from_str(text.as_str()).unwrap();

    let match_id= result["MatchID"].as_str().unwrap();
    println!("match_id = {:?}", match_id);


    // Get current match

    let url: String = format!("https://glz-{region}-1.{region}.a.pvp.net/core-game/v1/matches/{match_id}");
    println!("url = {:?}", url);

    let body = client
        .get(url)
        .header("Authorization", &auth_value)
        .header("X-Riot-Entitlements-JWT", entitlement)
        .send()
        .expect("Request failed");


    let text = body.text().unwrap();
    println!("body = {:?}", text);

    let result: Value = serde_json::from_str(text.as_str()).unwrap();

    let players: Vec<&str> = result["Players"].as_array().unwrap()
        .iter()
        .map(|player| player["Subject"].as_str().unwrap())
        .collect();
    
    println!("players = {:?}", players);


    // names

    let current_season = "d929bc38-4ab6-7da4-94f0-ee84f8ac141e";

    // names

    let url: String = format!("https://pd.{region}.a.pvp.net/name-service/v2/players");
    println!("url = {:?}", url);

    let player_json_body = serde_json::to_string(&players).unwrap();

    let body = client
        .put(url)
        .header("Content-Type", "application/json")
        .body(player_json_body)
        .send()
        .expect("Request failed");

    let text = body.text().unwrap();
    println!("body = {:?}", text);


    let result: Value = serde_json::from_str(text.as_str()).unwrap();

    let players = result.as_array().unwrap();

    // ranks
    let display_names = [
        "Unrated",
        "UNUSED1",
        "UNUSED2",
        "IRON 1",
        "IRON 2",
        "IRON 3",
        "BRONZE 1",
        "BRONZE 2",
        "BRONZE 3",
        "SILVER 1",
        "SILVER 2",
        "SILVER 3",
        "GOLD 1",
        "GOLD 2",
        "GOLD 3",
        "PLATINUM 1",
        "PLATINUM 2",
        "PLATINUM 3",
        "DIAMOND 1",
        "DIAMOND 2",
        "DIAMOND 3",
        "IMMORTAL 1",
        "IMMORTAL 2",
        "IMMORTAL 3",
        "RADIANT"
    ];


    println!("{:?}", players.len());
    for player in players.iter() {
        let player_puuid = player["Subject"].as_str().unwrap();
        let name = player["GameName"].as_str().unwrap();
        let tagline = player["TagLine"].as_str().unwrap();

        let tier = get_player_tier(&client, &context, player_puuid);

        if let Option::Some(tier) = tier {
            println!("{}#{} = {:?}", name, tagline, display_names[tier as usize])
        } else {
            println!("{}#{} = {:?}", name, tagline, "Unknown/Unrated")
        }
    }

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

#[derive(Debug)]
struct Context {
    token: String,
    entitlement: String,
    client_version: String,
    client_platform: String,
    region: String,
    authorization_headers: HeaderMap
}

impl Context {
    fn new(token: String, entitlement: String, client_version: String, client_platform: String, region: String) -> Context {

        let mut authorization_headers = HeaderMap::new();
        authorization_headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {token}").as_str()).unwrap());
        authorization_headers.insert("X-Riot-Entitlements-JWT", HeaderValue::from_str(entitlement.as_str()).unwrap());
        authorization_headers.insert("X-Riot-ClientVersion", HeaderValue::from_str(client_version.as_str()).unwrap());
        authorization_headers.insert("X-Riot-ClientPlatform", HeaderValue::from_str(client_platform.as_str()).unwrap());


        Context { 
            token, 
            entitlement,
            client_version,
            client_platform,
            region,
            authorization_headers
        }
    }
}


fn get_player_tier(client: &Client, ctx: &Context, puuid: &str) -> Option<i64> {
    let region = ctx.region.as_str();
    let url: String = format!("https://pd.{region}.a.pvp.net/mmr/v1/players/{puuid}");


    let response = client
        .get(url)
        .headers(ctx.authorization_headers.clone())
        .send()
        .expect("Request failed");


    let current_season = "d929bc38-4ab6-7da4-94f0-ee84f8ac141e";

    let text = response.text().unwrap();

    let result: Value = serde_json::from_str(text.as_str()).unwrap();

    result["QueueSkills"]["competitive"]["SeasonalInfoBySeasonID"][current_season]["CompetitiveTier"].as_i64()
}