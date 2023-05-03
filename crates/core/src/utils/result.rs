use crate::bindings;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

/// Covers errors that can be returned by the steamworks API
///
/// Documentation is based on official documentation which doesn't
/// always explain when an error could be returned or its meaning.
#[derive(Copy, Clone, Debug, Error, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SteamResult {
    #[error("No error")]
    Ok,

    /// Returned if the steamworks API fails to initialize.
    #[error("Failed to init the steamworks API")]
    InitFailed,
    /// Returned if the steamworks API fails to perform an action
    #[error("A generic failure from the steamworks API")]
    Generic,
    /// Returned when steam fails performing a network request
    #[error("There isn't a network connection to steam or it failed to connect")]
    NoConnection,
    /// Return when the password or ticked used is invalid
    #[error("Password or ticket is invalid")]
    InvalidPassword,
    /// Returned when the user is already logged in at another location
    #[error("User logged in elsewhere")]
    LoggedInElsewhere,
    /// Returned when the protocol version is incorrect
    #[error("The protocol version is incorrect")]
    InvalidProtocolVersion,
    /// Returned when a passed parameter is invalid
    #[error("A parameter is invalid")]
    InvalidParameter,
    /// Returned when a file is not found
    #[error("A file was not found")]
    FileNotFound,
    /// Returned when the called method was busy
    ///
    /// No action was performed
    #[error("Method busy")]
    Busy,
    /// Returned when the called object was in an
    /// invalid state
    #[error("Object in invalid state")]
    InvalidState,
    /// Returned when the name is invalid
    #[error("Name is invalid")]
    InvalidName,
    /// Returned when the email is invalid
    #[error("Email is invalid")]
    InvalidEmail,
    /// Returned when the name is not unique
    #[error("Name is not unique")]
    DuplicateName,
    /// Returned when access is denied
    #[error("Access denied")]
    AccessDenied,
    /// Returned when the operation timed out
    #[error("Operation timed out")]
    Timeout,
    /// Returned when the user is VAC2 banned
    #[error("VAC2 banned")]
    Banned,
    /// Returned when the account is not found
    #[error("Account not found")]
    AccountNotFound,
    /// Returned when the passed steam id is invalid
    #[error("SteamID is invalid")]
    InvalidSteamID,
    /// Returned when the requested service in unavailable
    #[error("Requested service is unavailable")]
    ServiceUnavailable,
    /// Returned when the user is not logged on
    #[error("User not logged on")]
    NotLoggedOn,
    /// Returned when the request is pending (e.g. in progress/waiting)
    #[error("Request is pending")]
    Pending,
    /// Returned when encryption or decryption fails
    #[error("Encryption/decryption failed")]
    EncryptionFailure,
    /// Returned when you have insufficient privilege to perform
    /// the action
    #[error("Insufficient privilege")]
    InsufficientPrivilege,
    /// Returned when you have hit the API limits
    #[error("Limit exceeded")]
    LimitExceeded,
    /// Returned when the user's access has been revoked (e.g. revoked
    /// guess passes)
    #[error("Access revoked")]
    Revoked,
    /// Returned when the user's access has expired
    #[error("Access expired")]
    Expired,
    /// Returned when the licence/guest pass has already been redeemed
    #[error("Licence/guest pass already redeemed")]
    AlreadyRedeemed,
    /// Returned when the requested action is a duplicate and has
    /// already occurred.
    ///
    /// The action will be ignored
    #[error("Request is a duplicate")]
    DuplicateRequest,
    /// Returned when all the games in the guest pass are already
    /// owned by the user
    #[error("All games requested already owned")]
    AlreadyOwned,
    /// Returned when the ip address is not found
    #[error("Ip address not found")]
    IPNotFound,
    /// Returned when the change failed to write to the data store
    #[error("Failed to write change")]
    PersistFailed,
    /// Returned when the operation failed to acquire the access lock
    #[error("Failed to acquire access lock")]
    LockingFailed,
    /// Undocumented
    #[error("Logon session replaced")]
    LogonSessionReplaced,
    /// Undocumented
    #[error("Connect failed")]
    ConnectFailed,
    /// Undocumented
    #[error("Handshake failed")]
    HandshakeFailed,
    /// Undocumented
    #[error("IO failure")]
    IOFailure,
    /// Undocumented
    #[error("Remote disconnect")]
    RemoteDisconnect,
    /// Returned when the requested shopping cart wasn't found
    #[error("Failed to find the requested shopping cart")]
    ShoppingCartNotFound,
    /// Returned when the user blocks an action
    #[error("Action blocked")]
    Blocked,
    /// Returned when the target user is ignoring the sender
    #[error("Target is ignoring sender")]
    Ignored,
    /// Returned when nothing matching the request is found
    #[error("No matches found")]
    NoMatch,
    /// Undocumented
    #[error("Account disabled")]
    AccountDisabled,
    /// Returned when the service isn't accepting content changes at
    /// this moment
    #[error("Service is read only")]
    ServiceReadOnly,
    /// Returned when the account doesn't have value so the feature
    /// isn't available
    #[error("Account not featured")]
    AccountNotFeatured,
    /// Allowed to take this action but only because the requester is
    /// an admin
    #[error("Administrator ok")]
    AdministratorOK,
    /// Returned when there is a version mismatch in content transmitted
    /// within the steam protocol
    #[error("Version mismatch with transmitted content")]
    ContentVersion,
    /// Returned when the current CM cannot service the user's request.
    ///
    /// The user should try another.
    #[error("CM cannot service user")]
    TryAnotherCM,
    /// Returned when the user is already logged in elsewhere and the
    /// cached credential login failed.
    #[error("User already logged in, cached login failed")]
    PasswordRequiredToKickSession,
    /// Returned when the user is already logged in elsewhere, you
    /// must wait before trying again
    #[error("User already logged in, please wait")]
    AlreadyLoggedInElsewhere,
    /// Returned when a long running operation (e.g. download) is
    /// suspended/paused.
    #[error("Operation suspended/paused")]
    Suspended,
    /// Returned when an operation is cancelled
    #[error("Operation cancelled")]
    Cancelled,
    /// Returned when an operation is cancelled due to data corruption
    #[error("Operation cancelled due to data corruption")]
    DataCorruption,
    /// Returned when an operation is cancelled due to running out of disk
    /// space
    #[error("Operation cancelled due to the disk being full")]
    DiskFull,
    /// Returned when a remote call or an IPC call failed
    #[error("Remote/IPC call failed")]
    RemoteCallFailed,
    /// Returned when a password could not be verified as its unset
    /// server side
    #[error("Cannot verify unset password")]
    PasswordUnset,
    /// Returned when the external account is not linked to a steam
    /// account
    #[error("External account not linked to steam")]
    ExternalAccountUnlinked,
    /// Returned when the PSN ticket is invalid
    #[error("PSN ticket invalid")]
    PSNTicketInvalid,
    /// Returned when the external account is already linked to a steam
    /// account
    #[error("External account already linked")]
    ExternalAccountAlreadyLinked,
    /// Returned when sync cannot resume due to a file conflict
    #[error("Sync conflict between remote and local files")]
    RemoteFileConflict,
    /// Returned when the requested new password is not legal
    #[error("New password is illegal")]
    IllegalPassword,
    /// Returned when the new value is the same as the previous value
    #[error("New value is the same as old value")]
    SameAsPreviousValue,
    /// Returned when the account logon is denied to 2nd factor authentication
    /// failure
    #[error("2nd factor authentication failed")]
    AccountLogonDenied,
    /// Returned when the requested new password is the same as the
    /// previous password
    #[error("Cannot use old password")]
    CannotUseOldPassword,
    /// Returned when logging in is denied due to an invalid auth code
    #[error("Invalid login auth code")]
    InvalidLoginAuthCode,
    /// Returned when logging in fails due to no email being set for 2nd
    /// factor authentication
    #[error("No email for 2nd factor authentication")]
    AccountLogonDeniedNoMail,
    /// Undocumented
    #[error("Hardware not capable of IPT")]
    HardwareNotCapableOfIPT,
    /// Undocumented
    #[error("IPT init error")]
    IPTInitError,
    /// Returned when a operation fails due to parental control restrictions
    /// for a user
    #[error("Restricted due to parental controls")]
    ParentalControlRestricted,
    /// Returned when a facebook query returns an error
    #[error("Facebook query failed")]
    FacebookQueryError,
    /// Returned when account login is denied due to an expired auth code
    #[error("Login denied due to exipred auth code")]
    ExpiredLoginAuthCode,
    /// Undocumented
    #[error("IP login restriction failed")]
    IPLoginRestrictionFailed,
    /// Undocumented
    #[error("Account locked down")]
    AccountLockedDown,
    /// Undocumented
    #[error("Account logon denied verified email required")]
    AccountLogonDeniedVerifiedEmailRequired,
    /// Undocumented
    #[error("No matching URL")]
    NoMatchingURL,
    /// Returned when something fails to parse/has a missing field
    #[error("Bad response")]
    BadResponse,
    /// Returned when a user cannot complete the action until they
    /// re-enter their password
    #[error("Password re-entry required")]
    RequirePasswordReEntry,
    /// Returned when an entered value is outside the acceptable range
    #[error("Value is out of range")]
    ValueOutOfRange,
    /// Returned when an error happens that the steamworks API didn't
    /// expect to happen
    #[error("Unexpected error")]
    UnexpectedError,
    /// Returned when the requested service is disabled
    #[error("Service disabled")]
    Disabled,
    /// Returned when the set of files submitted to the CEG server
    /// are not valid
    #[error("Submitted files to CEG are invalid")]
    InvalidCEGSubmission,
    /// Returned when the device being used is not allowed to perform
    /// this action
    #[error("Device is restricted from action")]
    RestrictedDevice,
    /// Returned when an action is prevented due to region restrictions
    #[error("Region restrictions prevented action")]
    RegionLocked,
    /// Returned when an action failed due to a temporary rate limit
    #[error("Temporary rate limit exceeded")]
    RateLimitExceeded,
    /// Returned when a account needs to use a two-factor code to login
    #[error("Two-factor authetication required for login")]
    AccountLoginDeniedNeedTwoFactor,
    /// Returned when the item attempting to be accessed has been deleted
    #[error("Item deleted")]
    ItemDeleted,
    /// Returned when the account login failed and you should throttle the
    /// response to the possible attacker
    #[error("Account login denied, throttled")]
    AccountLoginDeniedThrottle,
    /// Returned when the two factor code provided mismatched the expected
    /// one
    #[error("Two-factor code mismatched")]
    TwoFactorCodeMismatch,
    /// Returned when the two factor activation code mismatched the expected
    /// one
    #[error("Two-factor activation code mismatched")]
    TwoFactorActivationCodeMismatch,
    /// Returned when the account has been associated with multiple partners
    #[error("Account associated to multiple partners")]
    AccountAssociatedToMultiplePartners,
    /// Returned when the data wasn't modified
    #[error("Data not modified")]
    NotModified,
    /// Returned when the account doesn't have a mobile device associated with
    /// it
    #[error("No mobile device associated with account")]
    NoMobileDevice,
    /// Returned when the current time is out of range or tolerance
    #[error("Time not synced correctly")]
    TimeNotSynced,
    /// Returned when the sms code failed to validate
    #[error("Sms code validation failed")]
    SmsCodeFailed,
    /// Returned when too many accounts are accessing the requested
    /// resource
    #[error("Account limit exceeded for resource")]
    AccountLimitExceeded,
    /// Returned when there have been too many changes to the account
    #[error("Account activity limit exceeded")]
    AccountActivityLimitExceeded,
    /// Returned when there have been too many changes to the phone
    #[error("Phone activity limited exceeded")]
    PhoneActivityLimitExceeded,
    /// Returned when the refund can not be sent to the payment method
    /// and the steam wallet must be used
    #[error("Must refund to wallet instead of payment method")]
    RefundToWallet,
    /// Returned when steam failed to send an email
    #[error("Email sending failed")]
    EmailSendFailure,
    /// Returned when an action cannot be performed until the payment
    /// has settled
    #[error("Action cannot be performed until payment has settled")]
    NotSettled,
    /// Returned when the user needs to provide a valid captcha
    #[error("Valid captcha required")]
    NeedCaptcha,
    /// Returned when the game server login token owned by the token's owner
    /// been banned
    #[error("Game server login token has been banned")]
    GSLTDenied,
    /// Returned when the game server owner has been denied for other reasons
    /// (account lock, community ban, vac ban, missing phone)
    #[error("Game server owner denied")]
    GSOwnerDenied,
    /// Returned when the type of item attempted to be acted on is invalid
    #[error("Invalid item type")]
    InvalidItemType,
    /// Returned when the IP address has been banned for taking this action
    #[error("IP banned from action")]
    IPBanned,
    /// Returned when the game server login token has expired
    ///
    /// It can be reset for use
    #[error("Game server login token expired")]
    GSLTExpired,
    /// Returned when the user does not have the wallet funds to complete
    /// the action
    #[error("Insufficient wallet funds for action")]
    InsufficientFunds,
    /// Returned when there are too many of the requested action pending
    /// already
    #[error("Too many actions pending")]
    TooManyPending,
    /// Returned when there is no site licenses found
    #[error("No site licenses found")]
    NoSiteLicensesFound,
    /// Returned when WG could not send a response because it exceeded the
    /// max network send size
    #[error("WG network send size exceeded")]
    WGNetworkSendExceeded,
}

