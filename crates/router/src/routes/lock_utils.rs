use router_env::Flow;

#[derive(Clone, Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum ApiIdentifier {
    Payments,
    Refunds,
    Webhooks,
    Organization,
    MerchantAccount,
    MerchantConnector,
    Configs,
    Customers,
    Ephemeral,
    Health,
    Mandates,
    PaymentMethods,
    PaymentMethodAuth,
    Payouts,
    Disputes,
    CardsInfo,
    Files,
    Cache,
    Profile,
    Verification,
    ApiKeys,
    PaymentLink,
    Routing,
<<<<<<< HEAD
    Subscription,
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    Blocklist,
    Forex,
    RustLockerMigration,
    Gsm,
    Role,
    User,
    UserRole,
    ConnectorOnboarding,
    Recon,
    AiWorkflow,
    Poll,
    ApplePayCertificatesMigration,
    Relay,
    Documentation,
    CardNetworkTokenization,
    Hypersense,
    PaymentMethodSession,
    ProcessTracker,
    Authentication,
    Proxy,
    ProfileAcquirer,
    ThreeDsDecisionRule,
    GenericTokenization,
<<<<<<< HEAD
    RecoveryRecovery,
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
}

impl From<Flow> for ApiIdentifier {
    fn from(flow: Flow) -> Self {
        match flow {
            Flow::MerchantsAccountCreate
            | Flow::MerchantsAccountRetrieve
            | Flow::MerchantsAccountUpdate
            | Flow::MerchantsAccountDelete
            | Flow::MerchantTransferKey
            | Flow::MerchantAccountList
            | Flow::EnablePlatformAccount => Self::MerchantAccount,
<<<<<<< HEAD
            Flow::OrganizationCreate | Flow::OrganizationRetrieve | Flow::OrganizationUpdate => {
                Self::Organization
            }
=======

            Flow::OrganizationCreate | Flow::OrganizationRetrieve | Flow::OrganizationUpdate => {
                Self::Organization
            }

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::RoutingCreateConfig
            | Flow::RoutingLinkConfig
            | Flow::RoutingUnlinkConfig
            | Flow::RoutingRetrieveConfig
            | Flow::RoutingRetrieveActiveConfig
            | Flow::RoutingRetrieveDefaultConfig
            | Flow::RoutingRetrieveDictionary
            | Flow::RoutingUpdateConfig
            | Flow::RoutingUpdateDefaultConfig
            | Flow::RoutingDeleteConfig
            | Flow::DecisionManagerDeleteConfig
            | Flow::DecisionManagerRetrieveConfig
            | Flow::ToggleDynamicRouting
            | Flow::CreateDynamicRoutingConfig
            | Flow::UpdateDynamicRoutingConfigs
            | Flow::DecisionManagerUpsertConfig
            | Flow::RoutingEvaluateRule
            | Flow::DecisionEngineRuleMigration
            | Flow::VolumeSplitOnRoutingType
            | Flow::DecisionEngineDecideGatewayCall
            | Flow::DecisionEngineGatewayFeedbackCall => Self::Routing,
<<<<<<< HEAD
            Flow::CreateSubscription
            | Flow::ConfirmSubscription
            | Flow::CreateAndConfirmSubscription
            | Flow::GetSubscription
            | Flow::UpdateSubscription
            | Flow::GetSubscriptionEstimate
            | Flow::GetPlansForSubscription => Self::Subscription,
            Flow::RetrieveForexFlow => Self::Forex,
=======

            Flow::RetrieveForexFlow => Self::Forex,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::AddToBlocklist => Self::Blocklist,
            Flow::DeleteFromBlocklist => Self::Blocklist,
            Flow::ListBlocklist => Self::Blocklist,
            Flow::ToggleBlocklistGuard => Self::Blocklist,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::MerchantConnectorsCreate
            | Flow::MerchantConnectorsRetrieve
            | Flow::MerchantConnectorsUpdate
            | Flow::MerchantConnectorsDelete
            | Flow::MerchantConnectorsList => Self::MerchantConnector,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::ConfigKeyCreate
            | Flow::ConfigKeyFetch
            | Flow::ConfigKeyUpdate
            | Flow::ConfigKeyDelete
            | Flow::CreateConfigKey => Self::Configs,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::CustomersCreate
            | Flow::CustomersRetrieve
            | Flow::CustomersUpdate
            | Flow::CustomersDelete
            | Flow::CustomersGetMandates
<<<<<<< HEAD
            | Flow::CustomersList
            | Flow::CustomersListWithConstraints => Self::Customers,
            Flow::EphemeralKeyCreate | Flow::EphemeralKeyDelete => Self::Ephemeral,
            Flow::DeepHealthCheck | Flow::HealthCheck => Self::Health,
            Flow::MandatesRetrieve | Flow::MandatesRevoke | Flow::MandatesList => Self::Mandates,
=======
            | Flow::CustomersList => Self::Customers,

            Flow::EphemeralKeyCreate | Flow::EphemeralKeyDelete => Self::Ephemeral,

            Flow::DeepHealthCheck | Flow::HealthCheck => Self::Health,
            Flow::MandatesRetrieve | Flow::MandatesRevoke | Flow::MandatesList => Self::Mandates,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::PaymentMethodsCreate
            | Flow::PaymentMethodsMigrate
            | Flow::PaymentMethodsBatchUpdate
            | Flow::PaymentMethodsList
            | Flow::CustomerPaymentMethodsList
            | Flow::GetPaymentMethodTokenData
            | Flow::PaymentMethodsRetrieve
            | Flow::PaymentMethodsUpdate
            | Flow::PaymentMethodsDelete
<<<<<<< HEAD
            | Flow::NetworkTokenStatusCheck
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            | Flow::PaymentMethodCollectLink
            | Flow::ValidatePaymentMethod
            | Flow::ListCountriesCurrencies
            | Flow::DefaultPaymentMethodsSet
            | Flow::PaymentMethodSave
            | Flow::TotalPaymentMethodCount => Self::PaymentMethods,
<<<<<<< HEAD
            Flow::PmAuthLinkTokenCreate | Flow::PmAuthExchangeToken => Self::PaymentMethodAuth,
=======

            Flow::PmAuthLinkTokenCreate | Flow::PmAuthExchangeToken => Self::PaymentMethodAuth,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::PaymentsCreate
            | Flow::PaymentsRetrieve
            | Flow::PaymentsRetrieveForceSync
            | Flow::PaymentsUpdate
            | Flow::PaymentsConfirm
            | Flow::PaymentsCapture
            | Flow::PaymentsCancel
            | Flow::PaymentsCancelPostCapture
            | Flow::PaymentsApprove
            | Flow::PaymentsReject
            | Flow::PaymentsSessionToken
            | Flow::PaymentsStart
            | Flow::PaymentsList
            | Flow::PaymentsFilters
            | Flow::PaymentsAggregate
            | Flow::PaymentsRedirect
            | Flow::PaymentsIncrementalAuthorization
<<<<<<< HEAD
            | Flow::PaymentsExtendAuthorization
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            | Flow::PaymentsExternalAuthentication
            | Flow::PaymentsAuthorize
            | Flow::GetExtendedCardInfo
            | Flow::PaymentsCompleteAuthorize
            | Flow::PaymentsManualUpdate
            | Flow::SessionUpdateTaxCalculation
            | Flow::PaymentsConfirmIntent
            | Flow::PaymentsCreateIntent
            | Flow::PaymentsGetIntent
<<<<<<< HEAD
            | Flow::GiftCardBalanceCheck
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            | Flow::PaymentsPostSessionTokens
            | Flow::PaymentsUpdateMetadata
            | Flow::PaymentsUpdateIntent
            | Flow::PaymentsCreateAndConfirmIntent
            | Flow::PaymentStartRedirection
            | Flow::ProxyConfirmIntent
            | Flow::PaymentsRetrieveUsingMerchantReferenceId
            | Flow::PaymentAttemptsList
<<<<<<< HEAD
            | Flow::RecoveryPaymentsCreate
            | Flow::PaymentsSubmitEligibility => Self::Payments,
=======
            | Flow::RecoveryPaymentsCreate => Self::Payments,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::PayoutsCreate
            | Flow::PayoutsRetrieve
            | Flow::PayoutsUpdate
            | Flow::PayoutsCancel
            | Flow::PayoutsFulfill
            | Flow::PayoutsList
            | Flow::PayoutsFilter
            | Flow::PayoutsAccounts
            | Flow::PayoutsConfirm
            | Flow::PayoutLinkInitiate => Self::Payouts,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::RefundsCreate
            | Flow::RefundsRetrieve
            | Flow::RefundsRetrieveForceSync
            | Flow::RefundsUpdate
            | Flow::RefundsList
            | Flow::RefundsFilters
            | Flow::RefundsAggregate
            | Flow::RefundsManualUpdate => Self::Refunds,
            Flow::Relay | Flow::RelayRetrieve => Self::Relay,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::FrmFulfillment
            | Flow::IncomingWebhookReceive
            | Flow::IncomingRelayWebhookReceive
            | Flow::WebhookEventInitialDeliveryAttemptList
            | Flow::WebhookEventDeliveryAttemptList
            | Flow::WebhookEventDeliveryRetry
            | Flow::RecoveryIncomingWebhookReceive
            | Flow::IncomingNetworkTokenWebhookReceive => Self::Webhooks,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::ApiKeyCreate
            | Flow::ApiKeyRetrieve
            | Flow::ApiKeyUpdate
            | Flow::ApiKeyRevoke
            | Flow::ApiKeyList => Self::ApiKeys,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::DisputesRetrieve
            | Flow::DisputesList
            | Flow::DisputesFilters
            | Flow::DisputesEvidenceSubmit
            | Flow::AttachDisputeEvidence
            | Flow::RetrieveDisputeEvidence
            | Flow::DisputesAggregate
            | Flow::DeleteDisputeEvidence => Self::Disputes,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::CardsInfo
            | Flow::CardsInfoCreate
            | Flow::CardsInfoUpdate
            | Flow::CardsInfoMigrate => Self::CardsInfo,
<<<<<<< HEAD
            Flow::CreateFile | Flow::DeleteFile | Flow::RetrieveFile => Self::Files,
            Flow::CacheInvalidate => Self::Cache,
=======

            Flow::CreateFile | Flow::DeleteFile | Flow::RetrieveFile => Self::Files,

            Flow::CacheInvalidate => Self::Cache,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::ProfileCreate
            | Flow::ProfileUpdate
            | Flow::ProfileRetrieve
            | Flow::ProfileDelete
            | Flow::ProfileList
            | Flow::ToggleExtendedCardInfo
            | Flow::ToggleConnectorAgnosticMit => Self::Profile,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::PaymentLinkRetrieve
            | Flow::PaymentLinkInitiate
            | Flow::PaymentSecureLinkInitiate
            | Flow::PaymentLinkList
            | Flow::PaymentLinkStatus => Self::PaymentLink,
<<<<<<< HEAD
            Flow::Verification => Self::Verification,
=======

            Flow::Verification => Self::Verification,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::RustLockerMigration => Self::RustLockerMigration,
            Flow::GsmRuleCreate
            | Flow::GsmRuleRetrieve
            | Flow::GsmRuleUpdate
            | Flow::GsmRuleDelete => Self::Gsm,
<<<<<<< HEAD
            Flow::ApplePayCertificatesMigration => Self::ApplePayCertificatesMigration,
=======

            Flow::ApplePayCertificatesMigration => Self::ApplePayCertificatesMigration,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::UserConnectAccount
            | Flow::UserSignUp
            | Flow::UserSignIn
            | Flow::Signout
            | Flow::ChangePassword
            | Flow::SetDashboardMetadata
            | Flow::GetMultipleDashboardMetadata
            | Flow::VerifyPaymentConnector
            | Flow::InternalUserSignup
            | Flow::TenantUserCreate
            | Flow::SwitchOrg
            | Flow::SwitchMerchantV2
            | Flow::SwitchProfile
            | Flow::CreatePlatformAccount
            | Flow::UserOrgMerchantCreate
            | Flow::UserMerchantAccountCreate
            | Flow::GenerateSampleData
            | Flow::DeleteSampleData
            | Flow::GetUserDetails
            | Flow::GetUserRoleDetails
            | Flow::ForgotPassword
            | Flow::ResetPassword
            | Flow::RotatePassword
            | Flow::InviteMultipleUser
            | Flow::ReInviteUser
            | Flow::UserSignUpWithMerchantId
            | Flow::VerifyEmail
            | Flow::AcceptInviteFromEmail
            | Flow::VerifyEmailRequest
            | Flow::UpdateUserAccountDetails
            | Flow::TotpBegin
            | Flow::TotpReset
            | Flow::TotpVerify
            | Flow::TotpUpdate
            | Flow::RecoveryCodeVerify
            | Flow::RecoveryCodesGenerate
            | Flow::TerminateTwoFactorAuth
            | Flow::TwoFactorAuthStatus
            | Flow::CreateUserAuthenticationMethod
            | Flow::UpdateUserAuthenticationMethod
            | Flow::ListUserAuthenticationMethods
            | Flow::UserTransferKey
            | Flow::GetSsoAuthUrl
            | Flow::SignInWithSso
            | Flow::ListOrgForUser
            | Flow::ListMerchantsForUserInOrg
            | Flow::ListProfileForUserInOrgAndMerchant
            | Flow::ListInvitationsForUser
            | Flow::AuthSelect
            | Flow::GetThemeUsingLineage
            | Flow::GetThemeUsingThemeId
            | Flow::UploadFileToThemeStorage
            | Flow::CreateTheme
            | Flow::UpdateTheme
            | Flow::DeleteTheme
            | Flow::CreateUserTheme
            | Flow::UpdateUserTheme
            | Flow::DeleteUserTheme
            | Flow::GetUserThemeUsingThemeId
            | Flow::UploadFileToUserThemeStorage
            | Flow::GetUserThemeUsingLineage
            | Flow::ListAllThemesInLineage
            | Flow::CloneConnector => Self::User,

<<<<<<< HEAD
            Flow::GetDataFromHyperswitchAiFlow | Flow::ListAllChatInteractions => Self::AiWorkflow,
=======
            Flow::GetDataFromHyperswitchAiFlow => Self::AiWorkflow,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)

            Flow::ListRolesV2
            | Flow::ListInvitableRolesAtEntityLevel
            | Flow::ListUpdatableRolesAtEntityLevel
            | Flow::GetRole
            | Flow::GetRoleV2
            | Flow::GetRoleFromToken
            | Flow::GetRoleFromTokenV2
<<<<<<< HEAD
            | Flow::GetParentGroupsInfoForRoleFromToken
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            | Flow::UpdateUserRole
            | Flow::GetAuthorizationInfo
            | Flow::GetRolesInfo
            | Flow::GetParentGroupInfo
            | Flow::AcceptInvitationsV2
            | Flow::AcceptInvitationsPreAuth
            | Flow::DeleteUserRole
            | Flow::CreateRole
            | Flow::CreateRoleV2
            | Flow::UpdateRole
            | Flow::UserFromEmail
            | Flow::ListUsersInLineage => Self::UserRole,
<<<<<<< HEAD
            Flow::GetActionUrl | Flow::SyncOnboardingStatus | Flow::ResetTrackingId => {
                Self::ConnectorOnboarding
            }
=======

            Flow::GetActionUrl | Flow::SyncOnboardingStatus | Flow::ResetTrackingId => {
                Self::ConnectorOnboarding
            }

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::ReconMerchantUpdate
            | Flow::ReconTokenRequest
            | Flow::ReconServiceRequest
            | Flow::ReconVerifyToken => Self::Recon,
<<<<<<< HEAD
            Flow::RetrievePollStatus => Self::Poll,
            Flow::FeatureMatrix => Self::Documentation,
            Flow::TokenizeCard
            | Flow::TokenizeCardUsingPaymentMethodId
            | Flow::TokenizeCardBatch => Self::CardNetworkTokenization,
            Flow::HypersenseTokenRequest
            | Flow::HypersenseVerifyToken
            | Flow::HypersenseSignoutToken => Self::Hypersense,
=======

            Flow::RetrievePollStatus => Self::Poll,

            Flow::FeatureMatrix => Self::Documentation,

            Flow::TokenizeCard
            | Flow::TokenizeCardUsingPaymentMethodId
            | Flow::TokenizeCardBatch => Self::CardNetworkTokenization,

            Flow::HypersenseTokenRequest
            | Flow::HypersenseVerifyToken
            | Flow::HypersenseSignoutToken => Self::Hypersense,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::PaymentMethodSessionCreate
            | Flow::PaymentMethodSessionRetrieve
            | Flow::PaymentMethodSessionConfirm
            | Flow::PaymentMethodSessionUpdateSavedPaymentMethod
            | Flow::PaymentMethodSessionDeleteSavedPaymentMethod
            | Flow::PaymentMethodSessionUpdate => Self::PaymentMethodSession,
<<<<<<< HEAD
            Flow::RevenueRecoveryRetrieve | Flow::RevenueRecoveryResume => Self::ProcessTracker,
=======

            Flow::RevenueRecoveryRetrieve => Self::ProcessTracker,

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::AuthenticationCreate
            | Flow::AuthenticationEligibility
            | Flow::AuthenticationSync
            | Flow::AuthenticationSyncPostUpdate
            | Flow::AuthenticationAuthenticate => Self::Authentication,
            Flow::Proxy => Self::Proxy,
<<<<<<< HEAD
=======

>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            Flow::ProfileAcquirerCreate | Flow::ProfileAcquirerUpdate => Self::ProfileAcquirer,
            Flow::ThreeDsDecisionRuleExecute => Self::ThreeDsDecisionRule,
            Flow::TokenizationCreate | Flow::TokenizationRetrieve | Flow::TokenizationDelete => {
                Self::GenericTokenization
            }
<<<<<<< HEAD

            Flow::RecoveryDataBackfill | Flow::RevenueRecoveryRedis => Self::RecoveryRecovery,
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        }
    }
}
