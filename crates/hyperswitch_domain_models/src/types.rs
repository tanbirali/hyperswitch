pub use diesel_models::types::OrderDetailsWithAmount;

use crate::{
    router_data::{AccessToken, AccessTokenAuthenticationResponse, RouterData},
    router_data_v2::{self, RouterDataV2},
    router_flow_types::{
<<<<<<< HEAD
        mandate_revoke::MandateRevoke,
        revenue_recovery::InvoiceRecordBack,
        subscriptions::{
            GetSubscriptionEstimate, GetSubscriptionPlanPrices, GetSubscriptionPlans,
            SubscriptionCreate,
        },
        AccessTokenAuth, AccessTokenAuthentication, Authenticate, AuthenticationConfirmation,
        Authorize, AuthorizeSessionToken, BillingConnectorInvoiceSync,
        BillingConnectorPaymentsSync, CalculateTax, Capture, CompleteAuthorize,
        CreateConnectorCustomer, CreateOrder, Execute, ExtendAuthorization, ExternalVaultProxy,
        GiftCardBalanceCheck, IncrementalAuthorization, PSync, PaymentMethodToken,
        PostAuthenticate, PostCaptureVoid, PostSessionTokens, PreAuthenticate, PreProcessing,
        RSync, SdkSessionUpdate, Session, SetupMandate, UpdateMetadata, VerifyWebhookSource, Void,
=======
        mandate_revoke::MandateRevoke, revenue_recovery::RecoveryRecordBack, AccessTokenAuth,
        AccessTokenAuthentication, Authenticate, AuthenticationConfirmation, Authorize,
        AuthorizeSessionToken, BillingConnectorInvoiceSync, BillingConnectorPaymentsSync,
        CalculateTax, Capture, CompleteAuthorize, CreateConnectorCustomer, CreateOrder, Execute,
        ExternalVaultProxy, IncrementalAuthorization, PSync, PaymentMethodToken, PostAuthenticate,
        PostCaptureVoid, PostSessionTokens, PreAuthenticate, PreProcessing, RSync,
        SdkSessionUpdate, Session, SetupMandate, UpdateMetadata, VerifyWebhookSource, Void,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    },
    router_request_types::{
        revenue_recovery::{
            BillingConnectorInvoiceSyncRequest, BillingConnectorPaymentsSyncRequest,
<<<<<<< HEAD
            InvoiceRecordBackRequest,
        },
        subscriptions::{
            GetSubscriptionEstimateRequest, GetSubscriptionPlanPricesRequest,
            GetSubscriptionPlansRequest, SubscriptionCreateRequest,
=======
            RevenueRecoveryRecordBackRequest,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        },
        unified_authentication_service::{
            UasAuthenticationRequestData, UasAuthenticationResponseData,
            UasConfirmationRequestData, UasPostAuthenticationRequestData,
            UasPreAuthenticationRequestData,
        },
        AccessTokenAuthenticationRequestData, AccessTokenRequestData, AuthorizeSessionTokenData,
        CompleteAuthorizeData, ConnectorCustomerData, CreateOrderRequestData,
<<<<<<< HEAD
        ExternalVaultProxyPaymentsData, GiftCardBalanceCheckRequestData, MandateRevokeRequestData,
        PaymentMethodTokenizationData, PaymentsAuthenticateData, PaymentsAuthorizeData,
        PaymentsCancelData, PaymentsCancelPostCaptureData, PaymentsCaptureData,
        PaymentsExtendAuthorizationData, PaymentsIncrementalAuthorizationData,
        PaymentsPostAuthenticateData, PaymentsPostSessionTokensData, PaymentsPreAuthenticateData,
=======
        ExternalVaultProxyPaymentsData, MandateRevokeRequestData, PaymentMethodTokenizationData,
        PaymentsAuthorizeData, PaymentsCancelData, PaymentsCancelPostCaptureData,
        PaymentsCaptureData, PaymentsIncrementalAuthorizationData, PaymentsPostSessionTokensData,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        PaymentsPreProcessingData, PaymentsSessionData, PaymentsSyncData,
        PaymentsTaxCalculationData, PaymentsUpdateMetadataData, RefundsData,
        SdkPaymentsSessionUpdateData, SetupMandateRequestData, VaultRequestData,
        VerifyWebhookSourceRequestData,
    },
    router_response_types::{
        revenue_recovery::{
            BillingConnectorInvoiceSyncResponse, BillingConnectorPaymentsSyncResponse,
<<<<<<< HEAD
            InvoiceRecordBackResponse,
        },
        subscriptions::{
            GetSubscriptionEstimateResponse, GetSubscriptionPlanPricesResponse,
            GetSubscriptionPlansResponse, SubscriptionCreateResponse,
        },
        GiftCardBalanceCheckResponseData, MandateRevokeResponseData, PaymentsResponseData,
        RefundsResponseData, TaxCalculationResponseData, VaultResponseData,
        VerifyWebhookSourceResponseData,
=======
            RevenueRecoveryRecordBackResponse,
        },
        MandateRevokeResponseData, PaymentsResponseData, RefundsResponseData,
        TaxCalculationResponseData, VaultResponseData, VerifyWebhookSourceResponseData,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    },
};
#[cfg(feature = "payouts")]
pub use crate::{router_request_types::PayoutsData, router_response_types::PayoutsResponseData};