impl From<bindings::EResult> for SteamResult {
    fn from(r: bindings::EResult) -> Self {
        match r {
            bindings::EResult::k_EResultOK => SteamResult::Ok,
            bindings::EResult::k_EResultFail => SteamResult::Generic,
            bindings::EResult::k_EResultNoConnection => SteamResult::NoConnection,
            bindings::EResult::k_EResultInvalidPassword => SteamResult::InvalidPassword,
            bindings::EResult::k_EResultLoggedInElsewhere => SteamResult::LoggedInElsewhere,
            bindings::EResult::k_EResultInvalidProtocolVer => SteamResult::InvalidProtocolVersion,
            bindings::EResult::k_EResultInvalidParam => SteamResult::InvalidParameter,
            bindings::EResult::k_EResultFileNotFound => SteamResult::FileNotFound,
            bindings::EResult::k_EResultBusy => SteamResult::Busy,
            bindings::EResult::k_EResultInvalidState => SteamResult::InvalidState,
            bindings::EResult::k_EResultInvalidName => SteamResult::InvalidName,
            bindings::EResult::k_EResultInvalidEmail => SteamResult::InvalidEmail,
            bindings::EResult::k_EResultDuplicateName => SteamResult::DuplicateName,
            bindings::EResult::k_EResultAccessDenied => SteamResult::AccessDenied,
            bindings::EResult::k_EResultTimeout => SteamResult::Timeout,
            bindings::EResult::k_EResultBanned => SteamResult::Banned,
            bindings::EResult::k_EResultAccountNotFound => SteamResult::AccountNotFound,
            bindings::EResult::k_EResultInvalidSteamID => SteamResult::InvalidSteamID,
            bindings::EResult::k_EResultServiceUnavailable => SteamResult::ServiceUnavailable,
            bindings::EResult::k_EResultNotLoggedOn => SteamResult::NotLoggedOn,
            bindings::EResult::k_EResultPending => SteamResult::Pending,
            bindings::EResult::k_EResultEncryptionFailure => SteamResult::EncryptionFailure,
            bindings::EResult::k_EResultInsufficientPrivilege => SteamResult::InsufficientPrivilege,
            bindings::EResult::k_EResultLimitExceeded => SteamResult::LimitExceeded,
            bindings::EResult::k_EResultRevoked => SteamResult::Revoked,
            bindings::EResult::k_EResultExpired => SteamResult::Expired,
            bindings::EResult::k_EResultAlreadyRedeemed => SteamResult::AlreadyRedeemed,
            bindings::EResult::k_EResultDuplicateRequest => SteamResult::DuplicateRequest,
            bindings::EResult::k_EResultAlreadyOwned => SteamResult::AlreadyOwned,
            bindings::EResult::k_EResultIPNotFound => SteamResult::IPNotFound,
            bindings::EResult::k_EResultPersistFailed => SteamResult::PersistFailed,
            bindings::EResult::k_EResultLockingFailed => SteamResult::LockingFailed,
            bindings::EResult::k_EResultLogonSessionReplaced => SteamResult::LogonSessionReplaced,
            bindings::EResult::k_EResultConnectFailed => SteamResult::ConnectFailed,
            bindings::EResult::k_EResultHandshakeFailed => SteamResult::HandshakeFailed,
            bindings::EResult::k_EResultIOFailure => SteamResult::IOFailure,
            bindings::EResult::k_EResultRemoteDisconnect => SteamResult::RemoteDisconnect,
            bindings::EResult::k_EResultShoppingCartNotFound => SteamResult::ShoppingCartNotFound,
            bindings::EResult::k_EResultBlocked => SteamResult::Blocked,
            bindings::EResult::k_EResultIgnored => SteamResult::Ignored,
            bindings::EResult::k_EResultNoMatch => SteamResult::NoMatch,
            bindings::EResult::k_EResultAccountDisabled => SteamResult::AccountDisabled,
            bindings::EResult::k_EResultServiceReadOnly => SteamResult::ServiceReadOnly,
            bindings::EResult::k_EResultAccountNotFeatured => SteamResult::AccountNotFeatured,
            bindings::EResult::k_EResultAdministratorOK => SteamResult::AdministratorOK,
            bindings::EResult::k_EResultContentVersion => SteamResult::ContentVersion,
            bindings::EResult::k_EResultTryAnotherCM => SteamResult::TryAnotherCM,
            bindings::EResult::k_EResultPasswordRequiredToKickSession => {
                SteamResult::PasswordRequiredToKickSession
            }
            bindings::EResult::k_EResultAlreadyLoggedInElsewhere => {
                SteamResult::AlreadyLoggedInElsewhere
            }
            bindings::EResult::k_EResultSuspended => SteamResult::Suspended,
            bindings::EResult::k_EResultCancelled => SteamResult::Cancelled,
            bindings::EResult::k_EResultDataCorruption => SteamResult::DataCorruption,
            bindings::EResult::k_EResultDiskFull => SteamResult::DiskFull,
            bindings::EResult::k_EResultRemoteCallFailed => SteamResult::RemoteCallFailed,
            bindings::EResult::k_EResultPasswordUnset => SteamResult::PasswordUnset,
            bindings::EResult::k_EResultExternalAccountUnlinked => {
                SteamResult::ExternalAccountUnlinked
            }
            bindings::EResult::k_EResultPSNTicketInvalid => SteamResult::PSNTicketInvalid,
            bindings::EResult::k_EResultExternalAccountAlreadyLinked => {
                SteamResult::ExternalAccountAlreadyLinked
            }
            bindings::EResult::k_EResultRemoteFileConflict => SteamResult::RemoteFileConflict,
            bindings::EResult::k_EResultIllegalPassword => SteamResult::IllegalPassword,
            bindings::EResult::k_EResultSameAsPreviousValue => SteamResult::SameAsPreviousValue,
            bindings::EResult::k_EResultAccountLogonDenied => SteamResult::AccountLogonDenied,
            bindings::EResult::k_EResultCannotUseOldPassword => SteamResult::CannotUseOldPassword,
            bindings::EResult::k_EResultInvalidLoginAuthCode => SteamResult::InvalidLoginAuthCode,
            bindings::EResult::k_EResultAccountLogonDeniedNoMail => {
                SteamResult::AccountLogonDeniedNoMail
            }
            bindings::EResult::k_EResultHardwareNotCapableOfIPT => {
                SteamResult::HardwareNotCapableOfIPT
            }
            bindings::EResult::k_EResultIPTInitError => SteamResult::IPTInitError,
            bindings::EResult::k_EResultParentalControlRestricted => {
                SteamResult::ParentalControlRestricted
            }
            bindings::EResult::k_EResultFacebookQueryError => SteamResult::FacebookQueryError,
            bindings::EResult::k_EResultExpiredLoginAuthCode => SteamResult::ExpiredLoginAuthCode,
            bindings::EResult::k_EResultIPLoginRestrictionFailed => {
                SteamResult::IPLoginRestrictionFailed
            }
            bindings::EResult::k_EResultAccountLockedDown => SteamResult::AccountLockedDown,
            bindings::EResult::k_EResultAccountLogonDeniedVerifiedEmailRequired => {
                SteamResult::AccountLogonDeniedVerifiedEmailRequired
            }
            bindings::EResult::k_EResultNoMatchingURL => SteamResult::NoMatchingURL,
            bindings::EResult::k_EResultBadResponse => SteamResult::BadResponse,
            bindings::EResult::k_EResultRequirePasswordReEntry => {
                SteamResult::RequirePasswordReEntry
            }
            bindings::EResult::k_EResultValueOutOfRange => SteamResult::ValueOutOfRange,
            bindings::EResult::k_EResultUnexpectedError => SteamResult::UnexpectedError,
            bindings::EResult::k_EResultDisabled => SteamResult::Disabled,
            bindings::EResult::k_EResultInvalidCEGSubmission => SteamResult::InvalidCEGSubmission,
            bindings::EResult::k_EResultRestrictedDevice => SteamResult::RestrictedDevice,
            bindings::EResult::k_EResultRegionLocked => SteamResult::RegionLocked,
            bindings::EResult::k_EResultRateLimitExceeded => SteamResult::RateLimitExceeded,
            bindings::EResult::k_EResultAccountLoginDeniedNeedTwoFactor => {
                SteamResult::AccountLoginDeniedNeedTwoFactor
            }
            bindings::EResult::k_EResultItemDeleted => SteamResult::ItemDeleted,
            bindings::EResult::k_EResultAccountLoginDeniedThrottle => {
                SteamResult::AccountLoginDeniedThrottle
            }
            bindings::EResult::k_EResultTwoFactorCodeMismatch => SteamResult::TwoFactorCodeMismatch,
            bindings::EResult::k_EResultTwoFactorActivationCodeMismatch => {
                SteamResult::TwoFactorActivationCodeMismatch
            }
            bindings::EResult::k_EResultAccountAssociatedToMultiplePartners => {
                SteamResult::AccountAssociatedToMultiplePartners
            }
            bindings::EResult::k_EResultNotModified => SteamResult::NotModified,
            bindings::EResult::k_EResultNoMobileDevice => SteamResult::NoMobileDevice,
            bindings::EResult::k_EResultTimeNotSynced => SteamResult::TimeNotSynced,
            bindings::EResult::k_EResultSmsCodeFailed => SteamResult::SmsCodeFailed,
            bindings::EResult::k_EResultAccountLimitExceeded => SteamResult::AccountLimitExceeded,
            bindings::EResult::k_EResultAccountActivityLimitExceeded => {
                SteamResult::AccountActivityLimitExceeded
            }
            bindings::EResult::k_EResultPhoneActivityLimitExceeded => {
                SteamResult::PhoneActivityLimitExceeded
            }
            bindings::EResult::k_EResultRefundToWallet => SteamResult::RefundToWallet,
            bindings::EResult::k_EResultEmailSendFailure => SteamResult::EmailSendFailure,
            bindings::EResult::k_EResultNotSettled => SteamResult::NotSettled,
            bindings::EResult::k_EResultNeedCaptcha => SteamResult::NeedCaptcha,
            bindings::EResult::k_EResultGSLTDenied => SteamResult::GSLTDenied,
            bindings::EResult::k_EResultGSOwnerDenied => SteamResult::GSOwnerDenied,
            bindings::EResult::k_EResultInvalidItemType => SteamResult::InvalidItemType,
            bindings::EResult::k_EResultIPBanned => SteamResult::IPBanned,
            bindings::EResult::k_EResultGSLTExpired => SteamResult::GSLTExpired,
            bindings::EResult::k_EResultInsufficientFunds => SteamResult::InsufficientFunds,
            bindings::EResult::k_EResultTooManyPending => SteamResult::TooManyPending,
            bindings::EResult::k_EResultNoSiteLicensesFound => SteamResult::NoSiteLicensesFound,
            bindings::EResult::k_EResultWGNetworkSendExceeded => SteamResult::WGNetworkSendExceeded,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<i64> for SteamResult {
    type Error = InvalidErrorCode;

    fn try_from(r: i64) -> Result<Self, Self::Error> {
        let error = match r {
            x if x == bindings::EResult::k_EResultFail as i64 => SteamResult::Generic,
            x if x == bindings::EResult::k_EResultNoConnection as i64 => SteamResult::NoConnection,
            x if x == bindings::EResult::k_EResultInvalidPassword as i64 => {
                SteamResult::InvalidPassword
            }
            x if x == bindings::EResult::k_EResultLoggedInElsewhere as i64 => {
                SteamResult::LoggedInElsewhere
            }
            x if x == bindings::EResult::k_EResultInvalidProtocolVer as i64 => {
                SteamResult::InvalidProtocolVersion
            }
            x if x == bindings::EResult::k_EResultInvalidParam as i64 => {
                SteamResult::InvalidParameter
            }
            x if x == bindings::EResult::k_EResultFileNotFound as i64 => SteamResult::FileNotFound,
            x if x == bindings::EResult::k_EResultBusy as i64 => SteamResult::Busy,
            x if x == bindings::EResult::k_EResultInvalidState as i64 => SteamResult::InvalidState,
            x if x == bindings::EResult::k_EResultInvalidName as i64 => SteamResult::InvalidName,
            x if x == bindings::EResult::k_EResultInvalidEmail as i64 => SteamResult::InvalidEmail,
            x if x == bindings::EResult::k_EResultDuplicateName as i64 => {
                SteamResult::DuplicateName
            }
            x if x == bindings::EResult::k_EResultAccessDenied as i64 => SteamResult::AccessDenied,
            x if x == bindings::EResult::k_EResultTimeout as i64 => SteamResult::Timeout,
            x if x == bindings::EResult::k_EResultBanned as i64 => SteamResult::Banned,
            x if x == bindings::EResult::k_EResultAccountNotFound as i64 => {
                SteamResult::AccountNotFound
            }
            x if x == bindings::EResult::k_EResultInvalidSteamID as i64 => {
                SteamResult::InvalidSteamID
            }
            x if x == bindings::EResult::k_EResultServiceUnavailable as i64 => {
                SteamResult::ServiceUnavailable
            }
            x if x == bindings::EResult::k_EResultNotLoggedOn as i64 => SteamResult::NotLoggedOn,
            x if x == bindings::EResult::k_EResultPending as i64 => SteamResult::Pending,
            x if x == bindings::EResult::k_EResultEncryptionFailure as i64 => {
                SteamResult::EncryptionFailure
            }
            x if x == bindings::EResult::k_EResultInsufficientPrivilege as i64 => {
                SteamResult::InsufficientPrivilege
            }
            x if x == bindings::EResult::k_EResultLimitExceeded as i64 => {
                SteamResult::LimitExceeded
            }
            x if x == bindings::EResult::k_EResultRevoked as i64 => SteamResult::Revoked,
            x if x == bindings::EResult::k_EResultExpired as i64 => SteamResult::Expired,
            x if x == bindings::EResult::k_EResultAlreadyRedeemed as i64 => {
                SteamResult::AlreadyRedeemed
            }
            x if x == bindings::EResult::k_EResultDuplicateRequest as i64 => {
                SteamResult::DuplicateRequest
            }
            x if x == bindings::EResult::k_EResultAlreadyOwned as i64 => SteamResult::AlreadyOwned,
            x if x == bindings::EResult::k_EResultIPNotFound as i64 => SteamResult::IPNotFound,
            x if x == bindings::EResult::k_EResultPersistFailed as i64 => {
                SteamResult::PersistFailed
            }
            x if x == bindings::EResult::k_EResultLockingFailed as i64 => {
                SteamResult::LockingFailed
            }
            x if x == bindings::EResult::k_EResultLogonSessionReplaced as i64 => {
                SteamResult::LogonSessionReplaced
            }
            x if x == bindings::EResult::k_EResultConnectFailed as i64 => {
                SteamResult::ConnectFailed
            }
            x if x == bindings::EResult::k_EResultHandshakeFailed as i64 => {
                SteamResult::HandshakeFailed
            }
            x if x == bindings::EResult::k_EResultIOFailure as i64 => SteamResult::IOFailure,
            x if x == bindings::EResult::k_EResultRemoteDisconnect as i64 => {
                SteamResult::RemoteDisconnect
            }
            x if x == bindings::EResult::k_EResultShoppingCartNotFound as i64 => {
                SteamResult::ShoppingCartNotFound
            }
            x if x == bindings::EResult::k_EResultBlocked as i64 => SteamResult::Blocked,
            x if x == bindings::EResult::k_EResultIgnored as i64 => SteamResult::Ignored,
            x if x == bindings::EResult::k_EResultNoMatch as i64 => SteamResult::NoMatch,
            x if x == bindings::EResult::k_EResultAccountDisabled as i64 => {
                SteamResult::AccountDisabled
            }
            x if x == bindings::EResult::k_EResultServiceReadOnly as i64 => {
                SteamResult::ServiceReadOnly
            }
            x if x == bindings::EResult::k_EResultAccountNotFeatured as i64 => {
                SteamResult::AccountNotFeatured
            }
            x if x == bindings::EResult::k_EResultAdministratorOK as i64 => {
                SteamResult::AdministratorOK
            }
            x if x == bindings::EResult::k_EResultContentVersion as i64 => {
                SteamResult::ContentVersion
            }
            x if x == bindings::EResult::k_EResultTryAnotherCM as i64 => SteamResult::TryAnotherCM,
            x if x == bindings::EResult::k_EResultPasswordRequiredToKickSession as i64 => {
                SteamResult::PasswordRequiredToKickSession
            }
            x if x == bindings::EResult::k_EResultAlreadyLoggedInElsewhere as i64 => {
                SteamResult::AlreadyLoggedInElsewhere
            }
            x if x == bindings::EResult::k_EResultSuspended as i64 => SteamResult::Suspended,
            x if x == bindings::EResult::k_EResultCancelled as i64 => SteamResult::Cancelled,
            x if x == bindings::EResult::k_EResultDataCorruption as i64 => {
                SteamResult::DataCorruption
            }
            x if x == bindings::EResult::k_EResultDiskFull as i64 => SteamResult::DiskFull,
            x if x == bindings::EResult::k_EResultRemoteCallFailed as i64 => {
                SteamResult::RemoteCallFailed
            }
            x if x == bindings::EResult::k_EResultPasswordUnset as i64 => {
                SteamResult::PasswordUnset
            }
            x if x == bindings::EResult::k_EResultExternalAccountUnlinked as i64 => {
                SteamResult::ExternalAccountUnlinked
            }
            x if x == bindings::EResult::k_EResultPSNTicketInvalid as i64 => {
                SteamResult::PSNTicketInvalid
            }
            x if x == bindings::EResult::k_EResultExternalAccountAlreadyLinked as i64 => {
                SteamResult::ExternalAccountAlreadyLinked
            }
            x if x == bindings::EResult::k_EResultRemoteFileConflict as i64 => {
                SteamResult::RemoteFileConflict
            }
            x if x == bindings::EResult::k_EResultIllegalPassword as i64 => {
                SteamResult::IllegalPassword
            }
            x if x == bindings::EResult::k_EResultSameAsPreviousValue as i64 => {
                SteamResult::SameAsPreviousValue
            }
            x if x == bindings::EResult::k_EResultAccountLogonDenied as i64 => {
                SteamResult::AccountLogonDenied
            }
            x if x == bindings::EResult::k_EResultCannotUseOldPassword as i64 => {
                SteamResult::CannotUseOldPassword
            }
            x if x == bindings::EResult::k_EResultInvalidLoginAuthCode as i64 => {
                SteamResult::InvalidLoginAuthCode
            }
            x if x == bindings::EResult::k_EResultAccountLogonDeniedNoMail as i64 => {
                SteamResult::AccountLogonDeniedNoMail
            }
            x if x == bindings::EResult::k_EResultHardwareNotCapableOfIPT as i64 => {
                SteamResult::HardwareNotCapableOfIPT
            }
            x if x == bindings::EResult::k_EResultIPTInitError as i64 => SteamResult::IPTInitError,
            x if x == bindings::EResult::k_EResultParentalControlRestricted as i64 => {
                SteamResult::ParentalControlRestricted
            }
            x if x == bindings::EResult::k_EResultFacebookQueryError as i64 => {
                SteamResult::FacebookQueryError
            }
            x if x == bindings::EResult::k_EResultExpiredLoginAuthCode as i64 => {
                SteamResult::ExpiredLoginAuthCode
            }
            x if x == bindings::EResult::k_EResultIPLoginRestrictionFailed as i64 => {
                SteamResult::IPLoginRestrictionFailed
            }
            x if x == bindings::EResult::k_EResultAccountLockedDown as i64 => {
                SteamResult::AccountLockedDown
            }
            x if x
                == bindings::EResult::k_EResultAccountLogonDeniedVerifiedEmailRequired as i64 =>
            {
                SteamResult::AccountLogonDeniedVerifiedEmailRequired
            }
            x if x == bindings::EResult::k_EResultNoMatchingURL as i64 => {
                SteamResult::NoMatchingURL
            }
            x if x == bindings::EResult::k_EResultBadResponse as i64 => SteamResult::BadResponse,
            x if x == bindings::EResult::k_EResultRequirePasswordReEntry as i64 => {
                SteamResult::RequirePasswordReEntry
            }
            x if x == bindings::EResult::k_EResultValueOutOfRange as i64 => {
                SteamResult::ValueOutOfRange
            }
            x if x == bindings::EResult::k_EResultUnexpectedError as i64 => {
                SteamResult::UnexpectedError
            }
            x if x == bindings::EResult::k_EResultDisabled as i64 => SteamResult::Disabled,
            x if x == bindings::EResult::k_EResultInvalidCEGSubmission as i64 => {
                SteamResult::InvalidCEGSubmission
            }
            x if x == bindings::EResult::k_EResultRestrictedDevice as i64 => {
                SteamResult::RestrictedDevice
            }
            x if x == bindings::EResult::k_EResultRegionLocked as i64 => SteamResult::RegionLocked,
            x if x == bindings::EResult::k_EResultRateLimitExceeded as i64 => {
                SteamResult::RateLimitExceeded
            }
            x if x == bindings::EResult::k_EResultAccountLoginDeniedNeedTwoFactor as i64 => {
                SteamResult::AccountLoginDeniedNeedTwoFactor
            }
            x if x == bindings::EResult::k_EResultItemDeleted as i64 => SteamResult::ItemDeleted,
            x if x == bindings::EResult::k_EResultAccountLoginDeniedThrottle as i64 => {
                SteamResult::AccountLoginDeniedThrottle
            }
            x if x == bindings::EResult::k_EResultTwoFactorCodeMismatch as i64 => {
                SteamResult::TwoFactorCodeMismatch
            }
            x if x == bindings::EResult::k_EResultTwoFactorActivationCodeMismatch as i64 => {
                SteamResult::TwoFactorActivationCodeMismatch
            }
            x if x == bindings::EResult::k_EResultAccountAssociatedToMultiplePartners as i64 => {
                SteamResult::AccountAssociatedToMultiplePartners
            }
            x if x == bindings::EResult::k_EResultNotModified as i64 => SteamResult::NotModified,
            x if x == bindings::EResult::k_EResultNoMobileDevice as i64 => {
                SteamResult::NoMobileDevice
            }
            x if x == bindings::EResult::k_EResultTimeNotSynced as i64 => {
                SteamResult::TimeNotSynced
            }
            x if x == bindings::EResult::k_EResultSmsCodeFailed as i64 => {
                SteamResult::SmsCodeFailed
            }
            x if x == bindings::EResult::k_EResultAccountLimitExceeded as i64 => {
                SteamResult::AccountLimitExceeded
            }
            x if x == bindings::EResult::k_EResultAccountActivityLimitExceeded as i64 => {
                SteamResult::AccountActivityLimitExceeded
            }
            x if x == bindings::EResult::k_EResultPhoneActivityLimitExceeded as i64 => {
                SteamResult::PhoneActivityLimitExceeded
            }
            x if x == bindings::EResult::k_EResultRefundToWallet as i64 => {
                SteamResult::RefundToWallet
            }
            x if x == bindings::EResult::k_EResultEmailSendFailure as i64 => {
                SteamResult::EmailSendFailure
            }
            x if x == bindings::EResult::k_EResultNotSettled as i64 => SteamResult::NotSettled,
            x if x == bindings::EResult::k_EResultNeedCaptcha as i64 => SteamResult::NeedCaptcha,
            x if x == bindings::EResult::k_EResultGSLTDenied as i64 => SteamResult::GSLTDenied,
            x if x == bindings::EResult::k_EResultGSOwnerDenied as i64 => {
                SteamResult::GSOwnerDenied
            }
            x if x == bindings::EResult::k_EResultInvalidItemType as i64 => {
                SteamResult::InvalidItemType
            }
            x if x == bindings::EResult::k_EResultIPBanned as i64 => SteamResult::IPBanned,
            x if x == bindings::EResult::k_EResultGSLTExpired as i64 => SteamResult::GSLTExpired,
            x if x == bindings::EResult::k_EResultInsufficientFunds as i64 => {
                SteamResult::InsufficientFunds
            }
            x if x == bindings::EResult::k_EResultTooManyPending as i64 => {
                SteamResult::TooManyPending
            }
            x if x == bindings::EResult::k_EResultNoSiteLicensesFound as i64 => {
                SteamResult::NoSiteLicensesFound
            }
            x if x == bindings::EResult::k_EResultWGNetworkSendExceeded as i64 => {
                SteamResult::WGNetworkSendExceeded
            }
            _ => return Err(InvalidErrorCode),
        };
        Ok(error)
    }
}

#[derive(Debug, Error)]
#[error("error code could not be converted to rust enum")]
pub struct InvalidErrorCode;
