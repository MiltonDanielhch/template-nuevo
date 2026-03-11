use crate::domain::entities::user::User;
use crate::domain::value_objects::{Email, PasswordHash, UserId};
use crate::proto_generated::auth::v1::{
    LoginUserResponse, RegisterUserResponse, User as ProtoUser,
};
use chrono::{DateTime, Utc};

pub struct ProtoMapper;

impl ProtoMapper {
    pub fn to_domain_user(proto: ProtoUser) -> Result<User, String> {
        let user_id = UserId::new_from_string(proto.id.value)
            .map_err(|e| format!("Invalid user ID: {}", e))?;

        let email = Email::parse(proto.email).map_err(|e| format!("Invalid email: {}", e))?;

        let password_hash = PasswordHash::new(proto.password_hash)
            .map_err(|e| format!("Invalid password hash: {}", e))?;

        let created_at = proto_timestamp_to_datetime(&proto.created_at)?;
        let updated_at = proto_timestamp_to_datetime(&proto.updated_at)?;
        let deleted_at = proto
            .deleted_at
            .and_then(|t| proto_timestamp_to_datetime(&t).ok());

        let persistence_data = crate::domain::entities::user::UserPersistenceData {
            id: user_id,
            email,
            password_hash,
            username: Some(proto.username).filter(|s| !s.is_empty()),
            avatar_url: Some(proto.avatar_url).filter(|s| !s.is_empty()),
            email_verified: proto.email_verified,
            created_at,
            updated_at,
            deleted_at,
        };

        Ok(User::new_from_persistence(persistence_data))
    }

    pub fn from_domain_user(user: &User) -> ProtoUser {
        let created_at = user.created_at();
        let updated_at = user.updated_at();
        let deleted_at = user.deleted_at();

        ProtoUser {
            id: crate::proto_generated::common::v1::Uuid {
                value: user.id().to_string(),
            },
            username: user.username().clone().unwrap_or_default(),
            email: user.email().to_string(),
            password_hash: user.password_hash().to_string(),
            avatar_url: user.avatar_url().clone().unwrap_or_default(),
            email_verified: user.is_email_verified(),
            created_at: datetime_to_proto_timestamp(created_at),
            updated_at: datetime_to_proto_timestamp(updated_at),
            deleted_at: deleted_at.as_ref().map(datetime_to_proto_timestamp),
        }
    }

    pub fn map_register_response(user: User) -> RegisterUserResponse {
        RegisterUserResponse {
            user: Some(Self::from_domain_user(&user)),
        }
    }

    pub fn map_login_response(session_token: String, user: User) -> LoginUserResponse {
        LoginUserResponse {
            session_token,
            user: Some(Self::from_domain_user(&user)),
        }
    }
}

fn proto_timestamp_to_datetime(ts: &prost_types::Timestamp) -> Result<DateTime<Utc>, String> {
    let secs = ts.seconds;
    let nanos = ts.nanos as u32;
    DateTime::from_timestamp(secs, nanos).ok_or_else(|| "Invalid timestamp".to_string())
}

fn datetime_to_proto_timestamp(dt: &DateTime<Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}