pub type PaymentsAuthorizeRouterData =
    RouterData<Authorize, PaymentsAuthorizeData, PaymentsResponseData>;
pub type ExternalVaultProxyPaymentsRouterData =
    RouterData<ExternalVaultProxy, ExternalVaultProxyPaymentsData, PaymentsResponseData>;
pub type PaymentsAuthorizeSessionTokenRouterData =
    RouterData<AuthorizeSessionToken, AuthorizeSessionTokenData, PaymentsResponseData>;
pub type PaymentsPreProcessingRouterData =
    RouterData<PreProcessing, PaymentsPreProcessingData, PaymentsResponseData>;
<<<<<<< HEAD
pub type PaymentsPreAuthenticateRouterData =
    RouterData<PreAuthenticate, PaymentsPreAuthenticateData, PaymentsResponseData>;
pub type PaymentsAuthenticateRouterData =
    RouterData<Authenticate, PaymentsAuthenticateData, PaymentsResponseData>;
pub type PaymentsPostAuthenticateRouterData =
    RouterData<PostAuthenticate, PaymentsPostAuthenticateData, PaymentsResponseData>;
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
pub type PaymentsSyncRouterData = RouterData<PSync, PaymentsSyncData, PaymentsResponseData>;
pub type PaymentsCaptureRouterData = RouterData<Capture, PaymentsCaptureData, PaymentsResponseData>;
pub type PaymentsCancelRouterData = RouterData<Void, PaymentsCancelData, PaymentsResponseData>;
pub type PaymentsCancelPostCaptureRouterData =
    RouterData<PostCaptureVoid, PaymentsCancelPostCaptureData, PaymentsResponseData>;
pub type SetupMandateRouterData =
    RouterData<SetupMandate, SetupMandateRequestData, PaymentsResponseData>;
pub type RefundsRouterData<F> = RouterData<F, RefundsData, RefundsResponseData>;
pub type RefundExecuteRouterData = RouterData<Execute, RefundsData, RefundsResponseData>;
pub type RefundSyncRouterData = RouterData<RSync, RefundsData, RefundsResponseData>;
pub type TokenizationRouterData =
    RouterData<PaymentMethodToken, PaymentMethodTokenizationData, PaymentsResponseData>;
pub type ConnectorCustomerRouterData =
    RouterData<CreateConnectorCustomer, ConnectorCustomerData, PaymentsResponseData>;
pub type PaymentsCompleteAuthorizeRouterData =
    RouterData<CompleteAuthorize, CompleteAuthorizeData, PaymentsResponseData>;
pub type PaymentsTaxCalculationRouterData =
    RouterData<CalculateTax, PaymentsTaxCalculationData, TaxCalculationResponseData>;
pub type AccessTokenAuthenticationRouterData = RouterData<
    AccessTokenAuthentication,
    AccessTokenAuthenticationRequestData,
    AccessTokenAuthenticationResponse,
>;
<<<<<<< HEAD
pub type PaymentsGiftCardBalanceCheckRouterData = RouterData<
    GiftCardBalanceCheck,
    GiftCardBalanceCheckRequestData,
    GiftCardBalanceCheckResponseData,
>;
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
pub type RefreshTokenRouterData = RouterData<AccessTokenAuth, AccessTokenRequestData, AccessToken>;
pub type PaymentsPostSessionTokensRouterData =
    RouterData<PostSessionTokens, PaymentsPostSessionTokensData, PaymentsResponseData>;
pub type PaymentsSessionRouterData = RouterData<Session, PaymentsSessionData, PaymentsResponseData>;
pub type PaymentsUpdateMetadataRouterData =
    RouterData<UpdateMetadata, PaymentsUpdateMetadataData, PaymentsResponseData>;

pub type CreateOrderRouterData =
    RouterData<CreateOrder, CreateOrderRequestData, PaymentsResponseData>;
pub type UasPostAuthenticationRouterData =
    RouterData<PostAuthenticate, UasPostAuthenticationRequestData, UasAuthenticationResponseData>;
pub type UasPreAuthenticationRouterData =
    RouterData<PreAuthenticate, UasPreAuthenticationRequestData, UasAuthenticationResponseData>;

pub type UasAuthenticationConfirmationRouterData = RouterData<
    AuthenticationConfirmation,
    UasConfirmationRequestData,
    UasAuthenticationResponseData,
