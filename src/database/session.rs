use crate::{config::ENV, util::now};
use ipinfo::{IpInfo, IpInfoConfig};
use prelude::Decimal;
use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
    entity::*,
};
use woothee::parser::Parser;

pub struct Device {
    pub vendor: String,
    pub model: String,
}

pub struct OS {
    pub name: String,
    pub version: String,
}

pub struct Browser {
    pub name: String,
    pub version: String,
}

pub async fn create(
    db: &DatabaseConnection,
    id: &str,
    user_id: &str,
    ip_address: &str,
    user_agent: &str,
) -> Result<(), DbErr> {
    let now = now();
    let exp: i32 = (now + ENV.refresh_token_expiration)
        .try_into()
        .map_err(|_| DbErr::Custom(String::from("failed to convert expiration to i32")))?;

    let mut lat: Option<f32> = None;
    let mut lon: Option<f32> = None;
    let mut country: Option<String> = None;
    let mut city: Option<String> = None;
    let mut region: Option<String> = None;
    let mut timezone: Option<String> = None;
    let mut map_url: Option<String> = None;

    let mut device_vendor: Option<String> = None;
    let mut device_model: Option<String> = None;
    let mut os_name: Option<String> = None;
    let mut os_version: Option<String> = None;
    let mut browser_name: Option<String> = None;
    let mut browser_version: Option<String> = None;

    let parser = Parser::new();
    let result = parser.parse(user_agent);

    if let Some(result) = result {
        browser_name = Some(result.name.to_owned());
        browser_version = Some(result.version.to_owned());
        os_name = Some(result.os.to_owned());
        os_version = Some(result.os_version.to_string());
        device_vendor = Some(result.category.to_owned());
        device_model = Some(result.vendor.to_owned());
    }

    if ip_address != "127.0.0.1" && !ip_address.is_empty() {
        let config = IpInfoConfig {
            token: Some((*ENV.ipinfo_api_key).to_owned()),
            ..Default::default()
        };

        let mut ipinfo = IpInfo::new(config).map_err(|err| DbErr::Custom(err.to_string()))?;
        let result = ipinfo
            .lookup(ip_address)
            .await
            .map_err(|err| DbErr::Custom(err.to_string()))?;

        let mut split = result.loc.split(',');

        lat = split.next().and_then(|lat| lat.parse().ok()).or(Some(0.0));
        lon = split.next().and_then(|lon| lon.parse().ok()).or(Some(0.0));

        country = Some(result.country);
        city = Some(result.city);
        region = Some(result.region);
        timezone = result.timezone;

        map_url = Some(format!(
            "https://www.openstreetmap.org/?mlat={}&mlon={}",
            lat.unwrap_or(0.0),
            lon.unwrap_or(0.0),
        ));
    }

    let lat = lat
        .map(|lat| {
            Decimal::from_f32_retain(lat)
                .ok_or_else(|| DbErr::Custom(String::from("failed to convert lat to decimal")))
        })
        .transpose()?;
    let lon = lon
        .map(|lon| {
            Decimal::from_f32_retain(lon)
                .ok_or_else(|| DbErr::Custom(String::from("failed to convert lon to decimal")))
        })
        .transpose()?;

    let session = entity::session::ActiveModel {
        id: Set(id.to_owned()),
        user_id: Set(user_id.to_owned()),
        exp: Set(exp),
        login_at: Set(now.try_into().unwrap()),
        lat: Set(lat),
        lon: Set(lon),
        ip_address: Set(ip_address.to_owned()),
        device_vendor: Set(device_vendor),
        device_model: Set(device_model),
        os_name: Set(os_name),
        os_version: Set(os_version),
        browser_name: Set(browser_name),
        borwser_version: Set(browser_version),
        country: Set(country),
        city: Set(city),
        region: Set(region),
        timezone: Set(timezone),
        map_url: Set(map_url),
    };
    let _: entity::session::Model = session.insert(db).await?;

    Ok(())
}

pub async fn delete(db: &DatabaseConnection, rjti: &str) -> Result<(), DbErr> {
    let _ = entity::session::Entity::delete_by_id(rjti).exec(db).await?;

    Ok(())
}

pub async fn delete_expired_user_sessions(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<(), DbErr> {
    let now: i64 = now().try_into().unwrap();

    entity::session::Entity::delete_many()
        .filter(
            Condition::all()
                .add(entity::session::Column::UserId.eq(user_id))
                .add(entity::session::Column::Exp.lte(now + 60)),
        )
        .exec(db)
        .await?;

    Ok(())
}