>;

pub type MandateRevokeRouterData =
    RouterData<MandateRevoke, MandateRevokeRequestData, MandateRevokeResponseData>;
pub type PaymentsIncrementalAuthorizationRouterData = RouterData<
    IncrementalAuthorization,
    PaymentsIncrementalAuthorizationData,
    PaymentsResponseData,
>;
<<<<<<< HEAD
pub type PaymentsExtendAuthorizationRouterData =
    RouterData<ExtendAuthorization, PaymentsExtendAuthorizationData, PaymentsResponseData>;
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
pub type SdkSessionUpdateRouterData =
    RouterData<SdkSessionUpdate, SdkPaymentsSessionUpdateData, PaymentsResponseData>;

pub type VerifyWebhookSourceRouterData = RouterData<
    VerifyWebhookSource,
    VerifyWebhookSourceRequestData,
    VerifyWebhookSourceResponseData,
>;

#[cfg(feature = "payouts")]
pub type PayoutsRouterData<F> = RouterData<F, PayoutsData, PayoutsResponseData>;

<<<<<<< HEAD
pub type InvoiceRecordBackRouterData =
    RouterData<InvoiceRecordBack, InvoiceRecordBackRequest, InvoiceRecordBackResponse>;

pub type GetSubscriptionPlansRouterData =
    RouterData<GetSubscriptionPlans, GetSubscriptionPlansRequest, GetSubscriptionPlansResponse>;

pub type GetSubscriptionEstimateRouterData = RouterData<
    GetSubscriptionEstimate,
    GetSubscriptionEstimateRequest,
    GetSubscriptionEstimateResponse,
=======
pub type RevenueRecoveryRecordBackRouterData = RouterData<
    RecoveryRecordBack,
    RevenueRecoveryRecordBackRequest,
    RevenueRecoveryRecordBackResponse,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
>;

pub type UasAuthenticationRouterData =
    RouterData<Authenticate, UasAuthenticationRequestData, UasAuthenticationResponseData>;

pub type BillingConnectorPaymentsSyncRouterData = RouterData<
    BillingConnectorPaymentsSync,
    BillingConnectorPaymentsSyncRequest,
    BillingConnectorPaymentsSyncResponse,
>;

pub type BillingConnectorInvoiceSyncRouterData = RouterData<
    BillingConnectorInvoiceSync,
    BillingConnectorInvoiceSyncRequest,
    BillingConnectorInvoiceSyncResponse,
>;

pub type BillingConnectorInvoiceSyncRouterDataV2 = RouterDataV2<
    BillingConnectorInvoiceSync,
    router_data_v2::flow_common_types::BillingConnectorInvoiceSyncFlowData,
    BillingConnectorInvoiceSyncRequest,
    BillingConnectorInvoiceSyncResponse,
>;

pub type BillingConnectorPaymentsSyncRouterDataV2 = RouterDataV2<
    BillingConnectorPaymentsSync,
    router_data_v2::flow_common_types::BillingConnectorPaymentsSyncFlowData,
    BillingConnectorPaymentsSyncRequest,
    BillingConnectorPaymentsSyncResponse,
>;

<<<<<<< HEAD
pub type InvoiceRecordBackRouterDataV2 = RouterDataV2<
    InvoiceRecordBack,
    router_data_v2::flow_common_types::InvoiceRecordBackData,
    InvoiceRecordBackRequest,
    InvoiceRecordBackResponse,
>;

pub type GetSubscriptionPlanPricesRouterData = RouterData<
    GetSubscriptionPlanPrices,
    GetSubscriptionPlanPricesRequest,
    GetSubscriptionPlanPricesResponse,
=======
pub type RevenueRecoveryRecordBackRouterDataV2 = RouterDataV2<
    RecoveryRecordBack,
    router_data_v2::flow_common_types::RevenueRecoveryRecordBackData,
    RevenueRecoveryRecordBackRequest,
    RevenueRecoveryRecordBackResponse,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
>;

pub type VaultRouterData<F> = RouterData<F, VaultRequestData, VaultResponseData>;

pub type VaultRouterDataV2<F> = RouterDataV2<
    F,
    router_data_v2::flow_common_types::VaultConnectorFlowData,
    VaultRequestData,
    VaultResponseData,
>;

pub type ExternalVaultProxyPaymentsRouterDataV2 = RouterDataV2<
    ExternalVaultProxy,
    router_data_v2::flow_common_types::ExternalVaultProxyFlowData,
    ExternalVaultProxyPaymentsData,
    PaymentsResponseData,
>;
<<<<<<< HEAD

pub type SubscriptionCreateRouterData =
    RouterData<SubscriptionCreate, SubscriptionCreateRequest, SubscriptionCreateResponse>;
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
